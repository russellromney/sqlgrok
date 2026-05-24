import sqlgrok
from sqlgrok import sqlglot_bridge
from pathlib import Path


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
