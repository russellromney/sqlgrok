import sqlgrok
from sqlgrok import sqlglot_bridge
from pathlib import Path
import sys
import types


def test_transpile_returns_sqlglot_shaped_list():
    assert sqlgrok.transpile("SELECT 1", read="postgres", write="sqlite") == ["SELECT 1"]


def test_transpile_supports_pretty_output():
    assert sqlgrok.transpile(
        "SELECT a, b FROM t WHERE a > 1",
        read="sqlite",
        write="sqlite",
        pretty=True,
    ) == ["SELECT\n  a,\n  b\nFROM\n  t\nWHERE\n  a > 1"]


def test_bridge_manual_compare_match():
    case = sqlglot_bridge.compare_one(
        "SELECT 1",
        read="postgres",
        write="sqlite",
        expected="SELECT 1",
    )

    assert case.status == "match"
    assert case.actual == "SELECT 1"


def test_bridge_manual_compare_pretty_match():
    case = sqlglot_bridge.compare_one(
        "SELECT a, b FROM t",
        read="sqlite",
        write="sqlite",
        expected="SELECT\n  a,\n  b\nFROM\n  t",
        pretty=True,
    )

    assert case.status == "match"


def test_bridge_manual_compare_rust_error():
    case = sqlglot_bridge.compare_one(
        "SELECT 1",
        read="not-a-dialect",
        write="sqlite",
        expected="SELECT 1",
    )

    assert case.status == "rust-error"
    assert "unknown dialect" in (case.error or "")


def test_bridge_summary_reports_filtered_counts(tmp_path: Path):
    report = tmp_path / "bridge.jsonl"
    recorder = sqlglot_bridge.BridgeRecorder(
        report=report,
        family="transpile",
        read_filter="mysql",
        write_filter="sqlite",
    )

    recorder.compare_transpile(
        helper="manual",
        sql="SELECT 1",
        read="postgres",
        write="sqlite",
        expected="SELECT 1",
    )
    counts = recorder.write_report()

    assert counts == {}
    assert report.read_text() == ""
    summary = report.with_suffix(".md").read_text()
    assert "Observed helper attempts: `1`" in summary
    assert "Filtered by read/write: `1`" in summary
    assert "| `manual` | `postgres` | `sqlite` | 1 |" in summary


def test_bridge_forced_pair_uses_python_sqlglot_oracle(monkeypatch):
    bridge = sys.modules["sqlgrok.sqlglot_bridge"]
    fake_sqlglot = types.SimpleNamespace(
        transpile=lambda sql, read, write, pretty=False: [
            f"{sql} /* {read}->{write} pretty={pretty} */"
        ]
    )
    monkeypatch.setitem(sys.modules, "sqlglot", fake_sqlglot)
    monkeypatch.setattr(
        bridge.sqlgrok,
        "transpile",
        lambda sql, read, write, pretty=False: [
            f"{sql} /* {read}->{write} pretty={pretty} */"
        ],
    )

    case = bridge.compare_one(
        "SELECT a",
        read="mysql",
        write="duckdb",
        expected="ignored helper target",
        pretty=True,
        force_pair=True,
        read_filter="postgres",
        write_filter="sqlite",
    )

    assert case.status == "match"
    assert case.read == "postgres"
    assert case.write == "sqlite"
    assert case.expected == "SELECT a /* postgres->sqlite pretty=True */"


def test_bridge_forced_pair_records_oracle_error(monkeypatch):
    bridge = sys.modules["sqlgrok.sqlglot_bridge"]

    def raise_oracle(*args, **kwargs):
        raise ValueError("oracle rejected input")

    fake_sqlglot = types.SimpleNamespace(transpile=raise_oracle)
    monkeypatch.setitem(sys.modules, "sqlglot", fake_sqlglot)

    case = bridge.compare_one(
        "SELECT ?",
        read="mysql",
        write="duckdb",
        expected="ignored helper target",
        force_pair=True,
        read_filter="postgres",
        write_filter="sqlite",
    )

    assert case.status == "oracle-error"
    assert case.read == "postgres"
    assert case.write == "sqlite"
    assert "oracle rejected input" in (case.error or "")


def test_bridge_forced_pair_summary_does_not_filter_routes(tmp_path: Path, monkeypatch):
    bridge = sys.modules["sqlgrok.sqlglot_bridge"]
    fake_sqlglot = types.SimpleNamespace(
        transpile=lambda sql, read, write, pretty=False: ["SELECT 1"]
    )
    monkeypatch.setitem(sys.modules, "sqlglot", fake_sqlglot)

    report = tmp_path / "bridge.jsonl"
    recorder = bridge.BridgeRecorder(
        report=report,
        family="transpile",
        read_filter="mysql",
        write_filter="sqlite",
        force_pair=True,
    )

    recorder.compare_transpile(
        helper="manual",
        sql="SELECT 1",
        read="postgres",
        write="duckdb",
        expected="ignored helper target",
    )
    counts = recorder.write_report()

    assert counts == {"match": 1}
    summary = report.with_suffix(".md").read_text()
    assert "Mode: `forced-pair`" in summary
    assert "Requested pair: `mysql` -> `sqlite`" in summary
    assert "Filtered by read/write: `0`" in summary


def test_bridge_main_ignores_pytest_failure_when_report_exists(tmp_path: Path, monkeypatch):
    bridge = sys.modules["sqlgrok.sqlglot_bridge"]
    report = tmp_path / "bridge.jsonl"

    def fake_run_pytest(args):
        Path(args.report_output).write_text("", encoding="utf-8")
        return 1

    monkeypatch.setattr(bridge, "run_pytest", fake_run_pytest)

    assert (
        bridge.main(
            [
                "--sqlglot",
                ".",
                "--family",
                "transpile",
                "--read",
                "postgres",
                "--write",
                "sqlite",
                "--report-output",
                str(report),
            ]
        )
        == 0
    )


def test_bridge_main_preserves_pytest_failure_without_report(tmp_path: Path, monkeypatch):
    bridge = sys.modules["sqlgrok.sqlglot_bridge"]
    report = tmp_path / "missing.jsonl"
    monkeypatch.setattr(bridge, "run_pytest", lambda args: 1)

    assert (
        bridge.main(
            [
                "--sqlglot",
                ".",
                "--family",
                "transpile",
                "--read",
                "postgres",
                "--write",
                "sqlite",
                "--report-output",
                str(report),
            ]
        )
        == 1
    )


def test_bridge_main_strict_pytest_preserves_failure(tmp_path: Path, monkeypatch):
    bridge = sys.modules["sqlgrok.sqlglot_bridge"]
    report = tmp_path / "bridge.jsonl"

    def fake_run_pytest(args):
        Path(args.report_output).write_text("", encoding="utf-8")
        return 1

    monkeypatch.setattr(bridge, "run_pytest", fake_run_pytest)

    assert (
        bridge.main(
            [
                "--sqlglot",
                ".",
                "--family",
                "transpile",
                "--read",
                "postgres",
                "--write",
                "sqlite",
                "--report-output",
                str(report),
                "--strict-pytest",
            ]
        )
        == 1
    )
