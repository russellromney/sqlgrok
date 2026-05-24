"""Pytest bridge for running SQLGlot helper cases against sqlgrok.

The bridge records parity outcomes without failing SQLGlot's own test run. SQLGlot
keeps testing itself; this module observes helper calls and compares their expected
SQL with the Rust implementation exposed by ``sqlgrok.transpile``.
"""

from __future__ import annotations

import argparse
import inspect
import json
import os
import sys
from collections import Counter
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any, Optional

import sqlgrok


STATUSES = {
    "match",
    "mismatch",
    "rust-error",
    "oracle-error",
    "unsupported-harness-shape",
}


@dataclass
class BridgeCase:
    status: str
    family: str
    source_file: str
    source_line: int
    test_name: str
    helper: str
    sql: str
    read: Optional[str]
    write: Optional[str]
    expected: Optional[str]
    actual: Optional[str]
    error: Optional[str]


class BridgeRecorder:
    def __init__(
        self,
        *,
        report: Path,
        family: str,
        read_filter: Optional[str],
        write_filter: Optional[str],
        sqlglot_root: Optional[Path] = None,
    ) -> None:
        self.report = report
        self.family = family
        self.read_filter = read_filter
        self.write_filter = write_filter
        self.sqlglot_root = sqlglot_root.resolve() if sqlglot_root else None
        self.cases: list[BridgeCase] = []

    def wants(self, read: Optional[str], write: Optional[str]) -> bool:
        if self.read_filter and read != self.read_filter:
            return False
        if self.write_filter and write != self.write_filter:
            return False
        return True

    def record(
        self,
        *,
        helper: str,
        sql: str,
        read: Optional[str],
        write: Optional[str],
        expected: Optional[str],
        actual: Optional[str] = None,
        error: Optional[str] = None,
        status: Optional[str] = None,
    ) -> None:
        if not self.wants(read, write):
            return

        if status is None:
            status = classify(expected=expected, actual=actual, error=error)
        if status not in STATUSES:
            raise ValueError(f"unknown bridge status: {status}")

        frame = _first_test_frame(self.sqlglot_root)
        self.cases.append(
            BridgeCase(
                status=status,
                family=self.family,
                source_file=frame.filename,
                source_line=frame.lineno,
                test_name=frame.function,
                helper=helper,
                sql=sql,
                read=read,
                write=write,
                expected=expected,
                actual=actual,
                error=error,
            )
        )

    def compare_transpile(
        self,
        *,
        helper: str,
        sql: str,
        read: Optional[str],
        write: Optional[str],
        expected: str,
    ) -> None:
        if not self.wants(read, write):
            return

        try:
            outputs = sqlgrok.transpile(sql, read=read, write=write)
            actual = outputs[0] if outputs else None
            error = None
        except Exception as exc:  # noqa: BLE001 - bridge reports the exact Rust/Python failure
            actual = None
            error = f"{type(exc).__name__}: {exc}"

        self.record(
            helper=helper,
            sql=sql,
            read=read,
            write=write,
            expected=expected,
            actual=actual,
            error=error,
        )

    def unsupported(
        self,
        *,
        helper: str,
        sql: str,
        read: Optional[str],
        write: Optional[str],
        expected: Optional[str],
        reason: str,
    ) -> None:
        self.record(
            helper=helper,
            sql=sql,
            read=read,
            write=write,
            expected=expected,
            error=reason,
            status="unsupported-harness-shape",
        )

    def write_report(self) -> Counter[str]:
        self.report.parent.mkdir(parents=True, exist_ok=True)
        with self.report.open("w", encoding="utf-8") as handle:
            for case in self.cases:
                handle.write(json.dumps(asdict(case), sort_keys=True))
                handle.write("\n")
        return Counter(case.status for case in self.cases)


@dataclass(frozen=True)
class FrameInfo:
    filename: str
    lineno: int
    function: str


def classify(*, expected: Optional[str], actual: Optional[str], error: Optional[str]) -> str:
    if error:
        return "rust-error"
    if expected is None:
        return "oracle-error"
    if actual == expected:
        return "match"
    return "mismatch"


def compare_one(sql: str, *, read: Optional[str], write: Optional[str], expected: str) -> BridgeCase:
    recorder = BridgeRecorder(
        report=Path(os.devnull),
        family="transpile",
        read_filter=None,
        write_filter=None,
        sqlglot_root=None,
    )
    recorder.compare_transpile(
        helper="manual",
        sql=sql,
        read=read,
        write=write,
        expected=expected,
    )
    return recorder.cases[0]


