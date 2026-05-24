import sqlgrok


def test_transpile_returns_sqlglot_shaped_list():
    assert sqlgrok.transpile("SELECT 1", read="postgres", write="sqlite") == ["SELECT 1"]
