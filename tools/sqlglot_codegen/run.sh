#!/usr/bin/env bash
# Convenience wrapper for the SQLGlot codegen inventory extractor.
#
# Usage:
#   tools/sqlglot_codegen/run.sh [SQLGLOT_PATH] [OUT_DIR]
#
# Defaults match the local development checkout and regenerate the checked-in
# priority sample for postgres/mysql/sqlite.
set -euo pipefail

SQLGLOT="${1:-/Users/russellromney/Documents/Github/sqlglot}"
OUT="${2:-generated/sqlglot_inventory}"

exec uv run --python 3.10 tools/sqlglot_codegen/extract.py \
  --sqlglot "$SQLGLOT" \
  --out "$OUT" \
  --dialects postgres,mysql,sqlite
