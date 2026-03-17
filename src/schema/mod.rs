//! Schema management system for schema-aware analysis and optimization.
//!
//! Provides a [`Schema`] trait and [`MappingSchema`] implementation analogous
//! to Python sqlglot's `MappingSchema`. This is the foundation for type
//! annotation, column qualification, projection pushdown, and lineage analysis.
//!
//! # Example
//!
//! ```rust
//! use sqlglot_rust::schema::{MappingSchema, Schema};
//! use sqlglot_rust::ast::DataType;
//! use sqlglot_rust::Dialect;
//!
//! let mut schema = MappingSchema::new(Dialect::Ansi);
//! schema.add_table(
//!     &["catalog", "db", "users"],
//!     vec![
//!         ("id".to_string(), DataType::Int),
//!         ("name".to_string(), DataType::Varchar(Some(255))),
//!         ("email".to_string(), DataType::Text),
//!     ],
//! ).unwrap();
//!
//! assert_eq!(
//!     schema.column_names(&["catalog", "db", "users"]).unwrap(),
//!     vec!["id", "name", "email"],
//! );
//! assert_eq!(
//!     schema.get_column_type(&["catalog", "db", "users"], "id").unwrap(),
//!     DataType::Int,
//! );
//! assert!(schema.has_column(&["catalog", "db", "users"], "id"));
//! ```

use std::collections::HashMap;

use crate::ast::DataType;
use crate::dialects::Dialect;
use crate::errors::SqlglotError;

/// Errors specific to schema operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaError {
    /// The referenced table was not found in the schema.
    TableNotFound(String),
    /// The referenced column was not found in the table.
    ColumnNotFound { table: String, column: String },
    /// Duplicate table registration.
    DuplicateTable(String),
}

impl std::fmt::Display for SchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaError::TableNotFound(t) => write!(f, "Table not found: {t}"),
            SchemaError::ColumnNotFound { table, column } => {
                write!(f, "Column '{column}' not found in table '{table}'")
            }
            SchemaError::DuplicateTable(t) => write!(f, "Table already exists: {t}"),
        }
    }
}

impl std::error::Error for SchemaError {}

impl From<SchemaError> for SqlglotError {
    fn from(e: SchemaError) -> Self {
        SqlglotError::Internal(e.to_string())
    }
}

/// Schema trait for schema-aware analysis and optimization.
///
/// Provides methods to query table and column metadata. Implementations
/// can back this with in-memory mappings, database catalogs, etc.
pub trait Schema {
    /// Register a table with its column definitions.
    ///
    /// `table_path` is a slice of identifiers representing the fully qualified
    /// table path: `[catalog, database, table]`, `[database, table]`, or `[table]`.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaError::DuplicateTable`] if the table is already registered
    /// (use `replace_table` to overwrite).
    fn add_table(
        &mut self,
        table_path: &[&str],
        columns: Vec<(String, DataType)>,
    ) -> Result<(), SchemaError>;

    /// Get the column names for a table, in definition order.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaError::TableNotFound`] if the table is not registered.
    fn column_names(&self, table_path: &[&str]) -> Result<Vec<String>, SchemaError>;

    /// Get the data type of a specific column in a table.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaError::TableNotFound`] or [`SchemaError::ColumnNotFound`].
    fn get_column_type(
        &self,
        table_path: &[&str],
        column: &str,
    ) -> Result<DataType, SchemaError>;

    /// Check whether a column exists in the given table.
    fn has_column(&self, table_path: &[&str], column: &str) -> bool;

    /// Get the dialect used for identifier normalization.
    fn dialect(&self) -> Dialect;
}

/// Column metadata stored inside a [`MappingSchema`].
#[derive(Debug, Clone, PartialEq)]
struct ColumnInfo {
    /// Original insertion order for stable column listing.
    columns: Vec<(String, DataType)>,
    /// Fast lookup by normalized column name → index into `columns`.
    index: HashMap<String, usize>,
}

