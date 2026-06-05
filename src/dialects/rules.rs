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

/// Read-side: canonicalize a *source-dialect* function spelling into the
/// neutral name the AST carries, so the generator alone (via `rename_function`)
/// can render it for any target on every path (incl. identity). The parser
/// consults this when building a function node. `upper_name` must be uppercased.
///
/// Symmetric mirror of `rename_function`: read normalizes IN, write renders OUT.
/// Together they replace the old `if source==X && target==Y` transform rules.
pub(crate) fn canonicalize_function(source: Dialect, upper_name: &str) -> Option<&'static str> {
    // Source-independent canonicalizations: normalize to one neutral name for
    // every source (and render as that name in every target).
    match upper_name {
        // IFNULL is COALESCE in every dialect, including oracle (verified
        // across the full read x write matrix against SQLGlot). NVL is NOT
        // here: oracle renders it back to NVL, which a string canonical can't
        // distinguish from a true COALESCE — it stays a transform-layer rule.
        "IFNULL" => return Some("COALESCE"),
        _ => {}
    }
    match source {
        Dialect::Mysql => match upper_name {
            "BIT_AND" => Some("BITWISE_AND_AGG"),
            "BIT_OR" => Some("BITWISE_OR_AGG"),
            "BIT_XOR" => Some("BITWISE_XOR_AGG"),
            "BIT_COUNT" => Some("BITWISE_COUNT"),
            _ => None,
        },
        Dialect::Postgres => match upper_name {
            "BIT_AND" => Some("BITWISE_AND_AGG"),
            "BIT_OR" => Some("BITWISE_OR_AGG"),
            "BIT_XOR" => Some("BITWISE_XOR_AGG"),
            // Postgres BIT_COUNT is its own canonical (renders BIT_COUNT
            // everywhere); only MySQL's BIT_COUNT canonicalizes.
            _ => None,
        },
        _ => None,
    }
}

/// Map a (canonical, uppercased) function name to its target-dialect spelling.
/// Returns `None` when no rename applies (caller keeps the name).
pub(crate) fn rename_function(target: Dialect, upper_name: &str) -> Option<&'static str> {
    match target {
        Dialect::Sqlite => rename_function_sqlite(upper_name),
        // Write-side inverse of `canonicalize_function`: render the neutral
        // bitwise-aggregate names back to each dialect's own spelling.
        Dialect::Mysql => match upper_name {
            "BITWISE_AND_AGG" => Some("BIT_AND"),
            "BITWISE_OR_AGG" => Some("BIT_OR"),
            "BITWISE_XOR_AGG" => Some("BIT_XOR"),
            "BITWISE_COUNT" => Some("BIT_COUNT"),
            _ => None,
        },
        Dialect::Postgres => match upper_name {
            "BITWISE_AND_AGG" => Some("BIT_AND"),
            "BITWISE_OR_AGG" => Some("BIT_OR"),
            "BITWISE_XOR_AGG" => Some("BIT_XOR"),
            // canonical BITWISE_COUNT (from MySQL) renders BITWISE_COUNT in
            // postgres; postgres's own BIT_COUNT was never canonicalized.
            _ => None,
        },
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
        // MEDIUMINT is kept by SQLGlot (not folded to INTEGER).
        "INT4" | "INT1" | "INT8" | "INT16" | "INT32" | "INT64" => "INTEGER",
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
        "CHR" => "CHAR",
        "ANY_VALUE" => "MAX",
        "GEN_RANDOM_UUID" => "UUID",
        "BOOL_AND" | "LOGICAL_AND" => "MIN",
        "BOOL_OR" | "LOGICAL_OR" => "MAX",
        _ => return None,
    };
    Some(rename)
}