def load_report(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    if not path.exists():
        return rows
    with path.open(encoding="utf-8") as handle:
        for line in handle:
            if line.strip():
                rows.append(json.loads(line))
    return rows


def summarize_report(path: Path) -> Counter[str]:
    return Counter(row["status"] for row in load_report(path))


def check_budget(report: Path, budget: Path) -> None:
    actual = summarize_report(report)
    with budget.open(encoding="utf-8") as handle:
        expected = json.load(handle)

    regressions: list[str] = []
    for status in ("mismatch", "rust-error", "oracle-error", "unsupported-harness-shape"):
        budget_count = int(expected.get(status, 0))
        actual_count = actual.get(status, 0)
        if actual_count > budget_count:
            regressions.append(f"{status}: actual {actual_count} > budget {budget_count}")

    if regressions:
        raise SystemExit("bridge budget regression:\n" + "\n".join(regressions))


def pytest_addoption(parser: Any) -> None:
    group = parser.getgroup("sqlgrok")
    group.addoption("--sqlgrok-bridge-report", action="store", default=None)
    group.addoption("--sqlgrok-bridge-family", action="store", default="transpile")
    group.addoption("--sqlgrok-bridge-read", action="store", default=None)
    group.addoption("--sqlgrok-bridge-write", action="store", default=None)
    group.addoption("--sqlgrok-bridge-sqlglot-root", action="store", default=None)


def pytest_configure(config: Any) -> None:
    report = config.getoption("--sqlgrok-bridge-report")
    if not report:
        return

    recorder = BridgeRecorder(
        report=Path(report),
        family=config.getoption("--sqlgrok-bridge-family"),
        read_filter=config.getoption("--sqlgrok-bridge-read"),
        write_filter=config.getoption("--sqlgrok-bridge-write"),
        sqlglot_root=Path(config.getoption("--sqlgrok-bridge-sqlglot-root"))
        if config.getoption("--sqlgrok-bridge-sqlglot-root")
        else None,
    )
    config._sqlgrok_bridge_recorder = recorder


def pytest_collection_modifyitems(config: Any, items: list[Any]) -> None:
    recorder = getattr(config, "_sqlgrok_bridge_recorder", None)
    if recorder is not None:
        patch_sqlglot_helpers(recorder)


def pytest_sessionfinish(session: Any, exitstatus: int) -> None:  # noqa: ARG001
    recorder = getattr(session.config, "_sqlgrok_bridge_recorder", None)
    if recorder is None:
        return
    counts = recorder.write_report()
    summary = ", ".join(f"{status}={count}" for status, count in sorted(counts.items()))
    print(f"\nsqlgrok bridge wrote {recorder.report} ({summary or 'no cases'})")


def patch_sqlglot_helpers(recorder: BridgeRecorder) -> None:
    try:
        from sqlglot.errors import UnsupportedError
        from tests.dialects import test_dialect
        from tests import test_transpile
    except Exception as exc:  # noqa: BLE001
        recorder.unsupported(
            helper="patch",
            sql="",
            read=None,
            write=None,
            expected=None,
            reason=f"failed to import SQLGlot test helpers: {type(exc).__name__}: {exc}",
        )
        return

    if not getattr(test_transpile.TestTranspile.validate, "_sqlgrok_patched", False):
        original_validate = test_transpile.TestTranspile.validate

        def validate(self: Any, sql: str, target: str, **kwargs: Any) -> Any:
            read = kwargs.get("read")
            write = kwargs.get("write")
            extra_kwargs = sorted(set(kwargs) - {"read", "write"})
            if extra_kwargs:
                recorder.unsupported(
                    helper="validate",
                    sql=sql,
                    read=read,
                    write=write,
                    expected=target,
                    reason=f"unsupported validate kwargs: {', '.join(extra_kwargs)}",
                )
            else:
                recorder.compare_transpile(
                    helper="validate",
                    sql=sql,
                    read=read,
                    write=write,
                    expected=target,
                )
            return original_validate(self, sql, target, **kwargs)

        validate._sqlgrok_patched = True
        test_transpile.TestTranspile.validate = validate

    if not getattr(test_dialect.Validator.validate_all, "_sqlgrok_patched", False):
        original_validate_all = test_dialect.Validator.validate_all

        def validate_all(
            self: Any,
            sql: str,
            read: Optional[dict[str, str]] = None,
            write: Optional[dict[str, str]] = None,
            pretty: bool = False,
            identify: bool = False,
        ) -> Any:
            base_dialect = _dialect_name(getattr(self, "dialect", None))
            if pretty or identify:
                for read_dialect, read_sql in (read or {}).items():
                    recorder.unsupported(
                        helper="validate_all",
                        sql=read_sql,
                        read=_dialect_name(read_dialect),
                        write=base_dialect,
                        expected=sql,
                        reason="pretty/identify helper options are not supported yet",
                    )
                for write_dialect, write_sql in (write or {}).items():
                    recorder.unsupported(
                        helper="validate_all",
                        sql=sql,
                        read=base_dialect,
                        write=_dialect_name(write_dialect),
                        expected=None if write_sql is UnsupportedError else write_sql,
                        reason="pretty/identify helper options are not supported yet",
                    )
            else:
                for read_dialect, read_sql in (read or {}).items():
                    recorder.compare_transpile(
                        helper="validate_all",
                        sql=read_sql,
                        read=_dialect_name(read_dialect),
                        write=base_dialect,
                        expected=sql,
                    )
                for write_dialect, write_sql in (write or {}).items():
                    if write_sql is UnsupportedError:
                        recorder.unsupported(
                            helper="validate_all",
                            sql=sql,
                            read=base_dialect,
                            write=_dialect_name(write_dialect),
                            expected=None,
                            reason="SQLGlot expects UnsupportedError",
                        )
                    else:
                        recorder.compare_transpile(
                            helper="validate_all",
                            sql=sql,
                            read=base_dialect,
                            write=_dialect_name(write_dialect),
                            expected=write_sql,
                        )
            return original_validate_all(self, sql, read, write, pretty, identify)

        validate_all._sqlgrok_patched = True
        test_dialect.Validator.validate_all = validate_all

    if not getattr(test_dialect.Validator.validate_identity, "_sqlgrok_patched", False):
        original_validate_identity = test_dialect.Validator.validate_identity

        def validate_identity(
            self: Any,
            sql: str,
            write_sql: Optional[str] = None,
            pretty: bool = False,
            check_command_warning: bool = False,
            identify: bool = False,
        ) -> Any:
            dialect = _dialect_name(getattr(self, "dialect", None))
            expected = write_sql or sql
            if pretty or identify or check_command_warning:
                recorder.unsupported(
                    helper="validate_identity",
                    sql=sql,
                    read=dialect,
                    write=dialect,
                    expected=expected,
                    reason="pretty/identify/check_command_warning helper options are not supported yet",
                )
            else:
                recorder.compare_transpile(
                    helper="validate_identity",
                    sql=sql,
                    read=dialect,
                    write=dialect,
                    expected=expected,
                )
            return original_validate_identity(
                self,
                sql,
                write_sql,
                pretty,
                check_command_warning,
                identify,
            )

        validate_identity._sqlgrok_patched = True
        test_dialect.Validator.validate_identity = validate_identity


def _dialect_name(value: Any) -> Optional[str]:
    if value is None:
        return None
    value = getattr(value, "value", value)
    if isinstance(value, str):
        return value.lower()
    return str(value).lower()


def _first_test_frame(sqlglot_root: Optional[Path]) -> FrameInfo:
    for frame in inspect.stack()[2:]:
        path = frame.filename
        if "/tests/" in path and not path.endswith("sqlglot_bridge.py"):
            return FrameInfo(
                filename=_display_path(path, sqlglot_root),
                lineno=frame.lineno,
                function=frame.function,
            )
    caller = inspect.stack()[2]
    return FrameInfo(
        filename=_display_path(caller.filename, sqlglot_root),
        lineno=caller.lineno,
        function=caller.function,
    )


def _display_path(path: str, sqlglot_root: Optional[Path]) -> str:
    if sqlglot_root is None:
        return path
    try:
        return str(Path(path).resolve().relative_to(sqlglot_root))
    except ValueError:
        return path


def run_pytest(args: argparse.Namespace) -> int:
    import pytest

    sqlglot = Path(args.sqlglot).resolve()
    sys.path.insert(0, str(sqlglot))
    module = args.module or "tests/test_transpile.py"
    report = Path(args.report_output)

    pytest_args = [
        str(sqlglot / module),
        "-p",
        "sqlgrok.sqlglot_bridge",
        "--sqlgrok-bridge-report",
        str(report),
        "--sqlgrok-bridge-family",
        args.family,
        "--sqlgrok-bridge-sqlglot-root",
        str(sqlglot),
    ]
    if args.read:
        pytest_args.extend(["--sqlgrok-bridge-read", args.read])
    if args.write:
        pytest_args.extend(["--sqlgrok-bridge-write", args.write])
    if args.pytest_arg:
        pytest_args.extend(args.pytest_arg)

    return pytest.main(pytest_args)


def main(argv: Optional[list[str]] = None) -> int:
    parser = argparse.ArgumentParser(description="Run SQLGlot pytest helper parity via sqlgrok")
    parser.add_argument("--sqlglot", required=True)
    parser.add_argument("--family", default="transpile")
    parser.add_argument("--read")
    parser.add_argument("--write")
    parser.add_argument("--module")
    parser.add_argument("--report-output", required=True)
    parser.add_argument("--budget")
    parser.add_argument("--check-budget", action="store_true")
    args, unknown = parser.parse_known_args(argv)
    args.pytest_arg = unknown

    if args.family != "transpile":
        raise SystemExit(f"unsupported family {args.family!r}; only 'transpile' is implemented")

    exit_code = run_pytest(args)
    if args.check_budget:
        if not args.budget:
            raise SystemExit("--check-budget requires --budget")
        check_budget(Path(args.report_output), Path(args.budget))
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