impl ColumnInfo {
    fn new(columns: Vec<(String, DataType)>, dialect: Dialect) -> Self {
        let index = columns
            .iter()
            .enumerate()
            .map(|(i, (name, _))| (normalize_identifier(name, dialect), i))
            .collect();
        Self { columns, index }
    }

    fn column_names(&self) -> Vec<String> {
        self.columns.iter().map(|(n, _)| n.clone()).collect()
    }

    fn get_type(&self, column: &str, dialect: Dialect) -> Option<&DataType> {
        let key = normalize_identifier(column, dialect);
        self.index.get(&key).map(|&i| &self.columns[i].1)
    }

    fn has_column(&self, column: &str, dialect: Dialect) -> bool {
        let key = normalize_identifier(column, dialect);
        self.index.contains_key(&key)
    }
}

/// A schema backed by in-memory hash maps, supporting 3-level nesting:
/// `catalog → database → table → column → type`.
///
/// Analogous to Python sqlglot's `MappingSchema`.
///
/// Identifiers are normalized according to the configured dialect:
/// - Case-insensitive dialects (most): identifiers are lowercased for lookup.
/// - Case-sensitive dialects (e.g. BigQuery, Hive): identifiers are kept as-is.
/// - Quoted identifiers are always stored verbatim (not normalized).
#[derive(Debug, Clone)]
pub struct MappingSchema {
    dialect: Dialect,
    /// Nested map: catalog → database → table → ColumnInfo
    tables: HashMap<String, HashMap<String, HashMap<String, ColumnInfo>>>,
    /// UDF return type mappings: function_name (normalized) → return type
    udf_types: HashMap<String, DataType>,
}

impl MappingSchema {
    /// Create a new empty schema for the given dialect.
    #[must_use]
    pub fn new(dialect: Dialect) -> Self {
        Self {
            dialect,
            tables: HashMap::new(),
            udf_types: HashMap::new(),
        }
    }

    /// Replace a table if it already exists, or add it if it doesn't.
    pub fn replace_table(
        &mut self,
        table_path: &[&str],
        columns: Vec<(String, DataType)>,
    ) -> Result<(), SchemaError> {
        let (catalog, database, table) = self.resolve_path(table_path)?;
        let info = ColumnInfo::new(columns, self.dialect);
        self.tables
            .entry(catalog)
            .or_default()
            .entry(database)
            .or_default()
            .insert(table, info);
        Ok(())
    }

    /// Remove a table from the schema. Returns `true` if the table existed.
    pub fn remove_table(&mut self, table_path: &[&str]) -> Result<bool, SchemaError> {
        let (catalog, database, table) = self.resolve_path(table_path)?;
        let removed = self
            .tables
            .get_mut(&catalog)
            .and_then(|dbs| dbs.get_mut(&database))
            .map(|tbls| tbls.remove(&table).is_some())
            .unwrap_or(false);
        Ok(removed)
    }

    /// Register a UDF (user-defined function) with its return type.
    pub fn add_udf(&mut self, name: &str, return_type: DataType) {
        let key = normalize_identifier(name, self.dialect);
        self.udf_types.insert(key, return_type);
    }

    /// Get the return type of a registered UDF.
    #[must_use]
    pub fn get_udf_type(&self, name: &str) -> Option<&DataType> {
        let key = normalize_identifier(name, self.dialect);
        self.udf_types.get(&key)
    }

    /// List all registered tables as `(catalog, database, table)` triples.
    #[must_use]
    pub fn table_names(&self) -> Vec<(String, String, String)> {
        let mut result = Vec::new();
        for (catalog, dbs) in &self.tables {
            for (database, tbls) in dbs {
                for table in tbls.keys() {
                    result.push((catalog.clone(), database.clone(), table.clone()));
                }
            }
        }
        result
    }

