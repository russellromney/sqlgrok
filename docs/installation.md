# Installation

Getting started with **sqlglot-rust** — adding the crate and verifying your setup.

> **See also:** [Developer Guide](developer-guide.md) · [Reference](reference.md)

---

## Requirements

- **Rust** 2024 edition (1.85+)
- **Cargo** (ships with Rust)

No C compiler or system libraries are needed — the crate is pure Rust.

---

## Add the Dependency

Add `sqlglot-rust` to your project's `Cargo.toml`:

```toml
[dependencies]
sqlglot-rust = "0.9.3"
```

Then run:

```bash
cargo build
```

### Transitive Dependencies

The following crates are pulled in automatically — you do **not** need to add
them yourself:

| Crate | Purpose |
| --- | --- |
| `serde` (with `derive`) | AST serialization / deserialization |
| `serde_json` | JSON support for AST round-tripping |
| `thiserror` | Ergonomic error types |
| `log` | Optional structured logging |

---

## Verify the Installation

Create a small program to confirm everything works.

### Input — `src/main.rs`

```rust
use sqlglot_rust::{parse, generate, transpile, Dialect};

fn main() {
    // 1. Parse a SQL string into an AST
    let ast = parse(
        "SELECT a, b FROM users WHERE age > 21",
        Dialect::Ansi,
    )
    .expect("parse failed");

    // 2. Generate SQL back from the AST
    let sql = generate(&ast, Dialect::Ansi);
    println!("Roundtrip:  {sql}");

    // 3. Transpile from PostgreSQL to MySQL
    let mysql = transpile(
        "SELECT SUBSTRING(name, 1, 3) FROM users",
        Dialect::Postgres,
        Dialect::Mysql,
    )
    .expect("transpile failed");
    println!("Transpiled: {mysql}");
}
```

### Expected Output

```text
Roundtrip:  SELECT a, b FROM users WHERE age > 21
Transpiled: SELECT SUBSTR(name, 1, 3) FROM users
```

If you see this, the library is installed and ready.

---

## Using as a Library Dependency

sqlglot-rust is a **library crate** — it exposes modules you import in your own
code. The public entry points are re-exported from the crate root for
convenience:

```rust
// Commonly used imports (all re-exported from the crate root)
use sqlglot_rust::{parse, generate, generate_pretty, transpile, Dialect};
use sqlglot_rust::{Expr, Statement, QuoteStyle};
use sqlglot_rust::SqlglotError;

// For sub-module access
use sqlglot_rust::parser::parse_statements;
use sqlglot_rust::optimizer::optimize;
use sqlglot_rust::ast::{find_columns, find_tables, SelectItem};
```

---

## Next Steps

- **[Developer Guide](developer-guide.md)** — Parsing, generating, transpiling,
  working with the AST, optimization, and serialization with full code examples.
- **[Reference](reference.md)** — Complete API surface, type catalog, dialect
  tables, and error variants.
