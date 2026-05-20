# sqlgrok Architecture Notes

sqlgrok has two separate parser goals that must not get blurred:

- Behavioral contract: match Python SQLGlot wherever possible.
- Implementation quality: build a fast, maintainable Rust parser and transpiler.

Python SQLGlot remains the source of truth for AST parity, dialect behavior, fixture expectations, and compatibility decisions. Other parser projects can influence internals, but they do not replace the SQLGlot parity contract.

## Parser Inspiration

The Databend engineering article [RisingWave Query Parser](https://www.databend.com/blog/category-engineering/2025-09-10-query-parser/) is a useful design reference for Rust parser internals.

Ideas worth borrowing:

- Keep parsing syntax-focused and defer semantic analysis to later passes.
- Track source spans on tokens and AST nodes so diagnostics can point to exact input ranges.
- Track the furthest parse error to report the most useful failure instead of the last incidental one.
- Use precedence-driven expression parsing so operator behavior is explicit and testable.
- Consider zero-copy token and AST representation where it does not fight ownership clarity.

Boundaries:

- Do not adopt another project's AST as the sqlgrok AST contract.
- Do not optimize for zero-copy representation before parity gaps are measurable.
- Do not let parser ergonomics force divergences from Python SQLGlot behavior.
- Do not mix semantic validation into parsing unless Python SQLGlot does so for the same case.

## AST Contract

The long-term AST model should be shaped by Python SQLGlot expression coverage. Before broad AST expansion, create `docs/AST_INVENTORY.md` with:

- Python SQLGlot expression name.
- Current sqlgrok representation.
- Status: `supported`, `partial`, `unsupported`, or `out-of-scope`.
- Parser support.
- Generator support.
- Known fixture blockers.

The inventory should drive AST work in small, reviewable batches.

## Error Reporting Direction

Parser errors should eventually include:

- input span or byte offset;
- expected token or grammar shape;
- actual token;
- dialect;
- source SQL excerpt.

The first implementation step should be furthest-error tracking, because it improves developer experience without requiring a full AST redesign.

## Performance Direction

Zero-copy parsing is attractive, but it should come after correctness scaffolding:

1. parity fixtures and filters;
2. parser and generator regression tests;
3. AST inventory;
4. span-aware parsing;
5. selective zero-copy refactors where profiling shows value.

This order keeps optimization from hiding correctness gaps.