    /// Find a table across all catalogs/databases when only a short path is given.
    /// Returns the first match found (useful for unqualified table references).
    fn find_table(&self, table_path: &[&str]) -> Option<&ColumnInfo> {
        let (catalog, database, table) = match self.resolve_path(table_path) {
            Ok(parts) => parts,
            Err(_) => return None,
        };

        // Exact match first
        if let Some(info) = self
            .tables
            .get(&catalog)
            .and_then(|dbs| dbs.get(&database))
            .and_then(|tbls| tbls.get(&table))
        {
            return Some(info);
        }

        // For single-name lookups, search all catalogs/databases
        if table_path.len() == 1 {
            let norm_name = normalize_identifier(table_path[0], self.dialect);
            for dbs in self.tables.values() {
                for tbls in dbs.values() {
                    if let Some(info) = tbls.get(&norm_name) {
                        return Some(info);
                    }
                }
            }
        }

        // For 2-part lookups (db.table), search all catalogs
        if table_path.len() == 2 {
            let norm_db = normalize_identifier(table_path[0], self.dialect);
            let norm_tbl = normalize_identifier(table_path[1], self.dialect);
            for dbs in self.tables.values() {
                if let Some(info) = dbs.get(&norm_db).and_then(|tbls| tbls.get(&norm_tbl)) {
                    return Some(info);
                }
            }
        }

        None
    }

    /// Resolve a table path into normalized (catalog, database, table) parts,
    /// filling in defaults for missing levels.
    fn resolve_path(&self, table_path: &[&str]) -> Result<(String, String, String), SchemaError> {
        match table_path.len() {
            1 => Ok((
                String::new(),
                String::new(),
                normalize_identifier(table_path[0], self.dialect),
            )),
            2 => Ok((
                String::new(),
                normalize_identifier(table_path[0], self.dialect),
                normalize_identifier(table_path[1], self.dialect),
            )),
            3 => Ok((
                normalize_identifier(table_path[0], self.dialect),
                normalize_identifier(table_path[1], self.dialect),
                normalize_identifier(table_path[2], self.dialect),
            )),
            _ => Err(SchemaError::TableNotFound(table_path.join("."))),
        }
    }

    fn format_table_path(table_path: &[&str]) -> String {
        table_path.join(".")
    }
}

impl Schema for MappingSchema {
    fn add_table(
        &mut self,
        table_path: &[&str],
        columns: Vec<(String, DataType)>,
    ) -> Result<(), SchemaError> {
        let (catalog, database, table) = self.resolve_path(table_path)?;
        let entry = self
            .tables
            .entry(catalog)
            .or_default()
            .entry(database)
            .or_default();

        if entry.contains_key(&table) {
            return Err(SchemaError::DuplicateTable(
                Self::format_table_path(table_path),
            ));
        }

        let info = ColumnInfo::new(columns, self.dialect);
        entry.insert(table, info);
        Ok(())
    }

    fn column_names(&self, table_path: &[&str]) -> Result<Vec<String>, SchemaError> {
        self.find_table(table_path)
            .map(|info| info.column_names())
            .ok_or_else(|| SchemaError::TableNotFound(Self::format_table_path(table_path)))
    }

    fn get_column_type(
        &self,
        table_path: &[&str],
        column: &str,
    ) -> Result<DataType, SchemaError> {
        let table_str = Self::format_table_path(table_path);
        let info = self
            .find_table(table_path)
            .ok_or_else(|| SchemaError::TableNotFound(table_str.clone()))?;

        info.get_type(column, self.dialect)
            .cloned()
            .ok_or(SchemaError::ColumnNotFound {
                table: table_str,
                column: column.to_string(),
            })
    }

    fn has_column(&self, table_path: &[&str], column: &str) -> bool {
        self.find_table(table_path)
            .is_some_and(|info| info.has_column(column, self.dialect))
    }

