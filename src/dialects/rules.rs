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
