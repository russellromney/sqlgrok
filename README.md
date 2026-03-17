# sqlglot-rust

A SQL parser, optimizer, and transpiler library written in Rust, inspired by Python's [sqlglot](https://github.com/tobymao/sqlglot).

## Features

- **Parse** SQL strings into a structured AST
- **Generate** SQL from AST nodes
- **Transpile** between 30 SQL dialects
- **LIMIT / TOP / FETCH FIRST** transpilation across dialects
- **Quoted identifier** preservation and cross-dialect conversion (`"id"` ↔ `` `id` `` ↔ `[id]`)
- **Optimize** SQL queries (constant folding, boolean simplification, subquery unnesting)
- **Schema management** — dialect-aware table/column registration and type lookup
- **AST traversal** — walk, find, transform expressions
- CTEs, subqueries, set operations (UNION / INTERSECT / EXCEPT)
- Window functions with frames and filters
- CAST, TRY_CAST, EXTRACT, INTERVAL, EXISTS, ANY/ALL/SOME
- Full DDL support (CREATE TABLE, ALTER TABLE, DROP TABLE, CREATE VIEW, etc.)
- Serde serialization for AST nodes

## Supported Dialects

All 30 dialects from Python's sqlglot are supported, with function name mapping,
data type mapping, and ILIKE rewriting between dialects.

### Official Dialects

| Dialect | Description |
| --- | --- |
| ANSI SQL | Base SQL standard |
| Athena | AWS Athena (Presto-based) |
| BigQuery | Google BigQuery |
| ClickHouse | ClickHouse |
| Databricks | Databricks (Spark-based) |
| DuckDB | DuckDB |
| Hive | Apache Hive |
| MySQL | MySQL |
| Oracle | Oracle Database |
| PostgreSQL | PostgreSQL |
| Presto | Presto |
| Redshift | Amazon Redshift (Postgres-based) |
| Snowflake | Snowflake |
| Spark | Apache Spark SQL |
| SQLite | SQLite |
| StarRocks | StarRocks (MySQL-compatible) |
| Trino | Trino (Presto successor) |
| T-SQL | Microsoft SQL Server |

### Community Dialects

| Dialect | Description |
| --- | --- |
| Doris | Apache Doris (MySQL-compatible) |
| Dremio | Dremio |
| Drill | Apache Drill |
| Druid | Apache Druid |
| Exasol | Exasol |
| Fabric | Microsoft Fabric (T-SQL variant) |
| Materialize | Materialize (Postgres-compatible) |
| PRQL | Pipelined Relational Query Language |
| RisingWave | RisingWave (Postgres-compatible) |
| SingleStore | SingleStore (MySQL-compatible) |
| Tableau | Tableau |
| Teradata | Teradata |

### Dialect Transform Rules

The transpiler applies dialect-specific rewrite rules when converting between dialects:

| Rule | Example |
| --- | --- |
| Function name mapping | `NOW()` → `CURRENT_TIMESTAMP()`, `GETDATE()` |
| `SUBSTR` ↔ `SUBSTRING` | Postgres uses `SUBSTRING`, MySQL uses `SUBSTR` |
| `IFNULL` → `COALESCE` | MySQL `IFNULL` → ANSI `COALESCE` |
| `IFNULL` → `ISNULL` | MySQL `IFNULL` → T-SQL `ISNULL` |
| `NVL` → `COALESCE` | Oracle `NVL` → standard `COALESCE` |
| `LEN` ↔ `LENGTH` | T-SQL `LEN` ↔ standard `LENGTH` |
| `RAND` ↔ `RANDOM` | MySQL `RAND` ↔ Postgres `RANDOM` |
| `ILIKE` → `LOWER`/`LIKE` | Rewritten for dialects without native ILIKE |
| `LIMIT` ↔ `TOP` | `LIMIT 10` → `TOP 10` for T-SQL |
| `LIMIT` → `FETCH FIRST` | `LIMIT 10` → `FETCH FIRST 10 ROWS ONLY` (Oracle) |
| Quoted identifiers | `"id"` ↔ `` `id` `` ↔ `[id]` per dialect |
| Data type mapping | `TEXT` ↔ `STRING`, `INT` → `BIGINT` (BigQuery) |
| `BYTEA` ↔ `BLOB` | Postgres `BYTEA` ↔ MySQL `BLOB` |

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
sqlglot-rust = "0.9.5"
```

### Parse and generate SQL

```rust
use sqlglot_rust::{parse, generate, Dialect};

fn main() {
    // Parse a SQL query
    let ast = parse("SELECT a, b FROM t WHERE a > 1", Dialect::Ansi).unwrap();

    // Generate SQL (roundtrip)
    let sql = generate(&ast, Dialect::Ansi);
    assert_eq!(sql, "SELECT a, b FROM t WHERE a > 1");
}
```

### Transpile between dialects

```rust
use sqlglot_rust::{transpile, Dialect};

fn main() {
    // Postgres → BigQuery: NOW() becomes CURRENT_TIMESTAMP()
    let sql = transpile(
        "SELECT NOW(), SUBSTRING(name, 1, 3) FROM users",
        Dialect::Postgres,
        Dialect::BigQuery,
    ).unwrap();
    // → "SELECT CURRENT_TIMESTAMP(), SUBSTRING(name, 1, 3) FROM users"

    // Oracle → T-SQL: NVL becomes ISNULL
    let sql = transpile(
        "SELECT NVL(a, b) FROM t",
        Dialect::Oracle,
        Dialect::Tsql,
    ).unwrap();
    // → "SELECT ISNULL(a, b) FROM t"
}
```

### Parse a dialect name from a string

```rust
use sqlglot_rust::Dialect;