    fn dialect(&self) -> Dialect {
        self.dialect
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Identifier normalization
// ═══════════════════════════════════════════════════════════════════════

/// Normalize an identifier according to the given dialect's conventions.
///
/// Most SQL dialects treat unquoted identifiers as case-insensitive
/// (typically by converting to lowercase internally). A few dialects
/// (BigQuery, Hive, Spark, Databricks) are case-sensitive for table/column
/// names.
#[must_use]
pub fn normalize_identifier(name: &str, dialect: Dialect) -> String {
    if is_case_sensitive_dialect(dialect) {
        name.to_string()
    } else {
        name.to_lowercase()
    }
}

/// Returns `true` if the dialect treats unquoted identifiers as case-sensitive.
#[must_use]
pub fn is_case_sensitive_dialect(dialect: Dialect) -> bool {
    matches!(
        dialect,
        Dialect::BigQuery | Dialect::Hive | Dialect::Spark | Dialect::Databricks
    )
}

// ═══════════════════════════════════════════════════════════════════════
// Helper: build schema from nested maps
// ═══════════════════════════════════════════════════════════════════════

/// Build a [`MappingSchema`] from a nested map structure.
///
/// The input maps from table name → column name → data type, mirroring
/// the common pattern of constructing schemas from DDL or metadata.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
/// use sqlglot_rust::schema::{ensure_schema, Schema};
/// use sqlglot_rust::ast::DataType;
/// use sqlglot_rust::Dialect;
///
/// let mut tables = HashMap::new();
/// let mut columns = HashMap::new();
/// columns.insert("id".to_string(), DataType::Int);
/// columns.insert("name".to_string(), DataType::Varchar(Some(255)));
/// tables.insert("users".to_string(), columns);
///
/// let schema = ensure_schema(tables, Dialect::Postgres);
/// assert!(schema.has_column(&["users"], "id"));
/// ```
pub fn ensure_schema(
    tables: HashMap<String, HashMap<String, DataType>>,
    dialect: Dialect,
) -> MappingSchema {
    let mut schema = MappingSchema::new(dialect);
    for (table_name, columns) in tables {
        let col_vec: Vec<(String, DataType)> = columns.into_iter().collect();
        // Use replace_table to avoid DuplicateTable errors
        let _ = schema.replace_table(&[&table_name], col_vec);
    }
    schema
}

/// Type alias for the 3-level nested schema map:
/// `catalog → database → table → column → type`.
pub type CatalogMap = HashMap<String, HashMap<String, HashMap<String, HashMap<String, DataType>>>>;

/// Build a [`MappingSchema`] from a 3-level nested map:
/// `catalog → database → table → column → type`.
pub fn ensure_schema_nested(
    catalog_map: CatalogMap,
    dialect: Dialect,
) -> MappingSchema {
    let mut schema = MappingSchema::new(dialect);
    for (catalog, databases) in catalog_map {
        for (database, tables) in databases {
            for (table, columns) in tables {
                let col_vec: Vec<(String, DataType)> = columns.into_iter().collect();
                let _ = schema.replace_table(
                    &[&catalog, &database, &table],
                    col_vec,
                );
            }
        }
    }
    schema
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Basic operations ────────────────────────────────────────────────

    #[test]
    fn test_add_and_query_table() {
        let mut schema = MappingSchema::new(Dialect::Ansi);
        schema
            .add_table(
                &["users"],
                vec![
                    ("id".to_string(), DataType::Int),
                    ("name".to_string(), DataType::Varchar(Some(255))),
                    ("email".to_string(), DataType::Text),
                ],
            )
            .unwrap();

        assert_eq!(
            schema.column_names(&["users"]).unwrap(),
            vec!["id", "name", "email"]
        );
        assert_eq!(
            schema.get_column_type(&["users"], "id").unwrap(),
            DataType::Int
        );
        assert_eq!(
            schema.get_column_type(&["users"], "name").unwrap(),
            DataType::Varchar(Some(255))
        );
        assert!(schema.has_column(&["users"], "id"));
        assert!(schema.has_column(&["users"], "email"));
        assert!(!schema.has_column(&["users"], "nonexistent"));
    }

    #[test]
    fn test_duplicate_table_error() {
        let mut schema = MappingSchema::new(Dialect::Ansi);
        schema
            .add_table(&["t"], vec![("a".to_string(), DataType::Int)])
            .unwrap();

        let err = schema
            .add_table(&["t"], vec![("b".to_string(), DataType::Text)])
            .unwrap_err();
        assert!(matches!(err, SchemaError::DuplicateTable(_)));
    }

    #[test]
    fn test_replace_table() {
        let mut schema = MappingSchema::new(Dialect::Ansi);
        schema
            .add_table(&["t"], vec![("a".to_string(), DataType::Int)])
            .unwrap();

        schema
            .replace_table(&["t"], vec![("b".to_string(), DataType::Text)])
            .unwrap();

        assert_eq!(schema.column_names(&["t"]).unwrap(), vec!["b"]);
        assert_eq!(
            schema.get_column_type(&["t"], "b").unwrap(),
            DataType::Text
        );
    }

    #[test]
    fn test_remove_table() {
        let mut schema = MappingSchema::new(Dialect::Ansi);
        schema
            .add_table(&["t"], vec![("a".to_string(), DataType::Int)])
            .unwrap();

        assert!(schema.remove_table(&["t"]).unwrap());
        assert!(!schema.remove_table(&["t"]).unwrap());
        assert!(schema.column_names(&["t"]).is_err());
    }

    #[test]
    fn test_table_not_found() {
        let schema = MappingSchema::new(Dialect::Ansi);
        let err = schema.column_names(&["nonexistent"]).unwrap_err();
        assert!(matches!(err, SchemaError::TableNotFound(_)));
    }

    #[test]
    fn test_column_not_found() {
        let mut schema = MappingSchema::new(Dialect::Ansi);
        schema
            .add_table(&["t"], vec![("a".to_string(), DataType::Int)])
            .unwrap();

        let err = schema.get_column_type(&["t"], "z").unwrap_err();
        assert!(matches!(err, SchemaError::ColumnNotFound { .. }));
    }

    // ── Multi-level nesting ─────────────────────────────────────────────

    #[test]
    fn test_three_level_path() {
        let mut schema = MappingSchema::new(Dialect::Ansi);
        schema
            .add_table(
                &["my_catalog", "my_db", "orders"],
                vec![
                    ("order_id".to_string(), DataType::BigInt),
                    ("total".to_string(), DataType::Decimal {
                        precision: Some(10),
                        scale: Some(2),
                    }),
                ],
            )
            .unwrap();

        assert_eq!(
            schema
                .column_names(&["my_catalog", "my_db", "orders"])
                .unwrap(),
            vec!["order_id", "total"]
        );
        assert!(schema.has_column(&["my_catalog", "my_db", "orders"], "order_id"));
    }

    #[test]
    fn test_two_level_path() {
        let mut schema = MappingSchema::new(Dialect::Ansi);
        schema
            .add_table(
                &["public", "users"],
                vec![("id".to_string(), DataType::Int)],
            )
            .unwrap();

        assert_eq!(
            schema.column_names(&["public", "users"]).unwrap(),
            vec!["id"]
        );
    }

    #[test]
    fn test_short_path_searches_all() {
        let mut schema = MappingSchema::new(Dialect::Ansi);
        schema
            .add_table(
                &["catalog", "db", "orders"],
                vec![("id".to_string(), DataType::Int)],
            )
            .unwrap();

        // Single-name lookup should find it
        assert!(schema.has_column(&["orders"], "id"));
        assert_eq!(schema.column_names(&["orders"]).unwrap(), vec!["id"]);

        // Two-part lookup should find it
        assert!(schema.has_column(&["db", "orders"], "id"));
    }

    // ── Dialect-aware normalization ─────────────────────────────────────

    #[test]
    fn test_case_insensitive_dialect() {
        let mut schema = MappingSchema::new(Dialect::Postgres);
        schema
            .add_table(
                &["Users"],
                vec![("ID".to_string(), DataType::Int)],
            )
            .unwrap();

        // Lookups should be case-insensitive
        assert!(schema.has_column(&["users"], "id"));
        assert!(schema.has_column(&["USERS"], "ID"));
        assert!(schema.has_column(&["Users"], "Id"));
        assert_eq!(
            schema.get_column_type(&["users"], "id").unwrap(),
            DataType::Int
        );
    }

    #[test]
    fn test_case_sensitive_dialect() {
        let mut schema = MappingSchema::new(Dialect::BigQuery);
        schema
            .add_table(
                &["Users"],
                vec![("ID".to_string(), DataType::Int)],
            )
            .unwrap();

        // BigQuery is case-sensitive
        assert!(schema.has_column(&["Users"], "ID"));
        assert!(!schema.has_column(&["users"], "ID"));
        assert!(!schema.has_column(&["Users"], "id"));
    }

    #[test]
    fn test_hive_case_sensitive() {
        let mut schema = MappingSchema::new(Dialect::Hive);
        schema
            .add_table(
                &["MyTable"],
                vec![("Col1".to_string(), DataType::Text)],
            )
            .unwrap();

        assert!(schema.has_column(&["MyTable"], "Col1"));
        assert!(!schema.has_column(&["mytable"], "col1"));
    }

    // ── UDF return types ────────────────────────────────────────────────

    #[test]
    fn test_udf_return_type() {
        let mut schema = MappingSchema::new(Dialect::Ansi);
        schema.add_udf("my_custom_fn", DataType::Int);

        assert_eq!(
            schema.get_udf_type("my_custom_fn").unwrap(),
            &DataType::Int
        );
        // Case-insensitive for ANSI
        assert_eq!(
            schema.get_udf_type("MY_CUSTOM_FN").unwrap(),
            &DataType::Int
        );
        assert!(schema.get_udf_type("nonexistent").is_none());
    }

    #[test]
    fn test_udf_case_sensitive() {
        let mut schema = MappingSchema::new(Dialect::BigQuery);
        schema.add_udf("myFunc", DataType::Boolean);

        assert!(schema.get_udf_type("myFunc").is_some());
        assert!(schema.get_udf_type("MYFUNC").is_none());
    }

    // ── ensure_schema helpers ───────────────────────────────────────────

    #[test]
    fn test_ensure_schema() {
        let mut tables = HashMap::new();
        let mut cols = HashMap::new();
        cols.insert("id".to_string(), DataType::Int);
        cols.insert("name".to_string(), DataType::Text);
        tables.insert("users".to_string(), cols);

        let schema = ensure_schema(tables, Dialect::Postgres);
        assert!(schema.has_column(&["users"], "id"));
        assert!(schema.has_column(&["users"], "name"));
    }

    #[test]
    fn test_ensure_schema_nested() {
        let mut catalogs = HashMap::new();
        let mut databases = HashMap::new();
        let mut tables = HashMap::new();
        let mut cols = HashMap::new();
        cols.insert("order_id".to_string(), DataType::BigInt);
        tables.insert("orders".to_string(), cols);
        databases.insert("sales".to_string(), tables);
        catalogs.insert("warehouse".to_string(), databases);

        let schema = ensure_schema_nested(catalogs, Dialect::Ansi);
        assert!(schema.has_column(&["warehouse", "sales", "orders"], "order_id"));
        // Short-path lookup
        assert!(schema.has_column(&["orders"], "order_id"));
    }

    // ── table_names listing ─────────────────────────────────────────────

    #[test]
    fn test_table_names() {
        let mut schema = MappingSchema::new(Dialect::Ansi);
        schema
            .add_table(
                &["cat", "db", "t1"],
                vec![("a".to_string(), DataType::Int)],
            )
            .unwrap();
        schema
            .add_table(
                &["cat", "db", "t2"],
                vec![("b".to_string(), DataType::Int)],
            )
            .unwrap();

        let mut names = schema.table_names();
        names.sort();
        assert_eq!(names.len(), 2);
        assert!(names.iter().any(|(c, d, t)| c == "cat" && d == "db" && t == "t1"));
        assert!(names.iter().any(|(c, d, t)| c == "cat" && d == "db" && t == "t2"));
    }

    // ── Invalid path ────────────────────────────────────────────────────

    #[test]
    fn test_invalid_path_too_many_parts() {
        let mut schema = MappingSchema::new(Dialect::Ansi);
        let err = schema
            .add_table(
                &["a", "b", "c", "d"],
                vec![("x".to_string(), DataType::Int)],
            )
            .unwrap_err();
        assert!(matches!(err, SchemaError::TableNotFound(_)));
    }

    #[test]
    fn test_empty_schema_has_no_columns() {
        let schema = MappingSchema::new(Dialect::Ansi);
        assert!(!schema.has_column(&["any_table"], "any_col"));
    }

    // ── Schema error display ────────────────────────────────────────────

    #[test]
    fn test_schema_error_display() {
        let e = SchemaError::TableNotFound("users".to_string());
        assert_eq!(e.to_string(), "Table not found: users");

        let e = SchemaError::ColumnNotFound {
            table: "users".to_string(),
            column: "age".to_string(),
        };
        assert_eq!(e.to_string(), "Column 'age' not found in table 'users'");

        let e = SchemaError::DuplicateTable("users".to_string());
        assert_eq!(e.to_string(), "Table already exists: users");
    }

    // ── SchemaError → SqlglotError conversion ───────────────────────────

    #[test]
    fn test_schema_error_into_sqlglot_error() {
        let e: SqlglotError = SchemaError::TableNotFound("t".to_string()).into();
        assert!(matches!(e, SqlglotError::Internal(_)));
    }

    // ── Multiple dialects ───────────────────────────────────────────────

    #[test]
    fn test_multiple_dialects_normalization() {
        // Postgres: case-insensitive
        let mut pg = MappingSchema::new(Dialect::Postgres);
        pg.add_table(&["T"], vec![("C".to_string(), DataType::Int)])
            .unwrap();
        assert!(pg.has_column(&["t"], "c"));

        // MySQL: case-insensitive
        let mut my = MappingSchema::new(Dialect::Mysql);
        my.add_table(&["T"], vec![("C".to_string(), DataType::Int)])
            .unwrap();
        assert!(my.has_column(&["t"], "c"));

        // Spark: case-sensitive
        let mut sp = MappingSchema::new(Dialect::Spark);
        sp.add_table(&["T"], vec![("C".to_string(), DataType::Int)])
            .unwrap();
        assert!(!sp.has_column(&["t"], "c"));
        assert!(sp.has_column(&["T"], "C"));
    }

    // ── Complex data types ──────────────────────────────────────────────

    #[test]
    fn test_complex_data_types() {
        let mut schema = MappingSchema::new(Dialect::Ansi);
        schema
            .add_table(
                &["complex_table"],
                vec![
                    ("tags".to_string(), DataType::Array(Some(Box::new(DataType::Text)))),
                    ("metadata".to_string(), DataType::Json),
                    ("coords".to_string(), DataType::Struct(vec![
                        ("lat".to_string(), DataType::Double),
                        ("lng".to_string(), DataType::Double),
                    ])),
                    ("lookup".to_string(), DataType::Map {
                        key: Box::new(DataType::Text),
                        value: Box::new(DataType::Int),
                    }),
                ],
            )
            .unwrap();

        assert_eq!(
            schema.get_column_type(&["complex_table"], "tags").unwrap(),
            DataType::Array(Some(Box::new(DataType::Text)))
        );
        assert_eq!(
            schema.get_column_type(&["complex_table"], "metadata").unwrap(),
            DataType::Json
        );
    }

    // ── dialect() accessor ──────────────────────────────────────────────

    #[test]
    fn test_schema_dialect() {
        let schema = MappingSchema::new(Dialect::Snowflake);
        assert_eq!(schema.dialect(), Dialect::Snowflake);
    }
}
