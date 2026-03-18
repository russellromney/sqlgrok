# API Reference

Complete type and function reference for **sqlglot-rust**.

> **See also:** [Installation](installation.md) · [Developer Guide](developer-guide.md)

---

## Table of Contents

- [Top-Level Functions](#top-level-functions)
- [Statement Enum](#statement-enum)
  - [SelectStatement](#selectstatement)
  - [InsertStatement](#insertstatement)
  - [UpdateStatement](#updatestatement)
  - [DeleteStatement](#deletestatement)
  - [CreateTableStatement](#createtablestatement)
  - [AlterTableStatement](#altertablestatement)
  - [Other Statements](#other-statements)
- [Expr Enum](#expr-enum)
  - [Expr Variants](#expr-variants)
  - [Expr Methods](#expr-methods)
- [Supporting Types](#supporting-types)
  - [SelectItem](#selectitem)
  - [FromClause / TableSource](#fromclause--tablesource)
  - [TableRef](#tableref)
  - [JoinClause / JoinType](#joinclause--jointype)
  - [OrderByItem](#orderbyitem)
  - [Cte](#cte)
  - [WindowSpec / WindowFrame](#windowspec--windowframe)
  - [InsertSource / OnConflict](#insertsource--onconflict)
  - [ColumnDef](#columndef)
  - [TableConstraint](#tableconstraint)
  - [AlterTableAction](#altertableaction)
- [Operators](#operators)
  - [BinaryOperator](#binaryoperator)
  - [UnaryOperator](#unaryoperator)
- [DataType Enum](#datatype-enum)
- [DateTimeField Enum](#datetimefield-enum)
- [QuoteStyle Enum](#quotestyle-enum)
- [Dialect Enum](#dialect-enum)
  - [Dialect List](#dialect-list)
  - [Dialect Methods](#dialect-methods)
  - [Dialect Aliases (from_str)](#dialect-aliases-from_str)
  - [Function Mapping Matrix](#function-mapping-matrix)
  - [Data Type Mapping Matrix](#data-type-mapping-matrix)
  - [Identifier Quote Styles by Dialect](#identifier-quote-styles-by-dialect)
- [Error Types](#error-types)
- [Free Functions (ast module)](#free-functions-ast-module)
- [Schema System](#schema-system)
  - [Schema Trait](#schema-trait)
  - [MappingSchema](#mappingschema)
  - [Helper Functions](#helper-functions)
  - [SchemaError](#schemaerror)
- [Optimizer](#optimizer)
  - [Scope Analysis](#scope-analysis)
  - [Annotate Types](#annotate-types)

---

## Top-Level Functions

All re-exported from the crate root (`use sqlglot_rust::*`).

| Function | Signature | Returns | Description |
| --- | --- | --- | --- |
| `parse` | `(sql: &str, dialect: Dialect) -> Result<Statement>` | `Statement` | Parse a single SQL statement into an AST |
| `parse_statements` | `(sql: &str, dialect: Dialect) -> Result<Vec<Statement>>` | `Vec<Statement>` | Parse multiple semicolon-separated statements |
| `generate` | `(stmt: &Statement, dialect: Dialect) -> String` | `String` | Generate compact single-line SQL |
| `generate_pretty` | `(stmt: &Statement, dialect: Dialect) -> String` | `String` | Generate formatted SQL with indentation |
| `transpile` | `(sql: &str, read: Dialect, write: Dialect) -> Result<String>` | `String` | Parse → transform → generate in one call |
| `transpile_statements` | `(sql: &str, read: Dialect, write: Dialect) -> Result<Vec<String>>` | `Vec<String>` | Transpile multiple statements |

**`parse_statements`** is accessed via `sqlglot_rust::parser::parse_statements`.

### Examples

```rust
use sqlglot_rust::{parse, generate, generate_pretty, transpile, Dialect};

// parse + generate roundtrip
let ast = parse("SELECT 1", Dialect::Ansi).unwrap();
assert_eq!(generate(&ast, Dialect::Ansi), "SELECT 1");

// pretty output
let pretty = generate_pretty(&ast, Dialect::Ansi);
// => "SELECT\n  1"

// transpile
let out = transpile("SELECT NOW()", Dialect::Postgres, Dialect::Tsql).unwrap();
assert_eq!(out, "SELECT GETDATE()");
```

---

## Statement Enum

```rust
pub enum Statement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
    CreateTable(CreateTableStatement),
    DropTable(DropTableStatement),
    SetOperation(SetOperationStatement),
    AlterTable(AlterTableStatement),
    CreateView(CreateViewStatement),
    DropView(DropViewStatement),
    Truncate(TruncateStatement),
    Transaction(TransactionStatement),
    Explain(ExplainStatement),
    Use(UseStatement),
    Expression(Expr),
}
```

Derives: `Debug`, `Clone`, `PartialEq`, `Serialize`, `Deserialize`.

### SelectStatement

```rust
pub struct SelectStatement {
    pub ctes: Vec<Cte>,
    pub distinct: bool,
    pub top: Option<Box<Expr>>,
    pub columns: Vec<SelectItem>,
    pub from: Option<FromClause>,
    pub joins: Vec<JoinClause>,
    pub where_clause: Option<Expr>,
    pub group_by: Vec<Expr>,
    pub having: Option<Expr>,
    pub order_by: Vec<OrderByItem>,
    pub limit: Option<Expr>,
    pub offset: Option<Expr>,
    pub fetch_first: Option<Expr>,
    pub qualify: Option<Expr>,
    pub window_definitions: Vec<WindowDefinition>,
}
```

| Field | Type | Description |
| --- | --- | --- |
| `ctes` | `Vec<Cte>` | Common Table Expressions (`WITH` clause) |
| `distinct` | `bool` | `SELECT DISTINCT` flag |
| `top` | `Option<Box<Expr>>` | T-SQL `TOP N` expression |
| `columns` | `Vec<SelectItem>` | Select list items |
| `from` | `Option<FromClause>` | Primary `FROM` source |
| `joins` | `Vec<JoinClause>` | All `JOIN` clauses |
| `where_clause` | `Option<Expr>` | `WHERE` predicate |
| `group_by` | `Vec<Expr>` | `GROUP BY` expressions |
| `having` | `Option<Expr>` | `HAVING` predicate |
| `order_by` | `Vec<OrderByItem>` | `ORDER BY` items |
| `limit` | `Option<Expr>` | `LIMIT` value |
| `offset` | `Option<Expr>` | `OFFSET` value |
| `fetch_first` | `Option<Expr>` | Oracle `FETCH FIRST N ROWS ONLY` |
| `qualify` | `Option<Expr>` | BigQuery/Snowflake `QUALIFY` clause |
| `window_definitions` | `Vec<WindowDefinition>` | Named `WINDOW` specs |

**Example:**

```rust
use sqlglot_rust::{parse, Dialect, Statement};

let stmt = parse(
    "WITH cte AS (SELECT 1 AS x) SELECT DISTINCT x FROM cte WHERE x > 0 ORDER BY x LIMIT 5",
    Dialect::Ansi,
).unwrap();

if let Statement::Select(s) = &stmt {
    assert_eq!(s.ctes.len(), 1);         // 1 CTE
    assert!(s.distinct);                   // DISTINCT
    assert_eq!(s.columns.len(), 1);        // 1 column
    assert!(s.where_clause.is_some());     // has WHERE
    assert_eq!(s.order_by.len(), 1);       // 1 ORDER BY
    assert!(s.limit.is_some());            // has LIMIT
}
```

### InsertStatement

```rust
pub struct InsertStatement {
    pub table: TableRef,
    pub columns: Vec<String>,
    pub source: InsertSource,
    pub on_conflict: Option<OnConflict>,
    pub returning: Vec<SelectItem>,
}
```

| Field | Description |
| --- | --- |
| `table` | Target table |
| `columns` | Column list (may be empty) |
| `source` | `Values(rows)`, `Query(select)`, or `Default` |
| `on_conflict` | `ON CONFLICT` / `ON DUPLICATE KEY` clause |
| `returning` | `RETURNING` clause items |

**Example:**

```rust
let stmt = parse("INSERT INTO users (name, age) VALUES ('Alice', 30)", Dialect::Ansi).unwrap();
// => Statement::Insert(InsertStatement { table: "users", columns: ["name","age"], source: Values(...) })
```

### UpdateStatement

```rust
pub struct UpdateStatement {
    pub table: TableRef,
    pub assignments: Vec<(String, Expr)>,
    pub from: Option<FromClause>,
    pub where_clause: Option<Expr>,
    pub returning: Vec<SelectItem>,
}
```

**Example:**

```rust
let stmt = parse("UPDATE users SET name = 'Bob' WHERE id = 1", Dialect::Ansi).unwrap();
// => Statement::Update(UpdateStatement { table: "users", assignments: [("name", StringLiteral("Bob"))], ... })
```

### DeleteStatement

```rust
pub struct DeleteStatement {
    pub table: TableRef,
    pub using: Option<FromClause>,
    pub where_clause: Option<Expr>,
    pub returning: Vec<SelectItem>,
}
```

**Example:**

```rust
let stmt = parse("DELETE FROM users WHERE id = 1", Dialect::Ansi).unwrap();
// => Statement::Delete(DeleteStatement { table: "users", where_clause: Some(...) })
```

### CreateTableStatement

```rust
pub struct CreateTableStatement {
    pub if_not_exists: bool,
    pub temporary: bool,
    pub table: TableRef,
    pub columns: Vec<ColumnDef>,
    pub constraints: Vec<TableConstraint>,
    pub as_select: Option<Box<Statement>>,
}
```

**Example:**

```rust
let stmt = parse(
    "CREATE TABLE IF NOT EXISTS items (id INT NOT NULL, name VARCHAR(100), price DECIMAL(10, 2))",
    Dialect::Ansi,
).unwrap();
// => Statement::CreateTable(CreateTableStatement { if_not_exists: true, columns: [3 ColumnDefs], ... })
```

### AlterTableStatement

```rust
pub struct AlterTableStatement {
    pub table: TableRef,
    pub actions: Vec<AlterTableAction>,
}
```

### Other Statements

| Type | SQL |
| --- | --- |
| `DropTableStatement` | `DROP TABLE [IF EXISTS] name [CASCADE]` |
| `SetOperationStatement` | `query UNION [ALL] query`, `INTERSECT`, `EXCEPT` |
| `CreateViewStatement` | `CREATE [OR REPLACE] [MATERIALIZED] VIEW ...` |
| `DropViewStatement` | `DROP [MATERIALIZED] VIEW [IF EXISTS] name` |
| `TruncateStatement` | `TRUNCATE TABLE name` |
| `TransactionStatement` | `BEGIN`, `COMMIT`, `ROLLBACK`, `SAVEPOINT x`, `RELEASE SAVEPOINT x`, `ROLLBACK TO x` |
| `ExplainStatement` | `EXPLAIN [ANALYZE] statement` |
| `UseStatement` | `USE database_name` |

---

## Expr Enum

### Expr Variants

| Variant | Fields | SQL Example |
| --- | --- | --- |
| `Column` | `{ name, table?, quote_style, table_quote_style }` | `t.col`, `"col"` |
| `Number` | `(String)` | `42`, `3.14` |
| `StringLiteral` | `(String)` | `'hello'` |
| `Boolean` | `(bool)` | `TRUE`, `FALSE` |
| `Null` | — | `NULL` |
| `BinaryOp` | `{ left, op, right }` | `a + b`, `x AND y` |
| `UnaryOp` | `{ op, expr }` | `-x`, `NOT a`, `~b` |
| `Function` | `{ name, args, distinct, filter?, over? }` | `COUNT(DISTINCT x)`, `SUM(a) OVER (...)` |
| `TypedFunction` | `{ func: TypedFunction, filter?, over? }` | `SUBSTRING(x, 1, 3)`, `DATE_TRUNC('month', d)` |
| `Between` | `{ expr, low, high, negated }` | `x BETWEEN 1 AND 10` |
| `InList` | `{ expr, list, negated }` | `x IN (1, 2, 3)` |
| `InSubquery` | `{ expr, subquery, negated }` | `x IN (SELECT ...)` |
| `AnyOp` | `{ expr, op, right }` | `x = ANY(ARRAY[1, 2])`, `x = ANY(SELECT ...)` |
| `AllOp` | `{ expr, op, right }` | `x > ALL(SELECT ...)` |
| `IsNull` | `{ expr, negated }` | `x IS NULL`, `x IS NOT NULL` |
| `Like` | `{ expr, pattern, negated, escape? }` | `name LIKE '%test%'` |
| `ILike` | `{ expr, pattern, negated, escape? }` | `name ILIKE '%test%'` |
| `Case` | `{ operand?, when_clauses, else_clause? }` | `CASE WHEN ... THEN ... END` |
| `Nested` | `(Box<Expr>)` | `(a + b)` |
| `Wildcard` | — | `*` (in function args) |
| `Star` | — | `*` (select list) |
| `QualifiedWildcard` | `{ table }` | `t.*` |
| `Subquery` | `(Box<Statement>)` | `(SELECT 1)` |
| `Exists` | `{ subquery, negated }` | `EXISTS (SELECT ...)` |
| `Cast` | `{ expr, data_type }` | `CAST(x AS INT)` |
| `TryCast` | `{ expr, data_type }` | `TRY_CAST(x AS INT)` |
| `Extract` | `{ field, expr }` | `EXTRACT(YEAR FROM d)` |
| `Interval` | `{ value, unit? }` | `INTERVAL '1' DAY` |
| `ArrayLiteral` | `(Vec<Expr>)` | `ARRAY[1, 2, 3]` |
| `Tuple` | `(Vec<Expr>)` | `(1, 'a', true)` |
| `Coalesce` | `(Vec<Expr>)` | `COALESCE(a, b, c)` |
| `If` | `{ condition, true_val, false_val? }` | `IF(a > 0, a, 0)` |
| `NullIf` | `{ expr, else }` | `NULLIF(a, b)` |
| `Collate` | `{ expr, collation }` | `col COLLATE utf8` |
| `Parameter` | `(String)` | `$1`, `?`, `:name` |
| `TypeExpr` | `(DataType)` | Used in DDL contexts |
| `Alias` | `{ expr, name }` | `expr AS alias` |
| `ArrayIndex` | `{ expr, index }` | `arr[0]` |
| `JsonAccess` | `{ expr, path, as_text }` | `data->'key'`, `data->>'key'` |
| `Lambda` | `{ params, body }` | `x -> x + 1` |
| `Default` | — | `DEFAULT` |
| `Cube` | `{ exprs }` | `CUBE(a, b)` |
| `Rollup` | `{ exprs }` | `ROLLUP(a, b)` |
| `GroupingSets` | `{ sets }` | `GROUPING SETS((a, b), (a), ())` |

### Expr Methods

| Method | Signature | Returns | Description |
| --- | --- | --- | --- |
| `walk` | `(&self, visitor: &mut F)` where `F: FnMut(&Expr) -> bool` | `()` | Depth-first traversal. Visitor returns `true` to recurse. |
| `find` | `(&self, predicate: &F) -> Option<&Expr>` where `F: Fn(&Expr) -> bool` | `Option<&Expr>` | First matching node |
| `find_all` | `(&self, predicate: &F) -> Vec<&Expr>` where `F: Fn(&Expr) -> bool` | `Vec<&Expr>` | All matching nodes |
| `transform` | `(self, func: &F) -> Expr` where `F: Fn(Expr) -> Expr` | `Expr` | Bottom-up tree rewrite (consumes self) |
| `is_column` | `(&self) -> bool` | `bool` | `true` if `Column` variant |
| `is_literal` | `(&self) -> bool` | `bool` | `true` if `Number`, `StringLiteral`, or `Boolean` |
| `sql` | `(&self) -> String` | `String` | Quick SQL output (ANSI dialect) |

**Examples:**

```rust
use sqlglot_rust::Expr;

// sql()
assert_eq!(Expr::Number("42".into()).sql(), "42");
assert_eq!(Expr::StringLiteral("hello".into()).sql(), "'hello'");
assert_eq!(Expr::Boolean(true).sql(), "TRUE");
assert_eq!(Expr::Null.sql(), "NULL");

// is_column / is_literal
assert!(Expr::Column { name: "a".into(), table: None,
    quote_style: Default::default(), table_quote_style: Default::default() }.is_column());
assert!(Expr::Number("1".into()).is_literal());
assert!(Expr::Boolean(false).is_literal());
assert!(!Expr::Null.is_literal());
```

---

## TypedFunction Enum

Typed function variants enable per-function transpilation rules and dialect-specific code generation.
Each variant carries semantically typed arguments rather than a generic `Vec<Expr>`.

When a recognized function name is parsed (e.g., `SUBSTRING`, `NOW`, `COUNT`), the parser creates
`Expr::TypedFunction { func, filter, over }` instead of a generic `Expr::Function`. This allows
the generator to emit the correct SQL for each target dialect without relying on string-based
function renaming.

### TypedFunction Variants

#### Date/Time

| Variant | Fields | SQL |
| --- | --- | --- |
| `DateAdd` | `{ expr, interval, unit? }` | `DATE_ADD(d, INTERVAL 1 DAY)` |
| `DateDiff` | `{ start, end, unit? }` | `DATE_DIFF(d1, d2)` |
| `DateTrunc` | `{ unit, expr }` | `DATE_TRUNC('month', d)` |
| `DateSub` | `{ expr, interval, unit? }` | `DATE_SUB(d, INTERVAL 1 DAY)` |
| `CurrentDate` | — | `CURRENT_DATE` |
| `CurrentTimestamp` | — | `CURRENT_TIMESTAMP` / `NOW()` / `GETDATE()` |
| `StrToTime` | `{ expr, format }` | `STR_TO_TIME(s, fmt)` |
| `TimeToStr` | `{ expr, format }` | `TIME_TO_STR(t, fmt)` |
| `TsOrDsToDate` | `{ expr }` | `TS_OR_DS_TO_DATE(expr)` |
| `Year` | `{ expr }` | `YEAR(d)` |
| `Month` | `{ expr }` | `MONTH(d)` |
| `Day` | `{ expr }` | `DAY(d)` |

#### String

| Variant | Fields | SQL |
| --- | --- | --- |
| `Trim` | `{ expr, trim_type, trim_chars? }` | `TRIM(LEADING 'x' FROM s)` |
| `Substring` | `{ expr, start, length? }` | `SUBSTRING(s, 1, 3)` / `SUBSTR(s, 1, 3)` |
| `Upper` | `{ expr }` | `UPPER(s)` |
| `Lower` | `{ expr }` | `LOWER(s)` |
| `RegexpLike` | `{ expr, pattern, flags? }` | `REGEXP_LIKE(s, '^A')` |
| `RegexpExtract` | `{ expr, pattern, group_index? }` | `REGEXP_EXTRACT(s, '(\\d+)')` |
| `RegexpReplace` | `{ expr, pattern, replacement, flags? }` | `REGEXP_REPLACE(s, '\\d', 'X')` |
| `ConcatWs` | `{ separator, exprs }` | `CONCAT_WS(',', a, b, c)` |
| `Split` | `{ expr, delimiter }` | `SPLIT(s, ',')` |
| `Initcap` | `{ expr }` | `INITCAP(s)` |
| `Length` | `{ expr }` | `LENGTH(s)` / `LEN(s)` |
| `Replace` | `{ expr, from, to }` | `REPLACE(s, 'old', 'new')` |
| `Reverse` | `{ expr }` | `REVERSE(s)` |
| `Left` | `{ expr, n }` | `LEFT(s, 3)` |
| `Right` | `{ expr, n }` | `RIGHT(s, 3)` |
| `Lpad` | `{ expr, length, pad? }` | `LPAD(s, 10, '0')` |
| `Rpad` | `{ expr, length, pad? }` | `RPAD(s, 10, '0')` |

#### Aggregate

| Variant | Fields | SQL |
| --- | --- | --- |
| `Count` | `{ expr, distinct }` | `COUNT(*)`, `COUNT(DISTINCT x)` |
| `Sum` | `{ expr, distinct }` | `SUM(x)`, `SUM(DISTINCT x)` |
| `Avg` | `{ expr, distinct }` | `AVG(x)` |
| `Min` | `{ expr }` | `MIN(x)` |
| `Max` | `{ expr }` | `MAX(x)` |
| `ArrayAgg` | `{ expr, distinct }` | `ARRAY_AGG(x)` / `LIST(x)` / `COLLECT_LIST(x)` |
| `ApproxDistinct` | `{ expr }` | `APPROX_DISTINCT(x)` |
| `Variance` | `{ expr }` | `VARIANCE(x)` / `VAR_SAMP(x)` |
| `Stddev` | `{ expr }` | `STDDEV(x)` / `STDDEV_SAMP(x)` |

#### Array

| Variant | Fields | SQL |
| --- | --- | --- |
| `ArrayConcat` | `{ arrays }` | `ARRAY_CONCAT(a, b)` |
| `ArrayContains` | `{ array, element }` | `ARRAY_CONTAINS(arr, 1)` |
| `ArraySize` | `{ expr }` | `ARRAY_SIZE(arr)` / `ARRAY_LENGTH(arr)` |
| `Explode` | `{ expr }` | `EXPLODE(arr)` |
| `GenerateSeries` | `{ start, stop, step? }` | `GENERATE_SERIES(1, 10)` |
| `Flatten` | `{ expr }` | `FLATTEN(arr)` |

#### JSON

| Variant | Fields | SQL |
| --- | --- | --- |
| `JSONExtract` | `{ expr, path }` | `JSON_EXTRACT(doc, '$.key')` |
| `JSONExtractScalar` | `{ expr, path }` | `JSON_EXTRACT_SCALAR(doc, '$.key')` |
| `ParseJSON` | `{ expr }` | `PARSE_JSON(s)` |
| `JSONFormat` | `{ expr }` | `JSON_FORMAT(obj)` / `TO_JSON(obj)` |

#### Window

| Variant | Fields | SQL |
| --- | --- | --- |
| `RowNumber` | — | `ROW_NUMBER()` |
| `Rank` | — | `RANK()` |
| `DenseRank` | — | `DENSE_RANK()` |
| `NTile` | `{ n }` | `NTILE(4)` |
| `Lead` | `{ expr, offset?, default? }` | `LEAD(x, 1, 0)` |
| `Lag` | `{ expr, offset?, default? }` | `LAG(x, 1, 0)` |
| `FirstValue` | `{ expr }` | `FIRST_VALUE(x)` |
| `LastValue` | `{ expr }` | `LAST_VALUE(x)` |

#### Math

| Variant | Fields | SQL |
| --- | --- | --- |
| `Abs` | `{ expr }` | `ABS(x)` |
| `Ceil` | `{ expr }` | `CEIL(x)` / `CEILING(x)` |
| `Floor` | `{ expr }` | `FLOOR(x)` |
| `Round` | `{ expr, decimals? }` | `ROUND(x, 2)` |
| `Log` | `{ expr, base? }` | `LOG(x)` |
| `Ln` | `{ expr }` | `LN(x)` |
| `Pow` | `{ base, exponent }` | `POW(x, 2)` / `POWER(x, 2)` |
| `Sqrt` | `{ expr }` | `SQRT(x)` |
| `Greatest` | `{ exprs }` | `GREATEST(a, b, c)` |
| `Least` | `{ exprs }` | `LEAST(a, b, c)` |
| `Mod` | `{ left, right }` | `MOD(a, b)` |

#### Conversion

| Variant | Fields | SQL |
| --- | --- | --- |
| `Hex` | `{ expr }` | `HEX(x)` / `TO_HEX(x)` |
| `Unhex` | `{ expr }` | `UNHEX(x)` / `FROM_HEX(x)` |
| `Md5` | `{ expr }` | `MD5(x)` |
| `Sha` | `{ expr }` | `SHA(x)` / `SHA1(x)` |
| `Sha2` | `{ expr, bit_length }` | `SHA2(x, 256)` |

### TrimType Enum

```rust
pub enum TrimType {
    Leading,
    Trailing,
    Both,
}
```

### Dialect-Specific Function Generation

The generator emits different SQL for each `TypedFunction` variant depending on the target dialect:

| TypedFunction | ANSI / Default | T-SQL | MySQL | Oracle | Hive/Spark | Presto/Trino |
| --- | --- | --- | --- | --- | --- | --- |
| `CurrentTimestamp` | `CURRENT_TIMESTAMP()` | `GETDATE()` | `NOW()` | `CURRENT_TIMESTAMP()` | `CURRENT_TIMESTAMP()` | `CURRENT_TIMESTAMP()` |
| `Substring` | `SUBSTRING(x, a, b)` | `SUBSTRING(x, a, b)` | `SUBSTR(x, a, b)` | `SUBSTR(x, a, b)` | `SUBSTR(x, a, b)` | `SUBSTRING(x, a, b)` |
| `Length` | `LENGTH(x)` | `LEN(x)` | `LENGTH(x)` | `LENGTH(x)` | `LENGTH(x)` | `LENGTH(x)` |
| `DateTrunc` | `DATE_TRUNC('m', d)` | `DATETRUNC(m, d)` | `DATE_TRUNC('m', d)` | `TRUNC(d, 'm')` | `DATE_TRUNC('m', d)` | `DATE_TRUNC('m', d)` |
| `Ceil` | `CEIL(x)` | `CEILING(x)` | `CEIL(x)` | `CEIL(x)` | `CEIL(x)` | `CEIL(x)` |
| `Pow` | `POW(x, n)` | `POWER(x, n)` | `POW(x, n)` | `POWER(x, n)` | `POW(x, n)` | `POW(x, n)` |
| `ArrayAgg` | `ARRAY_AGG(x)` | `ARRAY_AGG(x)` | `ARRAY_AGG(x)` | `ARRAY_AGG(x)` | `COLLECT_LIST(x)` | `ARRAY_AGG(x)` |
| `Hex` | `HEX(x)` | `HEX(x)` | `HEX(x)` | `HEX(x)` | `HEX(x)` | `TO_HEX(x)` |
| `Unhex` | `UNHEX(x)` | `UNHEX(x)` | `UNHEX(x)` | `UNHEX(x)` | `UNHEX(x)` | `FROM_HEX(x)` |
| `Sha` | `SHA(x)` | `SHA(x)` | `SHA(x)` | `SHA(x)` | `SHA(x)` | `SHA1(x)` |
| `JSONFormat` | `JSON_FORMAT(x)` | `JSON_FORMAT(x)` | `JSON_FORMAT(x)` | `JSON_FORMAT(x)` | `JSON_FORMAT(x)` | `TO_JSON(x)` |

---

## Supporting Types

### SelectItem

```rust
pub enum SelectItem {
    Wildcard,                              // *
    QualifiedWildcard { table: String },   // t.*
    Expr { expr: Expr, alias: Option<String> },  // expr [AS alias]
}
```

### FromClause / TableSource

```rust
pub struct FromClause {
    pub source: TableSource,
}

pub enum TableSource {
    Table(TableRef),
    Subquery { query: Box<Statement>, alias: Option<String> },
    TableFunction { name: String, args: Vec<Expr>, alias: Option<String> },
    Lateral { source: Box<TableSource> },
    Unnest { expr: Box<Expr>, alias: Option<String>, with_offset: bool },
    Pivot { source: Box<TableSource>, aggregate: Box<Expr>, for_column: String, in_values: Vec<PivotValue>, alias: Option<String> },
    Unpivot { source: Box<TableSource>, value_column: String, for_column: String, in_columns: Vec<PivotValue>, alias: Option<String> },
}

pub struct PivotValue {
    pub value: Expr,
    pub alias: Option<String>,
}
```

**PIVOT** rotates rows into columns using an aggregate function:
```sql
SELECT * FROM sales PIVOT (SUM(amount) FOR quarter IN ('Q1', 'Q2', 'Q3')) AS pvt
```

**UNPIVOT** rotates columns back into rows:
```sql
SELECT * FROM quarterly UNPIVOT (amount FOR quarter IN (Q1, Q2, Q3, Q4)) AS unpvt
```

Pivot values/columns may include aliases (`'Q1' AS q1`). Supported in T-SQL, Oracle, Snowflake, BigQuery, Spark, and other dialects.

### TableRef

```rust
pub struct TableRef {
    pub catalog: Option<String>,
    pub schema: Option<String>,
    pub name: String,
    pub alias: Option<String>,
    pub name_quote_style: QuoteStyle,
}
```

### JoinClause / JoinType

```rust
pub struct JoinClause {
    pub join_type: JoinType,
    pub table: TableSource,
    pub on: Option<Expr>,
    pub using: Vec<String>,
}

pub enum JoinType {
    Inner, Left, Right, Full, Cross, Natural, Lateral,
}
```

### OrderByItem

```rust
pub struct OrderByItem {
    pub expr: Expr,
    pub ascending: bool,
    pub nulls_first: Option<bool>,
}
```

### Cte

```rust
pub struct Cte {
    pub name: String,
    pub columns: Vec<String>,
    pub query: Box<Statement>,
    pub materialized: Option<bool>,
    pub recursive: bool,
}
```

### WindowSpec / WindowFrame

```rust
pub struct WindowSpec {
    pub window_ref: Option<String>,
    pub partition_by: Vec<Expr>,
    pub order_by: Vec<OrderByItem>,
    pub frame: Option<WindowFrame>,
}

pub struct WindowFrame {
    pub kind: WindowFrameKind,       // Rows | Range | Groups
    pub start: WindowFrameBound,
    pub end: Option<WindowFrameBound>,
}

pub enum WindowFrameBound {
    CurrentRow,
    Preceding(Option<Box<Expr>>),   // None = UNBOUNDED PRECEDING
    Following(Option<Box<Expr>>),   // None = UNBOUNDED FOLLOWING
}
```

### InsertSource / OnConflict

```rust
pub enum InsertSource {
    Values(Vec<Vec<Expr>>),
    Query(Box<Statement>),
    Default,
}

pub struct OnConflict {
    pub columns: Vec<String>,
    pub action: ConflictAction,
}

pub enum ConflictAction {
    DoNothing,
    DoUpdate(Vec<(String, Expr)>),
}
```

### ColumnDef

```rust
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
    pub nullable: Option<bool>,
    pub default: Option<Expr>,
    pub primary_key: bool,
    pub unique: bool,
    pub auto_increment: bool,
    pub collation: Option<String>,
    pub comment: Option<String>,
}
```

### TableConstraint

```rust
pub enum TableConstraint {
    PrimaryKey { name: Option<String>, columns: Vec<String> },
    Unique { name: Option<String>, columns: Vec<String> },
    ForeignKey {
        name: Option<String>,
        columns: Vec<String>,
        ref_table: TableRef,
        ref_columns: Vec<String>,
        on_delete: Option<ReferentialAction>,
        on_update: Option<ReferentialAction>,
    },
    Check { name: Option<String>, expr: Expr },
}

pub enum ReferentialAction {
    Cascade, Restrict, NoAction, SetNull, SetDefault,
}
```

### AlterTableAction

```rust
pub enum AlterTableAction {
    AddColumn(ColumnDef),
    DropColumn { name: String, if_exists: bool },
    RenameColumn { old_name: String, new_name: String },
    AlterColumnType { name: String, data_type: DataType },
    AddConstraint(TableConstraint),
    DropConstraint { name: String },
    RenameTable { new_name: String },
}
```

---

## Operators

### BinaryOperator

| Variant | SQL |
| --- | --- |
| `Plus` | `+` |
| `Minus` | `-` |
| `Multiply` | `*` |
| `Divide` | `/` |
| `Modulo` | `%` |
| `Eq` | `=` |
| `Neq` | `<>` |
| `Lt` | `<` |
| `Gt` | `>` |
| `LtEq` | `<=` |
| `GtEq` | `>=` |
| `And` | `AND` |
| `Or` | `OR` |
| `Xor` | `XOR` |
| `Concat` | `\|\|` |
| `BitwiseAnd` | `&` |
| `BitwiseOr` | `\|` |
| `BitwiseXor` | `^` |
| `ShiftLeft` | `<<` |
| `ShiftRight` | `>>` |
| `Arrow` | `->` |
| `DoubleArrow` | `->>` |

### UnaryOperator

| Variant | SQL |
| --- | --- |
| `Not` | `NOT` |
| `Minus` | `-` (negation) |
| `Plus` | `+` (positive) |
| `BitwiseNot` | `~` |

---

## DataType Enum

50+ SQL types organized by category.

### Numeric

| Variant | Parameters | Example SQL |
| --- | --- | --- |
| `TinyInt` | — | `TINYINT` |
| `SmallInt` | — | `SMALLINT` |
| `Int` | — | `INT` |
| `BigInt` | — | `BIGINT` |
| `Float` | — | `FLOAT` |
| `Double` | — | `DOUBLE` |
| `Decimal` | `precision: Option<u32>, scale: Option<u32>` | `DECIMAL(10, 2)` |
| `Numeric` | `precision: Option<u32>, scale: Option<u32>` | `NUMERIC(5)` |
| `Real` | — | `REAL` |
| `Serial` | — | `SERIAL` |
| `BigSerial` | — | `BIGSERIAL` |
| `SmallSerial` | — | `SMALLSERIAL` |
| `Money` | — | `MONEY` |

### String

| Variant | Parameters | Example SQL |
| --- | --- | --- |
| `Varchar` | `Option<u32>` | `VARCHAR(255)` |
| `Char` | `Option<u32>` | `CHAR(10)` |
| `Text` | — | `TEXT` |
| `String` | — | `STRING` |

### Binary

| Variant | Parameters | Example SQL |
| --- | --- | --- |
| `Binary` | `Option<u32>` | `BINARY(16)` |
| `Varbinary` | `Option<u32>` | `VARBINARY(256)` |
| `Blob` | — | `BLOB` |
| `Bytea` | — | `BYTEA` |
| `Bytes` | — | `BYTES` |
| `Bit` | `Option<u32>` | `BIT(8)` |

### Boolean

| Variant | Example SQL |
| --- | --- |
| `Boolean` | `BOOLEAN` |

### Date / Time

| Variant | Parameters | Example SQL |
| --- | --- | --- |
| `Date` | — | `DATE` |
| `Time` | `precision: Option<u32>` | `TIME(3)` |
| `Timestamp` | `precision: Option<u32>, with_tz: bool` | `TIMESTAMP`, `TIMESTAMPTZ` |
| `DateTime` | — | `DATETIME` |
| `Interval` | — | `INTERVAL` |

### JSON

| Variant | Example SQL |
| --- | --- |
| `Json` | `JSON` |
| `Jsonb` | `JSONB` |

### UUID

| Variant | Example SQL |
| --- | --- |
| `Uuid` | `UUID` |

### Complex / Composite

| Variant | Parameters | Example SQL |
| --- | --- | --- |
| `Array` | `Option<Box<DataType>>` | `ARRAY<INT>`, `INT[]` |
| `Map` | `key: Box<DataType>, value: Box<DataType>` | `MAP<STRING, INT>` |
| `Struct` | `Vec<(String, DataType)>` | `STRUCT<name STRING, age INT>` |
| `Tuple` | `Vec<DataType>` | `TUPLE(INT, STRING)` |

### PostgreSQL-Specific

| Variant | Example SQL |
| --- | --- |
| `Inet` | `INET` |
| `Cidr` | `CIDR` |
| `Macaddr` | `MACADDR` |
| `Regclass` | `REGCLASS` |
| `Regtype` | `REGTYPE` |
| `Hstore` | `HSTORE` |
| `Geography` | `GEOGRAPHY` |
| `Geometry` | `GEOMETRY` |

### Vendor-Specific

| Variant | Dialects | Example SQL |
| --- | --- | --- |
| `Variant` | Snowflake | `VARIANT` |
| `Object` | Snowflake | `OBJECT` |
| `Super` | Redshift | `SUPER` |
| `Xml` | T-SQL, Oracle | `XML` |

### Special

| Variant | Description |
| --- | --- |
| `Null` | The NULL type |
| `Unknown(String)` | Fallback for unrecognized type names |

---

## DateTimeField Enum

Used by `EXTRACT(field FROM expr)` and `INTERVAL`:

| Variant | SQL |
| --- | --- |
| `Year` | `YEAR` |
| `Quarter` | `QUARTER` |
| `Month` | `MONTH` |
| `Week` | `WEEK` |
| `Day` | `DAY` |
| `DayOfWeek` | `DOW` |
| `DayOfYear` | `DOY` |
| `Hour` | `HOUR` |
| `Minute` | `MINUTE` |
| `Second` | `SECOND` |
| `Millisecond` | `MILLISECOND` |
| `Microsecond` | `MICROSECOND` |
| `Nanosecond` | `NANOSECOND` |
| `Epoch` | `EPOCH` |
| `Timezone` | `TIMEZONE` |
| `TimezoneHour` | `TIMEZONE_HOUR` |
| `TimezoneMinute` | `TIMEZONE_MINUTE` |

**Example:**

```rust
use sqlglot_rust::{parse, generate, Dialect};

let ast = parse("SELECT EXTRACT(YEAR FROM hire_date) FROM employees", Dialect::Ansi).unwrap();
let sql = generate(&ast, Dialect::Ansi);
assert_eq!(sql, "SELECT EXTRACT(YEAR FROM hire_date) FROM employees");
```

---

## QuoteStyle Enum

```rust
pub enum QuoteStyle {
    None,         // bare identifier
    DoubleQuote,  // "identifier"
    Backtick,     // `identifier`
    Bracket,      // [identifier]
}
```

### Methods

| Method | Signature | Description |
| --- | --- | --- |
| `for_dialect` | `(dialect: Dialect) -> QuoteStyle` | Canonical quote style for a dialect |
| `is_quoted` | `(self) -> bool` | `true` unless `None` |

### Quote Style by Dialect

| Style | Dialects |
| --- | --- |
| `DoubleQuote` | ANSI, PostgreSQL, Oracle, Snowflake, Presto, Trino, Redshift, Athena, DuckDB, and others |
| `Backtick` | MySQL, BigQuery, Hive, Spark, Databricks, Doris, SingleStore, StarRocks |
| `Bracket` | T-SQL, Fabric |

**Example:**

```rust
use sqlglot_rust::{QuoteStyle, Dialect};

assert_eq!(QuoteStyle::for_dialect(Dialect::Mysql), QuoteStyle::Backtick);
assert_eq!(QuoteStyle::for_dialect(Dialect::Postgres), QuoteStyle::DoubleQuote);
assert_eq!(QuoteStyle::for_dialect(Dialect::Tsql), QuoteStyle::Bracket);
assert!(QuoteStyle::Backtick.is_quoted());
assert!(!QuoteStyle::None.is_quoted());
```

---

## Dialect Enum

### Dialect List

30 dialects in two support tiers:

| # | Variant | Display Name | Support |
| --- | --- | --- | --- |
| 1 | `Ansi` | ANSI SQL | Official |
| 2 | `Athena` | Athena | Official |
| 3 | `BigQuery` | BigQuery | Official |
| 4 | `ClickHouse` | ClickHouse | Official |
| 5 | `Databricks` | Databricks | Official |
| 6 | `DuckDb` | DuckDB | Official |
| 7 | `Hive` | Hive | Official |
| 8 | `Mysql` | MySQL | Official |
| 9 | `Oracle` | Oracle | Official |
| 10 | `Postgres` | PostgreSQL | Official |
| 11 | `Presto` | Presto | Official |
| 12 | `Redshift` | Redshift | Official |
| 13 | `Snowflake` | Snowflake | Official |
| 14 | `Spark` | Spark | Official |
| 15 | `Sqlite` | SQLite | Official |
| 16 | `StarRocks` | StarRocks | Official |
| 17 | `Trino` | Trino | Official |
| 18 | `Tsql` | T-SQL | Official |
| 19 | `Doris` | Doris | Community |
| 20 | `Dremio` | Dremio | Community |
| 21 | `Drill` | Drill | Community |
| 22 | `Druid` | Druid | Community |
| 23 | `Exasol` | Exasol | Community |
| 24 | `Fabric` | Fabric | Community |
| 25 | `Materialize` | Materialize | Community |
| 26 | `Prql` | PRQL | Community |
| 27 | `RisingWave` | RisingWave | Community |
| 28 | `SingleStore` | SingleStore | Community |
| 29 | `Tableau` | Tableau | Community |
| 30 | `Teradata` | Teradata | Community |

### Dialect Methods

| Method | Signature | Description |
| --- | --- | --- |
| `all()` | `-> &'static [Dialect]` | All 30 dialect variants |
| `from_str(s)` | `(s: &str) -> Option<Dialect>` | Case-insensitive lookup with aliases |
| `support_level()` | `(&self) -> &'static str` | `"Official"` or `"Community"` |
| `Display` trait | `format!("{}", dialect)` | Human-readable name |

### Dialect Aliases (from_str)

| Input String(s) | Resolved Dialect |
| --- | --- |
| `postgres`, `postgresql` | `Dialect::Postgres` |
| `tsql`, `mssql`, `sqlserver` | `Dialect::Tsql` |
| `bigquery` | `Dialect::BigQuery` |
| `clickhouse` | `Dialect::ClickHouse` |
| `hive` | `Dialect::Hive` |
| `spark` | `Dialect::Spark` |
| `mysql` | `Dialect::Mysql` |
| `sqlite` | `Dialect::Sqlite` |
| `duckdb` | `Dialect::DuckDb` |

(All lookups are case-insensitive.)

**Example:**

```rust
use sqlglot_rust::Dialect;

assert_eq!(Dialect::from_str("postgresql"), Some(Dialect::Postgres));
assert_eq!(Dialect::from_str("mssql"), Some(Dialect::Tsql));
assert_eq!(Dialect::from_str("BIGQUERY"), Some(Dialect::BigQuery));
assert_eq!(Dialect::from_str("unknown"), None);

assert_eq!(Dialect::Postgres.support_level(), "Official");
assert_eq!(Dialect::Teradata.support_level(), "Community");

assert_eq!(format!("{}", Dialect::DuckDb), "DuckDB");
assert_eq!(Dialect::all().len(), 30);
```

### Function Mapping Matrix

Transformations applied automatically during transpilation via typed function expressions:

| Source Function | Target Dialects | Mapped To |
| --- | --- | --- |
| `NOW()` | BigQuery, Snowflake, ANSI, Hive, Spark, Presto, Trino, ClickHouse | `CURRENT_TIMESTAMP()` |
| `NOW()` | T-SQL | `GETDATE()` |
| `GETDATE()` | PostgreSQL, DuckDB, SQLite, Redshift | `NOW()` |
| `GETDATE()` | BigQuery, ANSI | `CURRENT_TIMESTAMP()` |
| `SUBSTRING(x, a, b)` | MySQL, SQLite, Oracle, Hive, Spark, Databricks | `SUBSTR(x, a, b)` |
| `SUBSTR(x, a, b)` | PostgreSQL, DuckDB, ANSI, Presto, Trino | `SUBSTRING(x, a, b)` |
| `LEN(x)` | PostgreSQL, MySQL, SQLite, DuckDB, ANSI | `LENGTH(x)` |
| `LENGTH(x)` | T-SQL | `LEN(x)` |
| `CEIL(x)` | T-SQL | `CEILING(x)` |
| `POW(x, n)` | T-SQL, Oracle | `POWER(x, n)` |
| `DATE_TRUNC(u, d)` | T-SQL | `DATETRUNC(u, d)` |
| `DATE_TRUNC(u, d)` | Oracle | `TRUNC(d, u)` |
| `ARRAY_AGG(x)` | DuckDB | `LIST(x)` |
| `ARRAY_AGG(x)` | Hive, Spark, Databricks | `COLLECT_LIST(x)` |
| `HEX(x)` | Presto, Trino | `TO_HEX(x)` |
| `UNHEX(x)` | Presto, Trino | `FROM_HEX(x)` |
| `SHA(x)` | Presto, Trino | `SHA1(x)` |
| `JSON_FORMAT(x)` | Presto, Trino | `TO_JSON(x)` |
| `IFNULL(a, b)` | PostgreSQL, ANSI, DuckDB | `COALESCE(a, b)` |
| `IFNULL(a, b)` | T-SQL | `ISNULL(a, b)` |

### Data Type Mapping Matrix

| Source Type | Target Dialect | Mapped Type |
| --- | --- | --- |
| `TEXT` | BigQuery | `STRING` |
| `STRING` | PostgreSQL, MySQL, SQLite | `TEXT` |
| `INT` | BigQuery | `BIGINT` |
| `FLOAT` | BigQuery | `DOUBLE` |
| `BYTEA` | MySQL, SQLite, BigQuery | `BLOB` |
| `BLOB` | PostgreSQL | `BYTEA` |

### Identifier Quote Styles by Dialect

| Quote Style | Dialects |
| --- | --- |
| `"double"` | ANSI, PostgreSQL, Oracle, Snowflake, Presto, Trino, Redshift, Athena, DuckDB |
| `` `backtick` `` | MySQL, BigQuery, Hive, Spark, Databricks, Doris, SingleStore, StarRocks |
| `[bracket]` | T-SQL, Fabric |

---

## Error Types

```rust
pub enum SqlglotError {
    TokenizerError { message: String, position: usize },
    ParserError { message: String },
    UnexpectedToken { token: Token },
    UnsupportedDialectFeature(String),
    Internal(String),
}
```

| Variant | When It Occurs | Key Fields |
| --- | --- | --- |
| `TokenizerError` | Invalid character or unterminated string | `message`, `position` (byte offset) |
| `ParserError` | Grammar / syntax error | `message` |
| `UnexpectedToken` | Parser hit an unexpected token | `token` (full `Token` struct) |
| `UnsupportedDialectFeature` | Feature unavailable in target dialect | Description string |
| `Internal` | Library bug | Description string |

All variants implement `std::fmt::Display` and `std::error::Error` via `thiserror`.

**Example:**

```rust
use sqlglot_rust::{parse, Dialect, SqlglotError};

let err = parse("SELECT ???", Dialect::Ansi).unwrap_err();
match err {
    SqlglotError::ParserError { message } => println!("Parse: {message}"),
    SqlglotError::TokenizerError { message, position } => {
        println!("Token error at byte {position}: {message}")
    }
    other => println!("{other}"),
}
```

---

## Free Functions (ast module)

Accessed via `use sqlglot_rust::ast::*`.

| Function | Signature | Returns | Description |
| --- | --- | --- | --- |
| `find_columns` | `(expr: &Expr) -> Vec<&Expr>` | `Vec<&Expr>` | All `Column` nodes in an expression tree |
| `find_tables` | `(stmt: &Statement) -> Vec<&TableRef>` | `Vec<&TableRef>` | All `TableRef` nodes in a statement |

**Example:**

```rust
use sqlglot_rust::{parse, Dialect};
use sqlglot_rust::ast::find_tables;

let stmt = parse(
    "SELECT a FROM t1 INNER JOIN t2 ON t1.id = t2.id",
    Dialect::Ansi,
).unwrap();
let tables = find_tables(&stmt);
assert_eq!(tables.len(), 2);
assert_eq!(tables[0].name, "t1");
assert_eq!(tables[1].name, "t2");
```

---

## Schema System

Accessed via `use sqlglot_rust::schema::*`.

The schema module provides dialect-aware table and column metadata management, serving as the foundation for type annotation, column qualification, and lineage analysis.

### Schema Trait

| Method | Signature | Returns | Description |
| --- | --- | --- | --- |
| `add_table` | `(&mut self, table_path: &[&str], columns: Vec<(String, DataType)>)` | `Result<(), SchemaError>` | Register a table with column definitions |
| `column_names` | `(&self, table_path: &[&str])` | `Result<Vec<String>, SchemaError>` | Get column names in definition order |
| `get_column_type` | `(&self, table_path: &[&str], column: &str)` | `Result<DataType, SchemaError>` | Get the data type of a column |
| `has_column` | `(&self, table_path: &[&str], column: &str)` | `bool` | Check if a column exists |
| `dialect` | `(&self)` | `Dialect` | Get the configured dialect |

### MappingSchema

In-memory implementation of `Schema` with 3-level nesting: `catalog → database → table → column → type`.

| Method | Signature | Returns | Description |
| --- | --- | --- | --- |
| `new` | `(dialect: Dialect)` | `MappingSchema` | Create an empty schema for the given dialect |
| `replace_table` | `(&mut self, table_path: &[&str], columns: Vec<(String, DataType)>)` | `Result<(), SchemaError>` | Add or overwrite a table |
| `remove_table` | `(&mut self, table_path: &[&str])` | `Result<bool, SchemaError>` | Remove a table (returns true if existed) |
| `add_udf` | `(&mut self, name: &str, return_type: DataType)` | `()` | Register a UDF with its return type |
| `get_udf_type` | `(&self, name: &str)` | `Option<&DataType>` | Look up a UDF's return type |
| `table_names` | `(&self)` | `Vec<(String, String, String)>` | List all registered tables |

**Table path formats:**

| Path | Example | Resolved as |
| --- | --- | --- |
| `&["table"]` | `&["users"]` | `"" → "" → "users"` |
| `&["db", "table"]` | `&["public", "users"]` | `"" → "public" → "users"` |
| `&["catalog", "db", "table"]` | `&["prod", "public", "users"]` | `"prod" → "public" → "users"` |

**Identifier normalization:** Unquoted identifiers are lowercased for case-insensitive dialects (Postgres, MySQL, Snowflake, etc.) and kept as-is for case-sensitive dialects (BigQuery, Hive, Spark, Databricks).

### Helper Functions

| Function | Signature | Returns | Description |
| --- | --- | --- | --- |
| `ensure_schema` | `(tables: HashMap<String, HashMap<String, DataType>>, dialect: Dialect)` | `MappingSchema` | Build schema from `table → column → type` map |
| `ensure_schema_nested` | `(catalog_map: CatalogMap, dialect: Dialect)` | `MappingSchema` | Build schema from `catalog → db → table → column → type` map |
| `normalize_identifier` | `(name: &str, dialect: Dialect)` | `String` | Normalize an identifier per dialect rules |
| `is_case_sensitive_dialect` | `(dialect: Dialect)` | `bool` | Check if a dialect is case-sensitive |

### SchemaError

| Variant | Description |
| --- | --- |
| `TableNotFound(String)` | The referenced table was not found |
| `ColumnNotFound { table, column }` | The column was not found in the table |
| `DuplicateTable(String)` | Table is already registered (use `replace_table` instead) |

`SchemaError` implements `From<SchemaError> for SqlglotError`.

---

## Optimizer

Accessed via `use sqlglot_rust::optimizer::optimize`.

| Function | Signature | Returns | Description |
| --- | --- | --- | --- |
| `optimize` | `(stmt: Statement) -> Result<Statement>` | `Statement` | Apply all optimization passes |
| `build_scope` | `(stmt: &Statement) -> Scope` | `Scope` | Build a scope tree from a parsed statement |
| `find_all_in_scope` | `(scope: &Scope, predicate: &F) -> Vec<&ColumnRef>` | `Vec<&ColumnRef>` | Find columns matching a predicate within a single scope |
| `annotate_types` | `(stmt: &Statement, schema: &S) -> TypeAnnotations` | `TypeAnnotations` | Infer SQL types for all expression nodes |

### Optimization Passes

| Pass | Description | Example |
| --- | --- | --- |
| **Constant Folding** | Evaluate compile-time expressions | `1 + 2` → `3` |
| **Boolean Simplification** | Eliminate tautologies / contradictions | `TRUE AND x` → `x` |
| **Dead Predicate Elimination** | Remove trivially-true WHERE clauses | `WHERE TRUE` → removed |
| **Subquery Unnesting** | Decorrelate subqueries into JOINs | `WHERE EXISTS (… WHERE b.id = a.id)` → `INNER JOIN` |
| **Qualify Columns** | Resolve and qualify column references | `SELECT col FROM t` → `SELECT t.col FROM t` |
| **Annotate Types** | Infer SQL types for all AST expression nodes | `Column(id)` → `Int`, `COUNT(*)` → `BigInt` |

### Subquery Unnesting Details

`unnest_subqueries` rewrites correlated subqueries in WHERE clauses into JOINs.

Accessed via `use sqlglot_rust::optimizer::unnest_subqueries::unnest_subqueries`.

| Function | Signature | Returns | Description |
| --- | --- | --- | --- |
| `unnest_subqueries` | `(stmt: Statement) -> Statement` | `Statement` | Decorrelate WHERE subqueries into JOINs |

| Pattern | Rewrite |
| --- | --- |
| `WHERE EXISTS (SELECT … WHERE b.id = a.id)` | `INNER JOIN (SELECT DISTINCT …) ON a.id = _u0.id` |
| `WHERE NOT EXISTS (…)` | `LEFT JOIN … WHERE _u0._sentinel IS NULL` |
| `WHERE x IN (SELECT col FROM …)` | `INNER JOIN (SELECT DISTINCT col AS _col0 …)` |
| `WHERE x NOT IN (SELECT col FROM …)` | `LEFT JOIN … WHERE _u0._col0 IS NULL` |

The pass bails out (no-op) when:

- No equality correlation exists in the subquery.
- Non-equality correlations are present (e.g., `<`, `>`) — would need LATERAL.
- The subquery is embedded in a SELECT-list function (e.g., `COALESCE`).

**Example:**

```rust
use sqlglot_rust::{parse, generate, Dialect};
use sqlglot_rust::optimizer::optimize;

let stmt = parse("SELECT 1 + 2 * 3", Dialect::Ansi).unwrap();
let opt = optimize(stmt).unwrap();
assert_eq!(generate(&opt, Dialect::Ansi), "SELECT 7");

let stmt = parse("SELECT a FROM t WHERE TRUE AND x > 1", Dialect::Ansi).unwrap();
let opt = optimize(stmt).unwrap();
assert_eq!(generate(&opt, Dialect::Ansi), "SELECT a FROM t WHERE x > 1");
```

### Qualify Columns

`qualify_columns` resolves column references against the schema, adds table qualifiers to
unqualified columns, and expands wildcard selects (`*`, `t.*`) into explicit column lists.

Accessed via `use sqlglot_rust::optimizer::qualify_columns::qualify_columns`.

| Function | Signature | Returns | Description |
| --- | --- | --- | --- |
| `qualify_columns` | `(stmt: Statement, schema: &S) -> Statement` | `Statement` | Qualify column references and expand wildcards |

The schema parameter must implement the `Schema` trait. The most common implementation
is `MappingSchema` (see [Schema System](#schema-system)).

| Transformation | Before | After |
| --- | --- | --- |
| Qualify unqualified column | `SELECT col FROM t` | `SELECT t.col FROM t` |
| Expand `*` | `SELECT * FROM t` | `SELECT t.id, t.name FROM t` |
| Expand `t.*` | `SELECT t.* FROM t` | `SELECT t.id, t.name FROM t` |
| Qualify WHERE / GROUP BY / ORDER BY | `WHERE col = 1` | `WHERE t.col = 1` |
| Qualify JOIN ON | `ON id = other_id` | `ON a.id = b.other_id` |
| CTE column resolution | `WITH cte AS (...) SELECT col FROM cte` | `WITH cte AS (...) SELECT cte.col FROM cte` |
| Derived table columns | `SELECT col FROM (SELECT ...) AS sub` | `SELECT sub.col FROM (SELECT ...) AS sub` |
| Subquery in WHERE | `WHERE id IN (SELECT fk FROM t2)` | `WHERE t.id IN (SELECT t2.fk FROM t2)` |

Ambiguous columns (present in multiple sources) are left unqualified.

**Example:**

```rust
use sqlglot_rust::{parse, generate, Dialect};
use sqlglot_rust::optimizer::qualify_columns::qualify_columns;
use sqlglot_rust::schema::MappingSchema;

let schema = MappingSchema::new()
    .with_table(vec!["users"], vec!["id", "name", "email"])
    .with_table(vec!["orders"], vec!["id", "user_id", "amount"]);

let stmt = parse("SELECT name FROM users WHERE id = 1", Dialect::Ansi).unwrap();
let qualified = qualify_columns(stmt, &schema);
assert_eq!(
    generate(&qualified, Dialect::Ansi),
    "SELECT users.name FROM users WHERE users.id = 1"
);

// Expand wildcard
let stmt = parse("SELECT * FROM users", Dialect::Ansi).unwrap();
let qualified = qualify_columns(stmt, &schema);
assert_eq!(
    generate(&qualified, Dialect::Ansi),
    "SELECT users.id, users.name, users.email FROM users"
);
```

### Scope Analysis

Scope analysis tracks the sources, columns, and inter-scope relationships
in a query tree. It is the foundation for qualify_columns, pushdown_predicates,
annotate_types, and column lineage analysis.

Accessed via `use sqlglot_rust::optimizer::scope_analysis::{build_scope, find_all_in_scope, Scope, ScopeType}`
or the crate-level re-exports `use sqlglot_rust::{build_scope, find_all_in_scope, Scope, ScopeType}`.

#### ScopeType

| Variant | Description |
| --- | --- |
| `Root` | The outermost query |
| `Subquery` | A scalar or lateral subquery (WHERE / SELECT / HAVING) |
| `DerivedTable` | A subquery in FROM |
| `Cte` | A CTE definition (`WITH name AS (...)`) |
| `Union` | One branch of a UNION / INTERSECT / EXCEPT |
| `Udtf` | A user-defined table function / LATERAL |

#### Scope Fields

| Field | Type | Description |
| --- | --- | --- |
| `scope_type` | `ScopeType` | Kind of scope |
| `sources` | `HashMap<String, Source>` | Source name/alias → table or child scope |
| `columns` | `Vec<ColumnRef>` | Column references in this scope (not child scopes) |
| `external_columns` | `Vec<ColumnRef>` | Columns referencing an outer scope (correlations) |
| `derived_table_scopes` | `Vec<Scope>` | Child scopes from subqueries in FROM |
| `subquery_scopes` | `Vec<Scope>` | Child scopes from scalar/EXISTS/IN subqueries |
| `union_scopes` | `Vec<Scope>` | Child scopes for UNION/INTERSECT/EXCEPT branches |
| `cte_scopes` | `Vec<Scope>` | Child scopes for CTE definitions |
| `selected_sources` | `HashMap<String, Source>` | Sources referenced by SELECT columns |
| `is_correlated` | `bool` | Whether scope references outer columns |

#### Example

```rust
use sqlglot_rust::{parse, Dialect, build_scope, find_all_in_scope};
use sqlglot_rust::optimizer::scope_analysis::ScopeType;

let ast = parse(
    "SELECT a FROM t1 WHERE EXISTS (SELECT 1 FROM t2 WHERE t2.id = t1.id)",
    Dialect::Ansi,
).unwrap();
let scope = build_scope(&ast);

assert_eq!(scope.scope_type, ScopeType::Root);
assert!(scope.sources.contains_key("t1"));
assert_eq!(scope.subquery_scopes.len(), 1);

let sub = &scope.subquery_scopes[0];
assert!(sub.is_correlated);
assert!(sub.external_columns.iter().any(|c| c.table.as_deref() == Some("t1")));

// Find all columns referencing t1 in the root scope
let t1_cols = find_all_in_scope(&scope, &|c| c.table.as_deref() == Some("t1"));
assert!(!t1_cols.is_empty());
```

### Annotate Types

`annotate_types` infers and propagates SQL data types across all AST expression nodes
using schema metadata. It is the foundation for type-aware transpilation and validation.

Accessed via `use sqlglot_rust::optimizer::annotate_types::{annotate_types, TypeAnnotations}`
or the crate-level re-exports `use sqlglot_rust::{annotate_types, TypeAnnotations}`.

| Function | Signature | Returns | Description |
| --- | --- | --- | --- |
| `annotate_types` | `(stmt: &Statement, schema: &S) -> TypeAnnotations` | `TypeAnnotations` | Annotate all expressions with inferred types |

#### TypeAnnotations

| Method | Signature | Returns | Description |
| --- | --- | --- | --- |
| `get_type` | `(&self, expr: &Expr) -> Option<&DataType>` | `Option<&DataType>` | Get the inferred type of an expression node |
| `len` | `(&self) -> usize` | `usize` | Number of annotated nodes |
| `is_empty` | `(&self) -> bool` | `bool` | Whether any annotations exist |

> **Important:** `TypeAnnotations` stores raw pointer references. The statement must not be
> moved or dropped while the annotations are in use. Work with the statement by reference
> after calling `annotate_types`.

#### Type Propagation Rules

| Expression | Inferred Type | Rule |
| --- | --- | --- |
| `42` (integer literal) | `Int` / `BigInt` | Fits `i32` → `Int`, otherwise `BigInt` |
| `3.14` (decimal literal) | `Double` | Contains `.`, `e`, or `E` |
| `'hello'` (string literal) | `Varchar` | All string literals |
| `TRUE` / `FALSE` | `Boolean` | Boolean literals |
| `NULL` | `Null` | Null literal |
| `Column(name)` | schema type | Looked up from `Schema` |
| `a + b` (arithmetic) | coerced type | Wider of operand types |
| `a > b` (comparison) | `Boolean` | All comparison operators |
| `a AND b` (logical) | `Boolean` | Logical operators |
| `a \|\| b` (concat) | `Varchar` | String concatenation |
| `CAST(x AS T)` | `T` | Target type |
| `CASE WHEN ... THEN a ELSE b END` | common type | Widest of all THEN/ELSE branches |
| `COUNT(*)` | `BigInt` | Always BigInt |
| `SUM(int_col)` | `BigInt` | Integer inputs → BigInt |
| `SUM(decimal_col)` | `Decimal` | Preserves precision/scale |
| `AVG(x)` | `Double` | Always Double |
| `MIN(x)` / `MAX(x)` | input type | Same as argument type |
| `UPPER(x)` / `LOWER(x)` | `Varchar` | String functions |
| `LENGTH(x)` | `Int` | Returns integer |
| `EXISTS (...)` | `Boolean` | Existence check |
| `x BETWEEN a AND b` | `Boolean` | Boolean predicate |
| `x IN (...)` | `Boolean` | Boolean predicate |
| `x IS NULL` | `Boolean` | Boolean predicate |
| `EXTRACT(YEAR FROM x)` | `Int` | Date part extraction |
| `ROW_NUMBER()` / `RANK()` | `BigInt` | Window ranking |
| `LEAD(x)` / `LAG(x)` | input type | Same as argument |
| `JSON_EXTRACT(...)` | `Json` | JSON type |
| `JSON_EXTRACT_SCALAR(...)` | `Varchar` | Text extraction |
| UDF | registered type | From `schema.add_udf(name, type)` |

#### Numeric Coercion Precedence

Wider types supersede narrower ones in arithmetic expressions:

```
Boolean < TinyInt < SmallInt < Int < BigInt < Float/Real < Double < Decimal/Numeric
```

#### Example

```rust
use sqlglot_rust::{parse, Dialect, annotate_types};
use sqlglot_rust::ast::DataType;
use sqlglot_rust::schema::{MappingSchema, Schema};

let mut schema = MappingSchema::new(Dialect::Ansi);
schema.add_table(&["users"], vec![
    ("id".to_string(), DataType::Int),
    ("name".to_string(), DataType::Varchar(Some(255))),
    ("salary".to_string(), DataType::Double),
]).unwrap();

let stmt = parse("SELECT id, name, salary * 1.1 FROM users WHERE id > 5", Dialect::Ansi).unwrap();
let ann = annotate_types(&stmt, &schema);

// Query types from the annotations
if let sqlglot_rust::Statement::Select(sel) = &stmt {
    for col in &sel.columns {
        if let sqlglot_rust::ast::SelectItem::Expr { expr, .. } = col {
            if let Some(dt) = ann.get_type(expr) {
                println!("Column type: {:?}", dt);
            }
        }
    }
}
```
