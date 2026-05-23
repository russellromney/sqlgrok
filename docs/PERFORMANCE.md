# Performance

sqlgrok should eventually justify the Rust port with both parity and speed. The
benchmark lane compares sqlgrok's in-process Rust transpiler with Python SQLGlot
on parity-clean MySQL/Postgres-to-SQLite cases.

Run it with a release build:

```bash
cargo run --release --bin xtask -- bench-sqlglot \
  --sqlglot /Users/russellromney/Documents/Github/sqlglot \
  --iterations 2000 \
  --warmup 100 \
  --output benchmarks/sqlglot_comparison.md
```

Use `--dry-run` to print the report instead of writing it.

The command validates every workload case against Python SQLGlot before timing.
If a case no longer matches, the benchmark fails instead of timing different
behavior. That keeps performance numbers tied to the same contract as the
parity harness.

## What It Measures

- Python SQLGlot: repeated `sqlglot.transpile(sql, read=..., write=...)` calls
  inside one Python process, after warmup.
- sqlgrok: repeated in-process `transpile(sql, read, write)` calls in the xtask
  binary, after warmup.
- Workload: fixed MySQL/Postgres-to-SQLite queries covering aggregates, JSON,
  limit/offset, date/time, expression rewrites, `DISTINCT ON`, grouping, and
  window ordering.

## Reading Results

The report includes total time, microseconds per operation, a speedup ratio, and
the workload SQL. Treat one run as a directional snapshot, not a stable product
claim. For decisions, run the command several times on a quiet machine and look
for consistent order-of-magnitude differences.

If sqlgrok is not materially faster, the next step is profiling the Rust path
rather than assuming the port is doomed. Useful targets include parser
allocation, tokenizer cloning, transform recursion, and generator string growth.
