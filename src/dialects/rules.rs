//! Representation-neutral, target-keyed dialect rules.
//!
//! These tables are the shared vocabulary of the parse→generate pipeline:
//! they take a `target` dialect and a canonical (uppercased) name and return a
//! `&'static str` replacement. Because the result is a static string and the
//! lookup happens at generate-time, applying a rule is a zero-allocation
//! `push_str` in both the owned generator and the borrowed fast-path
//! generator. They are also the exact shape of SQLGlot's per-dialect dicts, so
//! they can be transcribed/generated from upstream rather than rediscovered.
//!
//! Invariant: every entry here is *target-determined only* — it must not
//! depend on the source dialect. Source-dependent behaviour belongs in the
//! parser (read-side canonicalization), not here. See `docs/PORTING_PLAN.md`.

use crate::dialects::Dialect;

/// Map a function name to its target-dialect spelling. `upper_name` must
/// already be uppercased by the caller. Returns `None` when no rename applies
/// (the caller keeps the original name).
pub(crate) fn rename_function(target: Dialect, upper_name: &str) -> Option<&'static str> {
    match target {
        Dialect::Sqlite => rename_function_sqlite(upper_name),
        _ => None,
    }
}

/// Map a canonical (uppercased) type name to its target-dialect spelling.
/// Covers only pure name→name mappings; parametric forms like `INT(10)` or
/// `VARCHAR(MAX)` (which preserve a `(...)` suffix) are handled by the caller.
/// Returns `None` when no rename applies.
pub(crate) fn map_type(target: Dialect, upper_name: &str) -> Option<&'static str> {
    match target {
        Dialect::Sqlite => map_type_sqlite(upper_name),
        _ => None,
    }
}

fn map_type_sqlite(upper_name: &str) -> Option<&'static str> {
    let mapped = match upper_name {
        "NUMBER" | "FLOAT4" => "REAL",
        "INT4" | "INT1" | "MEDIUMINT" | "INT8" | "INT16" | "INT32" | "INT64" => "INTEGER",
        "HUGEINT" => "INT128",
        "UHUGEINT" => "UINT128",
        "INT128" => "INT128",
        "INT256" => "INT256",
        "UINT128" => "UINT128",
        "UINT256" => "UINT256",
        "BIGNUMERIC" => "BIGDECIMAL",
        "TIMESTAMP_NTZ" => "TIMESTAMPNTZ",
        "TIMESTAMP_LTZ" => "TIMESTAMPLTZ",
        "TIMESTAMP_TZ" => "TIMESTAMPTZ",
        _ => return None,
    };
    Some(mapped)
}

fn rename_function_sqlite(upper_name: &str) -> Option<&'static str> {
    let rename = match upper_name {
        "ARRAY_JOIN" => "ARRAY_TO_STRING",
        "ARRAY_INTERSECTION" => "ARRAY_INTERSECT",
        "STARTSWITH" => "STARTS_WITH",
        "YEAROFWEEK" => "YEAR_OF_WEEK",
        "YEAROFWEEKISO" => "YEAR_OF_WEEK_ISO",
        "LEVENSHTEIN" => "EDITDIST3",
        "ARRAY_FILTER" => "FILTER",
        "FARMFINGERPRINT64" => "FARM_FINGERPRINT",
        "LEFTPAD" => "LPAD",
        "RIGHTPAD" => "RPAD",
        "GET_BIT" => "GETBIT",
        "SIGNUM" => "SIGN",
        "STDEV" => "STDDEV",
        "ST_MAKEPOINT" => "ST_POINT",
        "ARGMAX" => "ARG_MAX",
        "ARGMIN" => "ARG_MIN",
        "APPROX_COUNT_DISTINCT" => "APPROX_DISTINCT",
        "STRING_AGG" => "GROUP_CONCAT",
        "STRPOS" => "INSTR",
        "BOOL_AND" | "LOGICAL_AND" => "MIN",
        "BOOL_OR" | "LOGICAL_OR" => "MAX",
        _ => return None,
    };
    Some(rename)
}
