# /// script
# requires-python = ">=3.10"
# dependencies = []
# ///
"""Mine Python SQLGlot for Rust-ready dialect inventories.

This is a codegen spike. It imports a local Python SQLGlot checkout and uses
plain introspection (no regex over Python source) to dump SQLGlot's dialect
knowledge into deterministic JSON, plus one proof-of-concept Rust file.

The point: replace "discover one parity gap at a time" with generated
inventories we can diff against the Rust implementation. See README.md for the
boundary against py2many-style source translation.

Run:

    uv run --python 3.10 tools/sqlglot_codegen/extract.py \
        --sqlglot /Users/russellromney/Documents/Github/sqlglot \
        --out generated/sqlglot_inventory \
        --dialects postgres,mysql,sqlite
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any


# ---------------------------------------------------------------------------
# Import wiring
# ---------------------------------------------------------------------------


def load_sqlglot(sqlglot_path: Path):
    """Import SQLGlot from a local checkout via sys.path.

    SQLGlot core is dependency-free, so a plain path insert is enough. We do not
    install or build anything, which keeps the SQLGlot checkout pristine.
    """
    if not (sqlglot_path / "sqlglot" / "__init__.py").exists():
        raise SystemExit(f"no sqlglot package found under {sqlglot_path}")
    sys.path.insert(0, str(sqlglot_path))
    import sqlglot  # noqa: E402

    return sqlglot


# ---------------------------------------------------------------------------
# Helpers for stable, JSON-friendly values
# ---------------------------------------------------------------------------


def enum_name(value: Any) -> str:
    """Render an enum member as its stable name (e.g. TokenType.SELECT)."""
    return getattr(value, "name", str(value))


def sorted_str_dict(d: dict) -> dict:
    return {k: d[k] for k in sorted(d)}


# ---------------------------------------------------------------------------
# Per-inventory extractors
# ---------------------------------------------------------------------------


def extract_dialect_names(sqlglot) -> list[str]:
    from sqlglot.dialects.dialect import Dialects

    names = [d.value for d in Dialects if d.value]
    return sorted(names)


def extract_keywords(tokenizer_cls) -> dict[str, str]:
    """tokenizer KEYWORDS: raw text -> TokenType name.

    We read the *class* attribute, not an instance. An instantiated tokenizer
    compiles KEYWORDS into a trie and exposes an empty dict, so the class is the
    stable source.
    """
    out: dict[str, str] = {}
    for kw, token_type in getattr(tokenizer_cls, "KEYWORDS", {}).items():
        out[kw] = enum_name(token_type)
    return sorted_str_dict(out)


def extract_type_mapping(generator_cls) -> dict[str, str]:
    """generator TYPE_MAPPING: canonical DataType.Type name -> rendered SQL."""
    out: dict[str, str] = {}
    for dtype, rendered in getattr(generator_cls, "TYPE_MAPPING", {}).items():
        out[enum_name(dtype)] = rendered
    return sorted_str_dict(out)


def classify_transform(fn: Any) -> dict[str, Any]:
    """Classify a generator TRANSFORMS value.

    TRANSFORMS values are Python callables. Most are dynamic and cannot be
    turned into static Rust data. We still classify them so the inventory makes
    the dynamic/portable boundary explicit:

    - ``rename``: produced by ``rename_func("NAME")``; the only freevar is
      ``name`` bound to a string. This is portable as static data.
    - ``lambda``: an anonymous function. Behavior is code, not data.
    - ``named``: a named helper (e.g. ``inline_array_sql``). Semi-portable: the
      name tells you which behavior to port by hand.
    """
    code = getattr(fn, "__code__", None)
    name = getattr(fn, "__name__", None)
    freevars = tuple(getattr(code, "co_freevars", ()) or ())
    closure = getattr(fn, "__closure__", None) or ()
    cells = {}
    for var, cell in zip(freevars, closure):
        try:
            cells[var] = cell.cell_contents
        except ValueError:
            cells[var] = None

    # rename_func("NAME") -> lambda self, expression: self.func(name, ...)
    if (
        name == "<lambda>"
        and set(freevars) == {"name"}
        and isinstance(cells.get("name"), str)
    ):
        return {"kind": "rename", "target": cells["name"]}

    if name == "<lambda>":
        info: dict[str, Any] = {"kind": "lambda"}
        if freevars:
            info["freevars"] = sorted(freevars)
        return info

    return {
        "kind": "named",
        "helper": name,
        "module": getattr(fn, "__module__", None),
    }


def extract_transforms(generator_cls, base_generator_cls) -> list[dict[str, Any]]:
    """generator TRANSFORMS classified by expression class name.

    Each entry records whether it overrides the base Generator (dialect-specific)
    so a consumer can focus on real dialect divergence instead of inherited
    defaults.
    """
    base = getattr(base_generator_cls, "TRANSFORMS", {})
    out: list[dict[str, Any]] = []
    for cls, fn in getattr(generator_cls, "TRANSFORMS", {}).items():
        entry: dict[str, Any] = {
            "expression": cls.__name__,
            "key": getattr(cls, "key", cls.__name__.lower()),
            "dialect_specific": base.get(cls) is not fn,
        }
        entry.update(classify_transform(fn))
        out.append(entry)
    out.sort(key=lambda e: (e["expression"], e["kind"]))
    return out


def extract_functions(parser_cls) -> list[str]:
    """parser FUNCTIONS keys: SQL function names this dialect parses specially."""
    return sorted(getattr(parser_cls, "FUNCTIONS", {}).keys())


def extract_time_mapping(dialect_cls) -> dict[str, str]:
    """dialect TIME_MAPPING: source format token -> Python/strftime token."""
    return sorted_str_dict(dict(getattr(dialect_cls, "TIME_MAPPING", {})))


def extract_expressions(sqlglot) -> list[dict[str, Any]]:
    """Expression class inventory with arg_types.

    arg_types is SQLGlot's declared shape of each node: arg name -> required.
    This is the closest thing SQLGlot has to an AST schema, and it is the
    natural driver for sqlgrok AST work.
    """
    from sqlglot import expressions as exp

    out: list[dict[str, Any]] = []
    for name in sorted(dir(exp)):
        obj = getattr(exp, name)
        if not isinstance(obj, type) or not issubclass(obj, exp.Expression):
            continue
        if obj is exp.Expression:
            continue
        # arg_types is defined on most concrete nodes; skip abstract-ish bases
        # that do not declare their own.
        arg_types = obj.__dict__.get("arg_types")
        if arg_types is None:
            continue
        out.append(
            {
                "class": name,
                "key": getattr(obj, "key", name.lower()),
                "arg_types": {k: bool(v) for k, v in arg_types.items()},
            }
        )
    return out


# ---------------------------------------------------------------------------
# Rust emitter (proof of concept)
# ---------------------------------------------------------------------------


def emit_rust_renames(
    dialect: str,
    renames: dict[str, str],
    version: str,
    commit: str,
    cmd: str,
) -> str:
    const = f"{dialect.upper()}_FUNCTION_RENAMES"
    lines = [
        f"// @generated by tools/sqlglot_codegen/extract.py",
        f"// Source: Python SQLGlot {version} ({commit}),",
        f'//   Dialect.get_or_raise("{dialect}").generator_class.TRANSFORMS rename_func entries.',
        "//",
        "// Each entry maps a canonical SQLGlot expression class to the function name",
        f"// it renders as for the {dialect} dialect. This is static rename data only;",
        "// non-rename transforms are dynamic code and are intentionally excluded.",
        "//",
        "// Do not edit by hand. Regenerate with:",
        f"//   {cmd}",
        "",
        "/// (expression_class, function_name) pairs, sorted by expression class.",
        f"pub static {const}: &[(&str, &str)] = &[",
    ]
    for cls in sorted(renames):
        lines.append(f'    ({_rust_str(cls)}, {_rust_str(renames[cls])}),')
    lines.append("];")
    lines.append("")
    return "\n".join(lines)


def _rust_str(s: str) -> str:
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


# ---------------------------------------------------------------------------
# Driver
# ---------------------------------------------------------------------------


def write_json(path: Path, data: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")


def main() -> int:
    ap = argparse.ArgumentParser(description="Extract SQLGlot inventories for sqlgrok codegen.")
    ap.add_argument("--sqlglot", required=True, type=Path, help="Path to a local Python SQLGlot checkout.")
    ap.add_argument("--out", required=True, type=Path, help="Output directory for the inventory.")
    ap.add_argument(
        "--dialects",
        default=None,
        help="Comma-separated dialect subset (default: all SQLGlot dialects).",
    )
    ap.add_argument(
        "--rust-dialect",
        default="sqlite",
        help="Dialect to emit the Rust rename PoC for (default: sqlite).",
    )
    ap.add_argument(
        "--include-expressions",
        action="store_true",
        help="Also emit expressions.json, the large SQLGlot expression-class inventory.",
    )
    args = ap.parse_args()

    sqlglot = load_sqlglot(args.sqlglot.resolve())
    from sqlglot.dialects.dialect import Dialect
    from sqlglot.generator import Generator as BaseGenerator

    version = getattr(sqlglot, "__version__", "unknown")
    commit = getattr(sqlglot, "__commit_id__", None)
    if not commit:
        try:
            from sqlglot import _version

            commit = getattr(_version, "__commit_id__", None)
        except Exception:
            commit = None
    commit = commit or "unknown"

    all_names = extract_dialect_names(sqlglot)
    if args.dialects:
        wanted = [d.strip() for d in args.dialects.split(",") if d.strip()]
        unknown = [d for d in wanted if d not in all_names]
        if unknown:
            raise SystemExit(f"unknown dialects: {unknown}")
        names = wanted
    else:
        names = all_names

    out = args.out
    write_json(out / "meta.json", {"sqlglot_version": version, "sqlglot_commit": commit})
    write_json(out / "dialects.json", all_names)
    if args.include_expressions:
        write_json(out / "expressions.json", extract_expressions(sqlglot))

    summary: dict[str, dict[str, int]] = {}
    rust_renames: dict[str, str] = {}

    for name in names:
        d = Dialect.get_or_raise(name)
        dialect_cls = type(d)
        gen_cls = d.generator_class
        tok_cls = d.tokenizer_class
        par_cls = d.parser_class

        keywords = extract_keywords(tok_cls)
        type_mapping = extract_type_mapping(gen_cls)
        transforms = extract_transforms(gen_cls, BaseGenerator)
        functions = extract_functions(par_cls)
        time_mapping = extract_time_mapping(dialect_cls)

        write_json(
            out / "dialects" / f"{name}.json",
            {
                "dialect": name,
                "tokenizer_keywords": keywords,
                "generator_type_mapping": type_mapping,
                "generator_transforms": transforms,
                "parser_functions": functions,
                "time_mapping": time_mapping,
            },
        )

        renames = {t["expression"]: t["target"] for t in transforms if t["kind"] == "rename"}
        summary[name] = {
            "tokenizer_keywords": len(keywords),
            "generator_type_mapping": len(type_mapping),
            "generator_transforms": len(transforms),
            "generator_transforms_rename": len(renames),
            "parser_functions": len(functions),
            "time_mapping": len(time_mapping),
        }
        if name == args.rust_dialect:
            rust_renames = renames

    write_json(out / "summary.json", summary)

    cmd = (
        "uv run --python 3.10 tools/sqlglot_codegen/extract.py "
        "--sqlglot /path/to/sqlglot --out generated/sqlglot_inventory "
        "--dialects postgres,mysql,sqlite"
    )
    rust_path = out / f"{args.rust_dialect}_function_renames.rs"
    rust_path.write_text(
        emit_rust_renames(args.rust_dialect, rust_renames, version, commit, cmd)
    )

    print(f"SQLGlot {version} ({commit})")
    print(f"wrote inventory for {len(names)} dialect(s) to {out}")
    for name in ("postgres", "mysql", "sqlite"):
        if name in summary:
            s = summary[name]
            print(
                f"  {name:9s} keywords={s['tokenizer_keywords']:4d} "
                f"types={s['generator_type_mapping']:3d} "
                f"transforms={s['generator_transforms']:3d} "
                f"(rename={s['generator_transforms_rename']:2d}) "
                f"functions={s['parser_functions']:4d} "
                f"time={s['time_mapping']:3d}"
            )
    print(f"  rust PoC: {rust_path} ({len(rust_renames)} {args.rust_dialect} renames)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
