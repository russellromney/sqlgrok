use sqlgrok::{Dialect, scalar_select_projection};

#[test]
fn scalar_select_projection_returns_expression_and_alias() {
    let projection = scalar_select_projection("SELECT 1.50 AS amount", Dialect::Postgres)
        .unwrap()
        .unwrap();

    assert_eq!(projection.expression_sql, "1.50");
    assert_eq!(projection.alias.as_deref(), Some("amount"));
}

#[test]
fn scalar_select_projection_rejects_table_backed_queries() {
    assert!(
        scalar_select_projection("SELECT id FROM users", Dialect::Postgres)
            .unwrap()
            .is_none()
    );
    assert!(
        scalar_select_projection("WITH c AS (SELECT 1) SELECT 1", Dialect::Postgres)
            .unwrap()
            .is_none()
    );
}

#[test]
fn scalar_select_projection_is_structural_not_substring_based() {
    let projection =
        scalar_select_projection("SELECT 'FROM pg_class' AS literal", Dialect::Postgres)
            .unwrap()
            .unwrap();

    assert_eq!(projection.expression_sql, "'FROM pg_class'");
}

#[test]
fn scalar_select_projection_handles_similar_to_structurally() {
    let projection = scalar_select_projection("SELECT 'abc' SIMILAR TO '%b%'", Dialect::Postgres)
        .unwrap()
        .unwrap();

    assert_eq!(projection.expression_sql, "'abc' SIMILAR TO '%b%'");
}
