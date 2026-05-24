import sqlgrok
from sqlgrok import sqlglot_bridge


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


def test_bridge_manual_compare_rust_error():
    case = sqlglot_bridge.compare_one(
        "SELECT 1",
        read="not-a-dialect",
        write="sqlite",
        expected="SELECT 1",
    )

    assert case.status == "rust-error"
    assert "unknown dialect" in (case.error or "")