let d = Dialect::from_str("postgres").unwrap();
assert_eq!(d, Dialect::Postgres);

// Multiple aliases are supported
assert_eq!(Dialect::from_str("tsql"), Some(Dialect::Tsql));
assert_eq!(Dialect::from_str("mssql"), Some(Dialect::Tsql));
assert_eq!(Dialect::from_str("sqlserver"), Some(Dialect::Tsql));
```

### Supported Statements

- `SELECT` (with JOINs, WHERE, GROUP BY, HAVING, ORDER BY, LIMIT, OFFSET, TOP, FETCH FIRST, QUALIFY)
- `INSERT INTO ... VALUES` / `INSERT INTO ... SELECT`
- `UPDATE ... SET ... WHERE` (with RETURNING)
- `DELETE FROM ... WHERE` (with USING)
- `CREATE TABLE` (with constraints, IF NOT EXISTS, TEMPORARY, AS SELECT)
- `DROP TABLE` (with IF EXISTS)
- `ALTER TABLE` (ADD COLUMN, DROP COLUMN, RENAME COLUMN, RENAME TO, ALTER TYPE)
- `CREATE VIEW` / `DROP VIEW`
- `TRUNCATE TABLE`
- `BEGIN` / `COMMIT` / `ROLLBACK`
- `EXPLAIN`
- `USE`

### Expression Support

- Column references (qualified and unqualified) with quote-style metadata
- Numeric, string, boolean, and NULL literals
- Binary operators (`+`, `-`, `*`, `/`, `%`, `=`, `<>`, `<`, `>`, `<=`, `>=`, `AND`, `OR`, `||`)
- Unary operators (`NOT`, `-`, `+`)
- Bitwise operators (`&`, `|`, `^`, `<<`, `>>`)
- Function calls (with DISTINCT, FILTER, OVER support)
- `BETWEEN`, `IN`, `IS NULL`, `LIKE`, `ILIKE`
- `CASE ... WHEN ... THEN ... ELSE ... END`
- `CAST`, `TRY_CAST`, PostgreSQL `::` cast
- `EXTRACT(field FROM expr)` for all date/time fields
- `INTERVAL` expressions
- `EXISTS`, `IN (subquery)`
- Array literals (`ARRAY[1, 2, 3]`)
- Window functions with frames (`ROWS`, `RANGE`, `GROUPS`)
- Common Table Expressions (WITH / WITH RECURSIVE)
- Set operations (UNION, INTERSECT, EXCEPT with ALL)
- Parenthesized sub-expressions and subqueries

## Documentation

- **[Installation](docs/installation.md)** — Dependency setup and verification
- **[Developer Guide](docs/developer-guide.md)** — Parsing, generating, transpiling, AST traversal, optimization, and serialization with full code examples
- **[API Reference](docs/reference.md)** — Complete type catalog, function signatures, dialect tables, and error variants

## Architecture

```text
src/
├── ast/          # AST node definitions (~40 expression types, 15 statement types)
├── tokens/       # Token types (~200+ variants) and tokenizer
├── parser/       # Recursive-descent SQL parser
├── generator/    # SQL code generator
├── dialects/     # 30 dialect definitions with transform rules
├── optimizer/    # Query optimization (constant folding, boolean simplification, subquery unnesting)
├── schema/       # Schema management (MappingSchema, dialect-aware lookups)
├── errors/       # Error types
└── lib.rs        # Public API (parse, generate, transpile)
```

## Development

```bash
# Build
cargo build

# Run tests (244+ tests)
cargo test

# Run benchmarks
cargo bench

# Lint
cargo clippy

# Format
cargo fmt
```

A `Makefile` is provided for convenience:

```bash
make build          # cargo build
make test           # cargo test
make lint           # cargo clippy
make fmt            # cargo fmt
make sbom           # Generate SPDX SBOM (see below)
make bump-version   # Update version across all files (see below)
make all            # build + sbom
```

## SBOM (Software Bill of Materials)

The project supports generating an SBOM in [SPDX](https://spdx.dev/) 2.3 JSON format using [`cargo-sbom`](https://crates.io/crates/cargo-sbom).

### Prerequisites

Install `cargo-sbom`:

```bash
cargo install cargo-sbom
```

### Generate the SBOM

```bash
make sbom
```

This writes the SBOM to `target/sbom/sqlglot-rust.spdx.json`. You can also run the command directly:

```bash
cargo sbom --output-format spdx_json_2_3 > target/sbom/sqlglot-rust.spdx.json
```

The generated SBOM includes all dependency packages with license information, download locations, and [Package URLs (PURLs)](https://github.com/package-url/purl-spec).

## Updating the Version

Use the `bump-version` Makefile target to update the version consistently across
all configuration and documentation files:

```bash
make bump-version VERSION=1.0.0
```

This updates:

- `Cargo.toml` — package version
- `README.md` — dependency snippet
- `docs/installation.md` — dependency snippet
- `Cargo.lock` — regenerated automatically

The `VERSION` parameter is required and must be a full semantic version (e.g. `1.0.0`, `0.10.1`).

## Acknowledgements

This project is inspired by and derived from [sqlglot](https://github.com/tobymao/sqlglot) by Toby Mao, licensed under the MIT License. The original sqlglot license is included in this repository as required by the MIT License terms. A copy of the original license can also be found [in the polyglot repository](https://github.com/tobilg/polyglot/tree/main/licenses).

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

This project includes code and concepts derived from [sqlglot](https://github.com/tobymao/sqlglot), which is also licensed under the MIT License. The original copyright notice and permission notice are reproduced below:

```text
MIT License

Copyright (c) 2023 Toby Mao

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
