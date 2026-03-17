# Developer Guide

A hands-on guide to using **sqlglot-rust** in your application — parsing SQL,
generating output, transpiling across dialects, inspecting and transforming
the AST, optimizing queries, and serializing results.

> **See also:** [Installation](installation.md) · [Reference](reference.md)

---

## Table of Contents

- [Parsing SQL](#parsing-sql)
  - [Single Statement](#single-statement)
  - [Multiple Statements](#multiple-statements)
  - [What Can Be Parsed](#what-can-be-parsed)
- [Generating SQL](#generating-sql)
  - [Compact Output](#compact-output)
  - [Pretty Printing](#pretty-printing)
- [Transpiling Between Dialects](#transpiling-between-dialects)
  - [Single Statement Transpilation](#single-statement-transpilation)
  - [Multi-Statement Transpilation](#multi-statement-transpilation)
  - [Function Mapping Examples](#function-mapping-examples)
  - [Data Type Mapping Examples](#data-type-mapping-examples)
  - [ILIKE Rewriting](#ilike-rewriting)
  - [Identifier Quoting](#identifier-quoting)
  - [LIMIT / TOP / FETCH FIRST](#limit--top--fetch-first)
- [Working with the AST](#working-with-the-ast)
  - [Matching Statement Types](#matching-statement-types)
  - [Inspecting a SELECT](#inspecting-a-select)
  - [Constructing Expressions](#constructing-expressions)
- [Traversal and Search](#traversal-and-search)
  - [Walking the Tree](#walking-the-tree)
  - [Finding Nodes](#finding-nodes)
  - [Finding Tables and Columns](#finding-tables-and-columns)
- [Transforming the AST](#transforming-the-ast)
  - [Doubling Numeric Literals](#doubling-numeric-literals)
  - [Renaming Columns](#renaming-columns)
- [Schema Management](#schema-management)
  - [Creating a Schema](#creating-a-schema)
  - [Querying the Schema](#querying-the-schema)
  - [Dialect-Aware Normalization](#dialect-aware-normalization)
  - [Building Schemas from Maps](#building-schemas-from-maps)
- [Query Optimization](#query-optimization)
  - [Constant Folding](#constant-folding)
  - [Boolean Simplification](#boolean-simplification)
  - [Subquery Unnesting](#subquery-unnesting)
- [Serialization (JSON Round-Tripping)](#serialization-json-round-tripping)
- [Error Handling](#error-handling)
- [SBOM Generation](#sbom-generation)
- [Updating the Version](#updating-the-version)

---

## Parsing SQL

### Single Statement

Use `parse` to convert a SQL string into a `Statement` AST node.

```rust
use sqlglot_rust::{parse, Dialect};

let stmt = parse("SELECT 1 + 2", Dialect::Ansi).unwrap();
```

**Input:**

```sql
SELECT 1 + 2
```

**Output (`stmt`):**

```text
Statement::Select(SelectStatement {
    columns: [Expr { expr: BinaryOp { left: Number("1"), op: Plus, right: Number("2") }, alias: None }],
    ...
})
```

### Multiple Statements

`parse_statements` splits on semicolons and returns a `Vec<Statement>`:

```rust
use sqlglot_rust::parser::parse_statements;
use sqlglot_rust::Dialect;

let stmts = parse_statements(
    "SELECT 1; SELECT 2; INSERT INTO t VALUES (1)",
    Dialect::Ansi,
).unwrap();
assert_eq!(stmts.len(), 3);
```

**Input:**

```sql
SELECT 1; SELECT 2; INSERT INTO t VALUES (1)
```

**Output:** A `Vec` of three `Statement` values — two `Select` and one `Insert`.

### What Can Be Parsed

| Category | Examples |
| --- | --- |
| **SELECT** | `SELECT`, `JOIN`, `WHERE`, `GROUP BY`, `HAVING`, `ORDER BY`, `LIMIT`, `OFFSET`, `FETCH FIRST`, `DISTINCT`, `TOP`, `QUALIFY`, window functions |
| **CTEs** | `WITH ... AS (...)`, `WITH RECURSIVE ...` |
| **Set Operations** | `UNION`, `UNION ALL`, `INTERSECT`, `EXCEPT` |
| **DML** | `INSERT INTO ... VALUES`, `INSERT INTO ... SELECT`, `UPDATE ... SET`, `DELETE FROM` |
| **DDL** | `CREATE TABLE`, `CREATE TABLE ... AS SELECT`, `ALTER TABLE` (add/drop/rename column, add/drop constraint), `DROP TABLE`, `CREATE VIEW`, `DROP VIEW`, `TRUNCATE` |
| **Transaction** | `BEGIN`, `COMMIT`, `ROLLBACK`, `SAVEPOINT`, `RELEASE SAVEPOINT`, `ROLLBACK TO` |
| **Other** | `EXPLAIN [ANALYZE]`, `USE database` |
| **Expressions** | Binary/unary ops, `BETWEEN`, `IN`, `ANY`/`ALL`/`SOME`, `LIKE`, `ILIKE`, `CASE`, `CAST`, `TRY_CAST`, `EXTRACT`, `EXISTS`, `COALESCE`, `IF`, `NULLIF`, `INTERVAL`, window functions, subqueries, array literals, JSON access (`->`, `->>`), parameters (`$1`, `?`, `:name`), lambdas |

---

## Generating SQL

### Compact Output

`generate` produces a single-line SQL string from an AST:

```rust
use sqlglot_rust::{parse, generate, Dialect};

let ast = parse("SELECT a FROM t WHERE a > 1", Dialect::Ansi).unwrap();
let sql = generate(&ast, Dialect::Ansi);
assert_eq!(sql, "SELECT a FROM t WHERE a > 1");
```

**Input (AST):** The parsed tree of `SELECT a FROM t WHERE a > 1`

**Output (String):**

```sql
SELECT a FROM t WHERE a > 1
```

### Pretty Printing

`generate_pretty` adds line breaks and indentation:

```rust
use sqlglot_rust::{parse, generate_pretty, Dialect};

let ast = parse(
    "SELECT u.id, u.name FROM users AS u INNER JOIN orders AS o ON u.id = o.user_id WHERE o.total > 100 ORDER BY u.name LIMIT 10",
    Dialect::Ansi,
).unwrap();
let pretty = generate_pretty(&ast, Dialect::Ansi);
println!("{pretty}");
```

**Input:**

```sql
SELECT u.id, u.name FROM users AS u INNER JOIN orders AS o ON u.id = o.user_id WHERE o.total > 100 ORDER BY u.name LIMIT 10
```

**Output:**

```sql
SELECT
  u.id,
  u.name
FROM users AS u
INNER JOIN orders AS o
  ON u.id = o.user_id
WHERE
  o.total > 100
ORDER BY
  u.name
LIMIT 10
```

#### Pretty-printing with CTEs

```rust
let ast = parse(
    "WITH active AS (SELECT * FROM users WHERE active = TRUE) SELECT a.name, COUNT(*) FROM active AS a INNER JOIN orders AS o ON a.id = o.user_id GROUP BY a.name HAVING COUNT(*) > 5 ORDER BY a.name",
    Dialect::Ansi,
).unwrap();
println!("{}", generate_pretty(&ast, Dialect::Ansi));
```

**Output:**

```sql
WITH active AS (
  SELECT
    *
  FROM users
  WHERE
    active = TRUE
)
SELECT
  a.name,
  COUNT(*)
FROM active AS a
INNER JOIN orders AS o
  ON a.id = o.user_id
GROUP BY
  a.name
HAVING
  COUNT(*) > 5
ORDER BY
  a.name
```

---

## Transpiling Between Dialects

`transpile` parses with one dialect and generates with another, applying all
dialect-specific transformations automatically.

### Single Statement Transpilation

```rust
use sqlglot_rust::{transpile, Dialect};

let result = transpile(
    "SELECT NOW()",
    Dialect::Postgres,   // read dialect
    Dialect::BigQuery,   // write dialect
).unwrap();
assert_eq!(result, "SELECT CURRENT_TIMESTAMP()");
```

**Input:**

| | |
| --- | --- |
| SQL | `SELECT NOW()` |
| Read dialect | PostgreSQL |
| Write dialect | BigQuery |

**Output:**

```sql
SELECT CURRENT_TIMESTAMP()
```

### Multi-Statement Transpilation

```rust
use sqlglot_rust::transpile_statements;
use sqlglot_rust::Dialect;

let results = transpile_statements(
    "SELECT SUBSTR(x, 1, 3); SELECT NOW()",
    Dialect::Mysql,
    Dialect::Postgres,
).unwrap();
```

**Input:**

```sql
SELECT SUBSTR(x, 1, 3); SELECT NOW()
```

**Output (`results`):**

| Index | SQL                         |
| ----- | --------------------------- |
| `[0]` | `SELECT SUBSTRING(x, 1, 3)` |
| `[1]` | `SELECT NOW()`              |

### Function Mapping Examples

Below are common function transformations. Each row shows a transpile call
with its input, source dialect, target dialect, and the resulting SQL.

Recognized functions are parsed into **Typed Function Expressions** (see
[TypedFunction Enum](reference.md#typedfunction-enum) in the reference) which
enables the generator to emit the correct syntax for each target dialect.

| Input SQL | From | To | Output SQL |
| --- | --- | --- | --- |
| `SELECT NOW()` | PostgreSQL | BigQuery | `SELECT CURRENT_TIMESTAMP()` |
| `SELECT NOW()` | PostgreSQL | T-SQL | `SELECT GETDATE()` |
| `SELECT NOW()` | PostgreSQL | Snowflake | `SELECT CURRENT_TIMESTAMP()` |
| `SELECT GETDATE()` | T-SQL | PostgreSQL | `SELECT NOW()` |
| `SELECT GETDATE()` | T-SQL | BigQuery | `SELECT CURRENT_TIMESTAMP()` |
| `SELECT SUBSTRING(x, 1, 3) FROM t` | PostgreSQL | MySQL | `SELECT SUBSTR(x, 1, 3) FROM t` |
| `SELECT SUBSTR(x, 1, 3) FROM t` | MySQL | PostgreSQL | `SELECT SUBSTRING(x, 1, 3) FROM t` |
| `SELECT LEN(name) FROM t` | BigQuery | PostgreSQL | `SELECT LENGTH(name) FROM t` |
| `SELECT LEN(x) FROM t` | BigQuery | MySQL | `SELECT LENGTH(x) FROM t` |
| `SELECT CEIL(x)` | ANSI | T-SQL | `SELECT CEILING(x)` |
| `SELECT POW(x, 2)` | MySQL | T-SQL | `SELECT POWER(x, 2)` |
| `SELECT DATE_TRUNC('month', d)` | PostgreSQL | T-SQL | `SELECT DATETRUNC(month, d)` |
| `SELECT ARRAY_AGG(x)` | PostgreSQL | DuckDB | `SELECT LIST(x)` |
| `SELECT HEX(x)` | MySQL | Presto | `SELECT TO_HEX(x)` |
| `SELECT IFNULL(a, b) FROM t` | MySQL | PostgreSQL | `SELECT COALESCE(a, b) FROM t` |
| `SELECT IFNULL(a, b) FROM t` | MySQL | T-SQL | `SELECT ISNULL(a, b) FROM t` |

### Typed Function Expressions

When the parser encounters a recognized function name (72+ functions across 8
categories), it creates `Expr::TypedFunction` instead of the generic
`Expr::Function`. This provides:

- **Dialect-specific SQL generation** — the generator emits the correct function
  name and argument order for each target dialect without string-based rewriting
- **Semantic argument types** — each function variant carries typed fields
  (e.g., `Substring { expr, start, length }`) rather than a generic argument list
- **Proper AST traversal** — `walk()` and `transform()` recurse into typed
  function children

Unrecognized functions fall back to the generic `Expr::Function` with
string-based name and argument list, preserving backward compatibility.

```rust
use sqlglot_rust::{parse, Dialect, ast::types::Expr};

let stmt = parse("SELECT SUBSTRING(name, 1, 3) FROM users", Dialect::Ansi).unwrap();

// The parser creates a TypedFunction variant:
// Expr::TypedFunction {
//     func: TypedFunction::Substring { expr: Column("name"), start: 1, length: Some(3) },
//     filter: None,
//     over: None,
// }
```

**Categories** (72 variants):
Date/Time (12), String (17), Aggregate (9), Array (6), JSON (4),
Window (8), Math (11), Conversion (5)

See the [TypedFunction Enum](reference.md#typedfunction-enum) in the API reference
for the complete variant list and dialect-specific generation rules.

### Data Type Mapping Examples

CAST expressions are rewritten to use the target dialect's preferred type names.

| Input SQL | From | To | Output SQL |
| --- | --- | --- | --- |
| `SELECT CAST(x AS TEXT) FROM t` | PostgreSQL | BigQuery | `SELECT CAST(x AS STRING) FROM t` |
| `SELECT CAST(x AS STRING) FROM t` | BigQuery | PostgreSQL | `SELECT CAST(x AS TEXT) FROM t` |
| `SELECT CAST(x AS STRING) FROM t` | BigQuery | MySQL | `SELECT CAST(x AS TEXT) FROM t` |
| `SELECT CAST(x AS INT) FROM t` | PostgreSQL | BigQuery | `SELECT CAST(x AS BIGINT) FROM t` |
| `SELECT CAST(x AS FLOAT) FROM t` | PostgreSQL | BigQuery | `SELECT CAST(x AS DOUBLE) FROM t` |
| `SELECT CAST(x AS BYTEA) FROM t` | PostgreSQL | MySQL | `SELECT CAST(x AS BLOB) FROM t` |

### ILIKE Rewriting

Dialects without native `ILIKE` support get a `LOWER()` rewrite:

```rust
// Preserved in dialects that support ILIKE natively
transpile("SELECT * FROM t WHERE name ILIKE '%test%'", Dialect::Postgres, Dialect::Postgres);
// => "SELECT * FROM t WHERE name ILIKE '%test%'"

transpile("SELECT * FROM t WHERE name ILIKE '%test%'", Dialect::Postgres, Dialect::DuckDb);
// => "SELECT * FROM t WHERE name ILIKE '%test%'"

transpile("SELECT * FROM t WHERE name ILIKE '%test%'", Dialect::Postgres, Dialect::Snowflake);
// => "SELECT * FROM t WHERE name ILIKE '%test%'"

// Rewritten for dialects without ILIKE
transpile("SELECT * FROM t WHERE name ILIKE '%test%'", Dialect::Postgres, Dialect::Mysql);
// => "SELECT * FROM t WHERE LOWER(name) LIKE LOWER('%test%')"

transpile("SELECT * FROM t WHERE name ILIKE '%test%'", Dialect::Postgres, Dialect::Sqlite);
// => "SELECT * FROM t WHERE LOWER(name) LIKE LOWER('%test%')"
```

### Identifier Quoting

Quoted identifiers are automatically converted to the target dialect's style:

```sql
-- PostgreSQL input:  SELECT "my column" FROM "my table"
-- MySQL output:      SELECT `my column` FROM `my table`
-- T-SQL output:      SELECT [my column] FROM [my table]
```

| Dialect Family | Quote Style | Example |
| --- | --- | --- |
| PostgreSQL, Snowflake, Oracle, ANSI | Double-quote | `"my_column"` |
| MySQL, BigQuery, Hive, Spark | Backtick | `` `my_column` `` |
| T-SQL, Fabric | Bracket | `[my_column]` |

### LIMIT / TOP / FETCH FIRST

Row-limit syntax is translated between dialect conventions:

| Input | From | To | Output |
| --- | --- | --- | --- |
| `SELECT * FROM t LIMIT 10` | ANSI | T-SQL | `SELECT TOP 10 * FROM t` |
| `SELECT * FROM t LIMIT 10` | ANSI | Oracle | `SELECT * FROM t FETCH FIRST 10 ROWS ONLY` |
| `SELECT TOP 10 * FROM t` | T-SQL | ANSI | `SELECT * FROM t LIMIT 10` |

---

## Working with the AST

### Matching Statement Types

After parsing, match on the `Statement` enum to handle each SQL command:

```rust
use sqlglot_rust::{parse, Dialect, Statement};

let stmt = parse("SELECT a, b FROM users", Dialect::Ansi).unwrap();

match &stmt {
    Statement::Select(s) => {
        println!("Columns: {}", s.columns.len());  // => 2
        println!("Has WHERE: {}", s.where_clause.is_some());  // => false
    }
    Statement::Insert(i) => { println!("Insert into: {}", i.table.name); }
    Statement::Update(u) => { println!("Update: {}", u.table.name); }
    Statement::Delete(d) => { println!("Delete from: {}", d.table.name); }
    Statement::CreateTable(c) => { println!("Create: {}", c.table.name); }
    Statement::DropTable(d)   => { println!("Drop: {}", d.table.name); }
    Statement::AlterTable(a)  => { println!("Alter: {}", a.table.name); }
    Statement::SetOperation(s) => { println!("Set op: {:?}", s.op); }
    Statement::Transaction(t) => { println!("Transaction: {:?}", t); }
    _ => {}
}
```

**Input:** `SELECT a, b FROM users`

**Output:**

```text
Columns: 2
Has WHERE: false
```

### Inspecting a SELECT

Access individual clauses of a `SelectStatement`:

```rust
use sqlglot_rust::{parse, Dialect, Statement};
use sqlglot_rust::ast::SelectItem;

let stmt = parse(
    "SELECT u.name, COUNT(*) FROM users AS u WHERE u.active = TRUE GROUP BY u.name ORDER BY u.name LIMIT 10",
    Dialect::Ansi,
).unwrap();

if let Statement::Select(s) = &stmt {
    // Columns
    println!("Select list has {} items", s.columns.len());  // => 2

    // FROM table
    if let Some(from) = &s.from {
        println!("From: {:?}", from.source);  // => Table { name: "users", alias: Some("u") }
    }

    // WHERE
    if let Some(w) = &s.where_clause {
        println!("Where: {}", w.sql());  // => u.active = TRUE
    }

    // GROUP BY
    println!("Group by: {} exprs", s.group_by.len());  // => 1

    // ORDER BY
    println!("Order by: {} items", s.order_by.len());  // => 1

    // LIMIT
    if let Some(l) = &s.limit {
        println!("Limit: {}", l.sql());  // => 10
    }
}
```

**Input:** `SELECT u.name, COUNT(*) FROM users AS u WHERE u.active = TRUE GROUP BY u.name ORDER BY u.name LIMIT 10`

**Output:**

```text
Select list has 2 items
From: Table(TableRef { name: "users", alias: Some("u"), ... })
Where: u.active = TRUE
Group by: 1 exprs
Order by: 1 items
Limit: 10
```

### Constructing Expressions

Build AST nodes programmatically:

```rust
use sqlglot_rust::Expr;
use sqlglot_rust::ast::BinaryOperator;

// Build: price * quantity
let expr = Expr::BinaryOp {
    left: Box::new(Expr::Column {
        name: "price".to_string(),
        table: None,
        quote_style: Default::default(),
        table_quote_style: Default::default(),
    }),
    op: BinaryOperator::Multiply,
    right: Box::new(Expr::Column {
        name: "quantity".to_string(),
        table: None,
        quote_style: Default::default(),
        table_quote_style: Default::default(),
    }),
};

println!("{}", expr.sql());
```

**Output:**

```text
price * quantity
```

---

## Traversal and Search

### Walking the Tree

`walk` visits every node in the expression tree depth-first. The visitor
callback returns `true` to recurse into children, `false` to skip them.

```rust
use sqlglot_rust::{parse, Dialect, Expr, Statement};
use sqlglot_rust::ast::SelectItem;

let stmt = parse("SELECT a + b * 2", Dialect::Ansi).unwrap();
if let Statement::Select(s) = &stmt {
    if let SelectItem::Expr { expr, .. } = &s.columns[0] {
        let mut col_names = Vec::new();
        expr.walk(&mut |e| {
            if let Expr::Column { name, .. } = e {
                col_names.push(name.clone());
            }
            true
        });
        println!("Columns found: {:?}", col_names);
    }
}
```

**Input:** `SELECT a + b * 2`

**Output:**

```text
Columns found: ["a", "b"]
```

### Finding Nodes

#### `find` — First match

Returns the first node matching a predicate, or `None`:

```rust
let found = expr.find(&|e| matches!(e, Expr::Number(_)));
```

**Input (expr from):** `SELECT a + 1`

**Output:**

```text
Some(Number("1"))
```

#### `find_all` — All matches

Returns every node matching a predicate:

```rust
let numbers = expr.find_all(&|e| matches!(e, Expr::Number(_)));
```

**Input (expr from):** `SELECT 1 + 2 + 3`

**Output:**

```text
[Number("1"), Number("2"), Number("3")]  // len() == 3
```

### Finding Tables and Columns

Free functions in the `ast` module collect common references:

```rust
use sqlglot_rust::{parse, Dialect};
use sqlglot_rust::ast::{find_columns, find_tables};

// --- find_tables ---
let stmt = parse(
    "SELECT * FROM t1 INNER JOIN t2 ON t1.id = t2.id",
    Dialect::Ansi,
).unwrap();
let tables = find_tables(&stmt);
println!("{:?}", tables.iter().map(|t| &t.name).collect::<Vec<_>>());
```

**Input:** `SELECT * FROM t1 INNER JOIN t2 ON t1.id = t2.id`

**Output:**

```text
["t1", "t2"]
```

```rust
// --- find_columns ---
use sqlglot_rust::{parse, Dialect, Expr, Statement};
use sqlglot_rust::ast::{find_columns, SelectItem};

let stmt = parse("SELECT a + b + c", Dialect::Ansi).unwrap();
if let Statement::Select(s) = &stmt {
    if let SelectItem::Expr { expr, .. } = &s.columns[0] {
        let cols = find_columns(expr);
        println!("Column count: {}", cols.len());
    }
}
```

**Input:** `SELECT a + b + c`

**Output:**

```text
Column count: 3
```

---

## Transforming the AST

`transform` creates a new expression tree by applying a function bottom-up to
every node. The original tree is consumed and each node passes through the
function, which can return it unchanged or provide a replacement.

### Doubling Numeric Literals

```rust
use sqlglot_rust::{parse, generate, Expr, Dialect, Statement};
use sqlglot_rust::ast::SelectItem;

let stmt = parse("SELECT 1 + 2", Dialect::Ansi).unwrap();

let new_stmt = if let Statement::Select(mut s) = stmt {
    if let SelectItem::Expr { expr, .. } = &mut s.columns[0] {
        *expr = expr.clone().transform(&|e| {
            if let Expr::Number(n) = &e {
                let val: f64 = n.parse().unwrap();
                Expr::Number(((val * 2.0) as i64).to_string())
            } else {
                e
            }
        });
    }
    Statement::Select(s)
} else {
    stmt
};

let sql = generate(&new_stmt, Dialect::Ansi);
println!("{sql}");
```

**Input:** `SELECT 1 + 2`

**Output:**

```text
SELECT 2 + 4
```

### Renaming Columns

```rust
let transformed = expr.transform(&|e| {
    if let Expr::Column { name, table, quote_style, table_quote_style } = &e {
        if name == "old_col" {
            return Expr::Column {
                name: "new_col".to_string(),
                table: table.clone(),
                quote_style: *quote_style,
                table_quote_style: *table_quote_style,
            };
        }
    }
    e
});
println!("{}", transformed.sql());
```

**Input (expr from):** `SELECT old_col + b`

**Output:**

```text
new_col + b
```

---

## Query Optimization

The optimizer applies safe, semantics-preserving rewrites to a `Statement`.

```rust
use sqlglot_rust::{parse, generate, Dialect};
use sqlglot_rust::optimizer::optimize;

let stmt = parse("SELECT 1 + 2 * 3", Dialect::Ansi).unwrap();
let optimized = optimize(stmt).unwrap();
let sql = generate(&optimized, Dialect::Ansi);
assert_eq!(sql, "SELECT 7");
```

### Constant Folding

Compile-time–evaluable expressions are reduced to their result:

| Input | Output |
| --- | --- |
| `SELECT 1 + 2` | `SELECT 3` |
| `SELECT 10 - 3` | `SELECT 7` |
| `SELECT 3 * 4` | `SELECT 12` |
| `SELECT 10 / 2` | `SELECT 5` |
| `SELECT 10 % 3` | `SELECT 1` |
| `SELECT 1 + 2 * 3` | `SELECT 7` |
| `SELECT 'hello' \|\| ' ' \|\| 'world'` | `SELECT 'hello world'` |
| `SELECT 1 < 2` | `SELECT TRUE` |
| `SELECT 1 > 2` | `SELECT FALSE` |
| `SELECT 1 = 1` | `SELECT TRUE` |
| `SELECT 1 <> 1` | `SELECT FALSE` |
| `SELECT 1 <= 1` | `SELECT TRUE` |
| `SELECT 2 <= 1` | `SELECT FALSE` |

### Boolean Simplification

Tautologies and contradictions are eliminated:

| Input | Output | Rule |
| --- | --- | --- |
| `SELECT TRUE AND x` | `SELECT x` | AND identity |
| `SELECT FALSE AND x` | `SELECT FALSE` | AND annihilation |
| `SELECT TRUE OR x` | `SELECT TRUE` | OR annihilation |
| `SELECT FALSE OR x` | `SELECT x` | OR identity |
| `SELECT NOT NOT x` | `SELECT x` | Double negation |
| `SELECT NOT TRUE` | `SELECT FALSE` | NOT constant |
| `SELECT NOT FALSE` | `SELECT TRUE` | NOT constant |
| `SELECT a FROM t WHERE TRUE` | `SELECT a FROM t` | WHERE TRUE removal |

### Subquery Unnesting

Correlated subqueries in WHERE clauses are rewritten into equivalent JOINs,
which most query engines execute more efficiently.

```rust
use sqlglot_rust::{parse, generate, Dialect};
use sqlglot_rust::optimizer::optimize;

let stmt = parse(
    "SELECT a.id FROM a WHERE EXISTS (SELECT 1 FROM b WHERE b.id = a.id)",
    Dialect::Ansi,
).unwrap();
let optimized = optimize(stmt).unwrap();
let sql = generate(&optimized, Dialect::Ansi);
// EXISTS is rewritten to INNER JOIN
assert!(sql.contains("INNER JOIN"));
```

| Input | Output | Rule |
| --- | --- | --- |
| `WHERE EXISTS (SELECT … WHERE b.id = a.id)` | `INNER JOIN (SELECT DISTINCT …) ON …` | Semi-join |
| `WHERE NOT EXISTS (SELECT … WHERE b.id = a.id)` | `LEFT JOIN … WHERE _u.col IS NULL` | Anti-join |
| `WHERE x IN (SELECT col FROM …)` | `INNER JOIN (SELECT DISTINCT …)` | Semi-join |
| `WHERE x NOT IN (SELECT col FROM …)` | `LEFT JOIN … WHERE _u.col IS NULL` | Anti-join |

The pass is conservative and leaves the query unchanged when:

- The subquery has no equality correlation predicates.
- The subquery uses non-equality correlations (e.g., `b.val < a.val`) that would require LATERAL/APPLY.
- The subquery is inside a function in the SELECT list (e.g., `COALESCE(...)`) rather than in a WHERE predicate.

You can also call the pass directly:

```rust
use sqlglot_rust::optimizer::unnest_subqueries::unnest_subqueries;

let unnested = unnest_subqueries(stmt);
```

### Qualify Columns

`qualify_columns` resolves column references against a schema,
adding table qualifiers to unqualified columns and expanding wildcards.

```rust
use sqlglot_rust::{parse, generate, Dialect};
use sqlglot_rust::optimizer::qualify_columns::qualify_columns;
use sqlglot_rust::schema::MappingSchema;

let schema = MappingSchema::new()
    .with_table(vec!["users"], vec!["id", "name", "email"])
    .with_table(vec!["orders"], vec!["id", "user_id", "amount"]);

let stmt = parse("SELECT * FROM users", Dialect::Ansi).unwrap();
let qualified = qualify_columns(stmt, &schema);
assert_eq!(
    generate(&qualified, Dialect::Ansi),
    "SELECT users.id, users.name, users.email FROM users"
);
```

| Transformation | Before | After |
| --- | --- | --- |
| Qualify column | `SELECT col FROM t` | `SELECT t.col FROM t` |
| Expand `*` | `SELECT * FROM t` | `SELECT t.id, t.name FROM t` |
| Expand `t.*` | `SELECT t.* FROM t` | `SELECT t.id, t.name FROM t` |
| Qualify WHERE | `WHERE col = 1` | `WHERE t.col = 1` |
| Qualify JOIN ON | `ON id = fk` | `ON a.id = b.fk` |
| CTE resolution | `WITH cte AS (...) SELECT col FROM cte` | `SELECT cte.col FROM cte` |
| Subquery in WHERE | `WHERE id IN (SELECT fk FROM t2)` | `WHERE t.id IN (SELECT t2.fk FROM t2)` |

Ambiguous columns (found in multiple sources) are left unqualified.
The pass resolves columns through CTEs, derived tables, and nested subqueries.

### Scope Analysis

Scope analysis builds a tree of `Scope` objects from a parsed SQL statement,
tracking sources, column references, and correlations at each level. It is the
foundation for qualify_columns, pushdown_predicates, annotate_types, and
column lineage.

```rust
use sqlglot_rust::{parse, Dialect, build_scope};
use sqlglot_rust::optimizer::scope_analysis::ScopeType;

let ast = parse(
    "WITH cte AS (SELECT id FROM t) \
     SELECT cte.id FROM cte WHERE EXISTS (SELECT 1 FROM s WHERE s.fk = cte.id)",
    Dialect::Ansi,
).unwrap();
let scope = build_scope(&ast);

// Root scope sees cte as a source
assert!(scope.sources.contains_key("cte"));
assert_eq!(scope.cte_scopes.len(), 1);
assert_eq!(scope.subquery_scopes.len(), 1);

// The EXISTS subquery is correlated — it references outer table cte
let sub = &scope.subquery_scopes[0];
assert!(sub.is_correlated);
assert!(sub.external_columns.iter().any(|c| c.table.as_deref() == Some("cte")));
```

| Concept | Description |
| --- | --- |
| **Root scope** | The outermost SELECT |
| **CTE scope** | Each `WITH name AS (...)` body |
| **Derived-table scope** | Each subquery in FROM |
| **Subquery scope** | Each scalar, EXISTS, or IN subquery |
| **Union scope** | Each branch of UNION / INTERSECT / EXCEPT |
| **Correlation** | A column in a child scope referencing an outer source |
| **`find_all_in_scope`** | Filter columns within a single scope (does not descend into children) |

You can walk the full scope tree:

```rust
scope.walk(&mut |s| {
    println!("{:?}: {} sources", s.scope_type, s.sources.len());
});
```

---

## Schema Management

The schema module provides a `Schema` trait and `MappingSchema` implementation
for registering table metadata and performing dialect-aware lookups. This is the
foundation for type annotation, column qualification, and lineage analysis.

### Creating a Schema

```rust
use sqlglot_rust::schema::{MappingSchema, Schema};
use sqlglot_rust::ast::DataType;
use sqlglot_rust::Dialect;

let mut schema = MappingSchema::new(Dialect::Postgres);

// Register a table with column definitions
schema.add_table(
    &["public", "users"],
    vec![
        ("id".to_string(), DataType::Int),
        ("name".to_string(), DataType::Varchar(Some(255))),
        ("email".to_string(), DataType::Text),
    ],
).unwrap();

// 3-level path: catalog.database.table
schema.add_table(
    &["prod", "analytics", "events"],
    vec![
        ("event_id".to_string(), DataType::BigInt),
        ("payload".to_string(), DataType::Json),
    ],
).unwrap();
```

### Querying the Schema

```rust
// Get column names in definition order
let cols = schema.column_names(&["public", "users"]).unwrap();
assert_eq!(cols, vec!["id", "name", "email"]);

// Look up a column's type
let dt = schema.get_column_type(&["public", "users"], "id").unwrap();
assert_eq!(dt, DataType::Int);

// Check column existence
assert!(schema.has_column(&["public", "users"], "email"));
assert!(!schema.has_column(&["public", "users"], "age"));

// Short-path lookup — searches all catalogs/databases
assert!(schema.has_column(&["users"], "id"));
```

### Dialect-Aware Normalization

Identifier lookups are normalized per the schema's dialect:

```rust
// Postgres: case-insensitive — these all match
let mut pg = MappingSchema::new(Dialect::Postgres);
pg.add_table(&["Users"], vec![("ID".to_string(), DataType::Int)]).unwrap();
assert!(pg.has_column(&["users"], "id"));   // matches
assert!(pg.has_column(&["USERS"], "ID"));   // matches

// BigQuery: case-sensitive — exact match required
let mut bq = MappingSchema::new(Dialect::BigQuery);
bq.add_table(&["Users"], vec![("ID".to_string(), DataType::Int)]).unwrap();
assert!(bq.has_column(&["Users"], "ID"));    // matches
assert!(!bq.has_column(&["users"], "id"));   // does not match
```

### Building Schemas from Maps

The `ensure_schema` helper builds a `MappingSchema` from nested `HashMap`s:

```rust
use std::collections::HashMap;
use sqlglot_rust::schema::ensure_schema;
use sqlglot_rust::ast::DataType;
use sqlglot_rust::Dialect;

let mut tables = HashMap::new();
let mut cols = HashMap::new();
cols.insert("id".to_string(), DataType::Int);
cols.insert("name".to_string(), DataType::Text);
tables.insert("users".to_string(), cols);

let schema = ensure_schema(tables, Dialect::Postgres);
assert!(schema.has_column(&["users"], "id"));
```

---

## Serialization (JSON Round-Tripping)

All AST types derive `serde::Serialize` and `serde::Deserialize`, so you can
convert any `Statement` to JSON and back:

```rust
use sqlglot_rust::{parse, generate, Dialect, Statement};

// Parse
let ast = parse("SELECT id, name FROM users WHERE active = TRUE", Dialect::Ansi).unwrap();

// Serialize to JSON
let json = serde_json::to_string_pretty(&ast).unwrap();
println!("{json}");

// Deserialize back
let restored: Statement = serde_json::from_str(&json).unwrap();
let sql = generate(&restored, Dialect::Ansi);
assert_eq!(sql, "SELECT id, name FROM users WHERE active = TRUE");
```

**Input:** `SELECT id, name FROM users WHERE active = TRUE`

**Output (JSON, abbreviated):**

```json
{
  "Select": {
    "ctes": [],
    "distinct": false,
    "top": null,
    "columns": [
      { "Expr": { "expr": { "Column": { "name": "id", "table": null } }, "alias": null } },
      { "Expr": { "expr": { "Column": { "name": "name", "table": null } }, "alias": null } }
    ],
    "from": { "source": { "Table": { "name": "users" } } },
    "where_clause": { "BinaryOp": { "left": { "Column": { "name": "active" } }, "op": "Eq", "right": { "Boolean": true } } },
    ...
  }
}
```

**Use cases:**

- Store parsed ASTs in databases or caches
- Send ASTs across service boundaries (microservices, WASM)
- Build language-server-style tooling
- Debugging — inspect the exact parse tree

---

## Error Handling

All fallible operations return `Result<T, SqlglotError>`. Errors implement
`Display` and `std::error::Error` (via `thiserror`), making them compatible
with `anyhow`, `eyre`, and the `?` operator.

```rust
use sqlglot_rust::{parse, Dialect, SqlglotError};

match parse("SELECT FROM", Dialect::Ansi) {
    Ok(stmt) => println!("Parsed: {:?}", stmt),
    Err(e)   => eprintln!("Error: {e}"),
}
```

**Input:** `SELECT FROM` (missing select list)

**Output:**

```text
Error: Parser error: ...
```

### Matching Error Variants

```rust
use sqlglot_rust::{parse, Dialect, SqlglotError};

let err = parse("SELECT @@@", Dialect::Ansi).unwrap_err();

match &err {
    SqlglotError::TokenizerError { message, position } => {
        println!("Tokenizer error at byte {position}: {message}");
    }
    SqlglotError::ParserError { message } => {
        println!("Parse error: {message}");
    }
    SqlglotError::UnexpectedToken { token } => {
        println!("Unexpected token: {:?}", token);
    }
    SqlglotError::UnsupportedDialectFeature(msg) => {
        println!("Unsupported: {msg}");
    }
    SqlglotError::Internal(msg) => {
        println!("Internal error: {msg}");
    }
}
```

### Using with `anyhow`

```rust
use anyhow::Result;
use sqlglot_rust::{parse, generate, Dialect};

fn process_query(sql: &str) -> Result<String> {
    let ast = parse(sql, Dialect::Ansi)?;    // SqlglotError → anyhow::Error
    Ok(generate(&ast, Dialect::Ansi))
}
```

---

## SBOM Generation

The project supports generating a Software Bill of Materials (SBOM) in
[SPDX](https://spdx.dev/) 2.3 JSON format. This documents every dependency
shipped with the library — useful for license compliance, supply-chain
security audits, and vulnerability tracking.

### Prerequisites

Install [`cargo-sbom`](https://crates.io/crates/cargo-sbom) (one-time setup):

```bash
cargo install cargo-sbom
```

> `cargo-sbom` is a standalone CLI tool. It is **not** a project dependency and
> does not appear in the generated SBOM.

### Generating the SBOM

Use the provided Makefile target:

```bash
make sbom
```

Or run the command directly:

```bash
cargo sbom --output-format spdx_json_2_3 > target/sbom/sqlglot-rust.spdx.json
```

**Output location:** `target/sbom/sqlglot-rust.spdx.json`

### What the SBOM Contains

The generated SPDX document includes:

| Field | Description |
| --- | --- |
| Package name and version | For every direct and transitive dependency |
| License (concluded) | SPDX license expression per package |
| Download location | Registry URL for the crate |
| Package URL (PURL) | `pkg:cargo/<name>@<version>` identifier |
| File checksums | SHA-1 hash of `Cargo.lock` |
| Creation info | Timestamp and tool version |

### Example: Inspecting the SBOM

List all packages:

```bash
jq -r '.packages[].name' target/sbom/sqlglot-rust.spdx.json
```

Count dependencies:

```bash
jq '.packages | length' target/sbom/sqlglot-rust.spdx.json
```

Extract licenses:

```bash
jq -r '.packages[] | "\(.name): \(.licenseConcluded)"' target/sbom/sqlglot-rust.spdx.json
```

---

## Updating the Version

The project provides a Makefile target to update the version consistently across
all configuration and documentation files:

```bash
make bump-version VERSION=1.0.0
```

### What It Updates

| File | Field |
| --- | --- |
| `Cargo.toml` | `version = "..."` |
| `README.md` | `sqlglot-rust = "..."` dependency snippet |
| `docs/installation.md` | `sqlglot-rust = "..."` dependency snippet |
| `Cargo.lock` | Regenerated via `cargo generate-lockfile` |

### Guidelines

- Always use a **full semantic version**: `MAJOR.MINOR.PATCH` (e.g. `1.0.0`, `0.10.1`).
- Run `make bump-version` **before** committing a release so all references stay in sync.
- The `VERSION` parameter is required — omitting it produces an error with usage instructions.

### Example

```bash
$ make bump-version VERSION=1.2.0
Bumping version to 1.2.0...
Version updated to 1.2.0
```

---

## Next Steps

- **[Reference](reference.md)** — Complete API tables, type catalogs, all 30
  dialects, `Expr` variants, `DataType` variants, and operator enums.
- **[Installation](installation.md)** — Dependency setup and verification.
