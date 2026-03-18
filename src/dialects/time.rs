//! Time Format Mapping for cross-dialect transpilation.
//!
//! This module handles conversion of date/time format specifiers between different
//! SQL dialects. Each dialect has its own conventions for formatting dates and times:
//!
//! - **strftime** (Python, SQLite): `%Y`, `%m`, `%d`, `%H`, `%M`, `%S`
//! - **MySQL**: `%Y`, `%m`, `%d`, `%H`, `%i`, `%s`
//! - **PostgreSQL/Oracle**: `YYYY`, `MM`, `DD`, `HH24`, `MI`, `SS`
//! - **BigQuery**: strftime-like (`%Y`, `%m`, `%d`, `%H`, `%M`, `%S`)
//! - **Snowflake**: `YYYY`, `MM`, `DD`, `HH24`, `MI`, `SS`, `FF`
//! - **Spark/Hive**: Java DateTimeFormatter (`yyyy`, `MM`, `dd`, `HH`, `mm`, `ss`)
//! - **T-SQL**: Primarily uses numeric style codes (120, 121, etc.)
//!
//! # Example
//!
//! ```rust
//! use sqlglot_rust::dialects::time::{format_time, TimeFormatStyle};
//!
//! // Convert MySQL format to PostgreSQL format
//! let pg_format = format_time("%Y-%m-%d %H:%i:%s", 
//!     TimeFormatStyle::Mysql, 
//!     TimeFormatStyle::Postgres);
//! assert_eq!(pg_format, "YYYY-MM-DD HH24:MI:SS");
//! ```

use super::Dialect;
use std::collections::HashMap;

/// Time format styles used by different dialect families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeFormatStyle {
    /// Python strftime / SQLite / BigQuery: `%Y`, `%m`, `%d`, `%H`, `%M`, `%S`
    Strftime,
    /// MySQL: `%Y`, `%m`, `%d`, `%H`, `%i`, `%s` (note: `%i` for minutes)
    Mysql,
    /// PostgreSQL / Oracle / Redshift: `YYYY`, `MM`, `DD`, `HH24`, `MI`, `SS`
    Postgres,
    /// Snowflake: `YYYY`, `MM`, `DD`, `HH24`, `MI`, `SS`, `FF`
    Snowflake,
    /// Spark / Hive / Databricks: Java DateTimeFormatter `yyyy`, `MM`, `dd`, `HH`, `mm`, `ss`
    Java,
    /// T-SQL: Uses numeric style codes (FORMAT function uses .NET patterns)
    Tsql,
    /// ClickHouse: Similar to strftime but with some differences
    ClickHouse,
}

impl TimeFormatStyle {
    /// Determine the time format style for a given dialect.
    #[must_use]
    pub fn for_dialect(dialect: Dialect) -> Self {
        match dialect {
            // strftime-style dialects
            Dialect::Ansi | Dialect::Sqlite | Dialect::BigQuery | Dialect::DuckDb => {
                TimeFormatStyle::Strftime
            }

            // MySQL family
            Dialect::Mysql
            | Dialect::Doris
            | Dialect::SingleStore
            | Dialect::StarRocks => TimeFormatStyle::Mysql,

            // Postgres family
            Dialect::Postgres
            | Dialect::Oracle
            | Dialect::Redshift
            | Dialect::Materialize
            | Dialect::RisingWave
            | Dialect::Exasol
            | Dialect::Teradata => TimeFormatStyle::Postgres,

            // Snowflake
            Dialect::Snowflake => TimeFormatStyle::Snowflake,

            // Hive/Spark family (Java DateTimeFormatter)
            Dialect::Hive | Dialect::Spark | Dialect::Databricks => TimeFormatStyle::Java,

            // T-SQL family
            Dialect::Tsql | Dialect::Fabric => TimeFormatStyle::Tsql,

            // Presto family uses Java-like patterns
            Dialect::Presto | Dialect::Trino | Dialect::Athena => TimeFormatStyle::Java,

            // ClickHouse
            Dialect::ClickHouse => TimeFormatStyle::ClickHouse,

            // Others - default to strftime
            Dialect::Dremio | Dialect::Drill | Dialect::Druid | Dialect::Tableau | Dialect::Prql => {
                TimeFormatStyle::Strftime
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Format specifier mappings
// ═══════════════════════════════════════════════════════════════════════════

/// Mapping entry representing a format specifier's equivalent across styles.
#[derive(Debug, Clone)]
struct FormatMapping {
    strftime: &'static str,
    mysql: &'static str,
    postgres: &'static str,
    snowflake: &'static str,
    java: &'static str,
    tsql: &'static str,
    clickhouse: &'static str,
}

impl FormatMapping {
    /// Get the specifier for a given style.
    fn get(&self, style: TimeFormatStyle) -> &'static str {
        match style {
            TimeFormatStyle::Strftime => self.strftime,
            TimeFormatStyle::Mysql => self.mysql,
            TimeFormatStyle::Postgres => self.postgres,
            TimeFormatStyle::Snowflake => self.snowflake,
            TimeFormatStyle::Java => self.java,
            TimeFormatStyle::Tsql => self.tsql,
            TimeFormatStyle::ClickHouse => self.clickhouse,
        }
    }
}

/// Build the canonical format mappings table.
/// Each entry maps a semantic time component to its representation in each style.
fn build_format_mappings() -> Vec<FormatMapping> {
    vec![
        // ── Year ───────────────────────────────────────────────────────
        FormatMapping {
            strftime: "%Y",      // 4-digit year
            mysql: "%Y",
            postgres: "YYYY",
            snowflake: "YYYY",
            java: "yyyy",
            tsql: "yyyy",
            clickhouse: "%Y",
        },
        FormatMapping {
            strftime: "%y",      // 2-digit year
            mysql: "%y",
            postgres: "YY",
            snowflake: "YY",
            java: "yy",
            tsql: "yy",
            clickhouse: "%y",
        },
        // ── Month ──────────────────────────────────────────────────────
        FormatMapping {
            strftime: "%m",      // Month as zero-padded decimal (01-12)
            mysql: "%m",
            postgres: "MM",
            snowflake: "MM",
            java: "MM",
            tsql: "MM",
            clickhouse: "%m",
        },
        FormatMapping {
            strftime: "%b",      // Abbreviated month name (Jan, Feb, ...)
            mysql: "%b",
            postgres: "Mon",
            snowflake: "MON",
            java: "MMM",
            tsql: "MMM",
            clickhouse: "%b",
        },
        FormatMapping {
            strftime: "%B",      // Full month name
            mysql: "%M",
            postgres: "Month",
            snowflake: "MMMM",
            java: "MMMM",
            tsql: "MMMM",
            clickhouse: "%B",
        },
        // ── Day ────────────────────────────────────────────────────────
        FormatMapping {
            strftime: "%d",      // Day of month as zero-padded decimal (01-31)
            mysql: "%d",
            postgres: "DD",
            snowflake: "DD",
            java: "dd",
            tsql: "dd",
            clickhouse: "%d",
        },
        FormatMapping {
            strftime: "%e",      // Day of month as space-padded decimal
            mysql: "%e",
            postgres: "FMDD",
            snowflake: "DD",     // Snowflake doesn't have space-padded
            java: "d",
            tsql: "d",
            clickhouse: "%e",
        },
        FormatMapping {
            strftime: "%j",      // Day of year (001-366)
            mysql: "%j",
            postgres: "DDD",
            snowflake: "DDD",
            java: "DDD",
            tsql: "",            // T-SQL doesn't have direct equivalent
            clickhouse: "%j",
        },
        // ── Weekday ────────────────────────────────────────────────────
        FormatMapping {
            strftime: "%a",      // Abbreviated weekday name
            mysql: "%a",
            postgres: "Dy",
            snowflake: "DY",
            java: "EEE",
            tsql: "ddd",
            clickhouse: "%a",
        },
        FormatMapping {
            strftime: "%A",      // Full weekday name
            mysql: "%W",
            postgres: "Day",
            snowflake: "DY",     // Snowflake uses uppercase abbreviated
            java: "EEEE",
            tsql: "dddd",
            clickhouse: "%A",
        },
        FormatMapping {
            strftime: "%w",      // Weekday as number (0=Sunday, 6=Saturday)
            mysql: "%w",
            postgres: "D",
            snowflake: "D",
            java: "e",
            tsql: "",
            clickhouse: "%w",
        },
        FormatMapping {
            strftime: "%u",      // Weekday as number (1=Monday, 7=Sunday)
            mysql: "%u",
            postgres: "ID",
            snowflake: "ID",
            java: "u",
            tsql: "",
            clickhouse: "%u",
        },
        // ── Week ───────────────────────────────────────────────────────
        FormatMapping {
            strftime: "%W",      // Week number of year (Monday as first day)
            mysql: "%v",         // MySQL uses %v for ISO week
            postgres: "IW",
            snowflake: "WW",
            java: "ww",
            tsql: "ww",
            clickhouse: "%V",
        },
        FormatMapping {
            strftime: "%U",      // Week number of year (Sunday as first day)
            mysql: "%U",
            postgres: "WW",
            snowflake: "WW",
            java: "ww",
            tsql: "ww",
            clickhouse: "%U",
        },
        // ── Hour ───────────────────────────────────────────────────────
        FormatMapping {
            strftime: "%H",      // Hour (24-hour) as zero-padded decimal (00-23)
            mysql: "%H",
            postgres: "HH24",
            snowflake: "HH24",
            java: "HH",
            tsql: "HH",
            clickhouse: "%H",
        },
        FormatMapping {
            strftime: "%I",      // Hour (12-hour) as zero-padded decimal (01-12)
            mysql: "%h",
            postgres: "HH12",
            snowflake: "HH12",
            java: "hh",
            tsql: "hh",
            clickhouse: "%I",
        },
        // ── Minute ─────────────────────────────────────────────────────
        FormatMapping {
            strftime: "%M",      // Minute as zero-padded decimal (00-59)
            mysql: "%i",         // NOTE: MySQL uses %i for minutes!
            postgres: "MI",
            snowflake: "MI",
            java: "mm",
            tsql: "mm",
            clickhouse: "%M",
        },
        // ── Second ─────────────────────────────────────────────────────
        FormatMapping {
            strftime: "%S",      // Second as zero-padded decimal (00-59)
            mysql: "%s",
            postgres: "SS",
            snowflake: "SS",
            java: "ss",
            tsql: "ss",
            clickhouse: "%S",
        },
        // ── Fractional seconds ─────────────────────────────────────────
        FormatMapping {
            strftime: "%f",      // Microseconds (6 digits)
            mysql: "%f",
            postgres: "US",      // Microseconds
            snowflake: "FF6",
            java: "SSSSSS",
            tsql: "ffffff",
            clickhouse: "%f",
        },
        // ── AM/PM ──────────────────────────────────────────────────────
        FormatMapping {
            strftime: "%p",      // AM or PM
            mysql: "%p",
            postgres: "AM",
            snowflake: "AM",
            java: "a",
            tsql: "tt",
            clickhouse: "%p",
        },
        // ── Timezone ───────────────────────────────────────────────────
        FormatMapping {
            strftime: "%z",      // UTC offset as +HHMM or -HHMM
            mysql: "",           // MySQL doesn't support timezone in format
            postgres: "OF",
            snowflake: "TZH:TZM",
            java: "Z",
            tsql: "zzz",
            clickhouse: "%z",
        },
        FormatMapping {
            strftime: "%Z",      // Timezone name
            mysql: "",
            postgres: "TZ",
            snowflake: "TZR",
            java: "z",
            tsql: "",
            clickhouse: "%Z",
        },
        // ── Special ────────────────────────────────────────────────────
        FormatMapping {
            strftime: "%%",      // Literal %
            mysql: "%%",
            postgres: "",        // Postgres doesn't need escaping
            snowflake: "",
            java: "",
            tsql: "",
            clickhouse: "%%",
        },
    ]
}

/// Lazily build and cache the format mappings.
fn get_format_mappings() -> &'static Vec<FormatMapping> {
    use std::sync::OnceLock;
    static MAPPINGS: OnceLock<Vec<FormatMapping>> = OnceLock::new();
    MAPPINGS.get_or_init(build_format_mappings)
}

/// Build a lookup table from source style specifiers to FormatMapping index.
///
/// This function is available for potential future optimization of format
/// string parsing, allowing O(1) lookups instead of linear scans.
#[allow(dead_code)]
fn build_style_lookup(style: TimeFormatStyle) -> HashMap<&'static str, usize> {
    let mappings = get_format_mappings();
    let mut lookup = HashMap::new();
    for (i, mapping) in mappings.iter().enumerate() {
        let spec = mapping.get(style);
        if !spec.is_empty() {
            lookup.insert(spec, i);
        }
    }
    lookup
}

// ═══════════════════════════════════════════════════════════════════════════
// Format conversion
// ═══════════════════════════════════════════════════════════════════════════

/// Convert a time format string from one dialect style to another.
///
/// # Arguments
///
/// * `format_str` - The format string to convert
/// * `source` - The source format style
/// * `target` - The target format style
///
/// # Returns
///
/// The converted format string with specifiers replaced according to the target style.
/// Literal text (not matching any known specifier) is preserved as-is.
///
/// # Example
///
/// ```rust
/// use sqlglot_rust::dialects::time::{format_time, TimeFormatStyle};
///
/// let result = format_time("%Y-%m-%d", TimeFormatStyle::Strftime, TimeFormatStyle::Postgres);
/// assert_eq!(result, "YYYY-MM-DD");
/// ```
#[must_use]
pub fn format_time(format_str: &str, source: TimeFormatStyle, target: TimeFormatStyle) -> String {
    if source == target {
        return format_str.to_string();
    }

    // Use the appropriate parser based on source style
    match source {
        TimeFormatStyle::Strftime
        | TimeFormatStyle::Mysql
        | TimeFormatStyle::ClickHouse => convert_strftime_style(format_str, source, target),
        TimeFormatStyle::Postgres => convert_postgres_style(format_str, target),
        TimeFormatStyle::Snowflake => convert_snowflake_style(format_str, target),
        TimeFormatStyle::Java | TimeFormatStyle::Tsql => convert_java_style(format_str, source, target),
    }
}

/// Convert a format string from strftime-style (%, MySQL, ClickHouse) to target.
fn convert_strftime_style(format_str: &str, source: TimeFormatStyle, target: TimeFormatStyle) -> String {
    let mappings = get_format_mappings();
    let mut result = String::with_capacity(format_str.len() * 2);
    let mut chars = format_str.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '%' {
            if let Some(&next) = chars.peek() {
                chars.next();
                let spec = format!("%{}", next);
                
                // Find matching mapping
                let mapped = mappings.iter().find(|m| m.get(source) == spec);
                
                if let Some(mapping) = mapped {
                    let target_spec = mapping.get(target);
                    if target_spec.is_empty() {
                        // No equivalent in target - keep original or use placeholder
                        result.push_str(&spec);
                    } else {
                        result.push_str(target_spec);
                    }
                } else {
                    // Unknown specifier - keep as-is
                    result.push_str(&spec);
                }
            } else {
                // Trailing % - keep it
                result.push('%');
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Convert a format string from Postgres style to target.
fn convert_postgres_style(format_str: &str, target: TimeFormatStyle) -> String {
    let mappings = get_format_mappings();
    let mut result = String::with_capacity(format_str.len() * 2);
    let chars: Vec<char> = format_str.chars().collect();
    let mut i = 0;

    // Postgres specifiers to check, ordered by length (longest first)
    let pg_specifiers: &[&str] = &[
        "YYYY", "MMMM", "Month", "Mon", "MM", "DDD", "DD", "Day", "Dy", "D",
        "HH24", "HH12", "HH", "MI", "SS", "US", "AM", "PM", "TZH:TZM", "TZR",
        "TZ", "OF", "IW", "WW", "YY", "ID", "FMDD",
    ];

    while i < chars.len() {
        let remaining: String = chars[i..].iter().collect();
        let mut matched = false;

        // Try to match longest specifier first
        for spec in pg_specifiers {
            if remaining.starts_with(spec) || remaining.to_uppercase().starts_with(&spec.to_uppercase()) {
                // Find the mapping
                let mapping = mappings.iter().find(|m|
                    m.postgres.eq_ignore_ascii_case(spec)
                );
                
                if let Some(m) = mapping {
                    let target_spec = m.get(target);
                    if !target_spec.is_empty() {
                        result.push_str(target_spec);
                    } else {
                        result.push_str(spec);
                    }
                } else {
                    result.push_str(spec);
                }
                i += spec.len();
                matched = true;
                break;
            }
        }

        if !matched {
            // Check for quoted literal (in Postgres, text in double quotes is literal)
            if chars[i] == '"' {
                result.push(chars[i]);
                i += 1;
                while i < chars.len() && chars[i] != '"' {
                    result.push(chars[i]);
                    i += 1;
                }
                if i < chars.len() {
                    result.push(chars[i]); // closing quote
                    i += 1;
                }
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }
    }

    result
}

/// Convert a format string from Snowflake style to target.
fn convert_snowflake_style(format_str: &str, target: TimeFormatStyle) -> String {
    let mappings = get_format_mappings();
    let mut result = String::with_capacity(format_str.len() * 2);
    let chars: Vec<char> = format_str.chars().collect();
    let mut i = 0;

    // Snowflake specifiers (similar to Postgres but with some differences)
    let sf_specifiers: &[&str] = &[
        "YYYY", "MMMM", "MON", "MM", "DDD", "DD", "DY", "D",
        "HH24", "HH12", "HH", "MI", "SS", "FF6", "FF3", "FF",
        "AM", "PM", "TZH:TZM", "TZR", "WW", "YY", "ID",
    ];

    while i < chars.len() {
        let remaining: String = chars[i..].iter().collect();
        let mut matched = false;

        for spec in sf_specifiers {
            if remaining.starts_with(spec) || remaining.to_uppercase().starts_with(&spec.to_uppercase()) {
                let mapping = mappings.iter().find(|m| 
                    m.snowflake.eq_ignore_ascii_case(spec)
                );
                
                if let Some(m) = mapping {
                    let target_spec = m.get(target);
                    if !target_spec.is_empty() {
                        result.push_str(target_spec);
                    } else {
                        result.push_str(spec);
                    }
                } else {
                    result.push_str(spec);
                }
                i += spec.len();
                matched = true;
                break;
            }
        }

        if !matched {
            // Check for quoted literal
            if chars[i] == '"' {
                result.push(chars[i]);
                i += 1;
                while i < chars.len() && chars[i] != '"' {
                    result.push(chars[i]);
                    i += 1;
                }
                if i < chars.len() {
                    result.push(chars[i]);
                    i += 1;
                }
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }
    }

    result
}

/// Convert a format string from Java/T-SQL style to target.
fn convert_java_style(format_str: &str, source: TimeFormatStyle, target: TimeFormatStyle) -> String {
    let mappings = get_format_mappings();
    let mut result = String::with_capacity(format_str.len() * 2);
    let chars: Vec<char> = format_str.chars().collect();
    let mut i = 0;

    // Java DateTimeFormatter patterns
    let java_specifiers: &[&str] = &[
        "yyyy", "YYYY", "yy", "YY",
        "MMMM", "MMM", "MM", "M",
        "dd", "d", "DDD",
        "EEEE", "EEE", "e", "u",
        "HH", "hh", "H", "h",
        "mm", "m",
        "ss", "s",
        "SSSSSS", "SSS", "SS", "S",
        "a", "Z", "z",
        "ww",
    ];

    while i < chars.len() {
        let remaining: String = chars[i..].iter().collect();
        let mut matched = false;

        // Check for quoted literals (Java uses single quotes)
        if chars[i] == '\'' {
            result.push(chars[i]);
            i += 1;
            while i < chars.len() && chars[i] != '\'' {
                result.push(chars[i]);
                i += 1;
            }
            if i < chars.len() {
                result.push(chars[i]);
                i += 1;
            }
            continue;
        }

        for spec in java_specifiers {
            if remaining.starts_with(spec) {
                let mapping = mappings.iter().find(|m| {
                    let src_spec = m.get(source);
                    src_spec == *spec
                });
                
                if let Some(m) = mapping {
                    let target_spec = m.get(target);
                    if !target_spec.is_empty() {
                        result.push_str(target_spec);
                    } else {
                        result.push_str(spec);
                    }
                } else {
                    result.push_str(spec);
                }
                i += spec.len();
                matched = true;
                break;
            }
        }

        if !matched {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}

// ═══════════════════════════════════════════════════════════════════════════
// Dialect-aware conversion
// ═══════════════════════════════════════════════════════════════════════════

/// Convert a time format string from one SQL dialect to another.
///
/// This is the main entry point for dialect-to-dialect format conversion.
///
/// # Arguments
///
/// * `format_str` - The format string to convert
/// * `source_dialect` - The source SQL dialect
/// * `target_dialect` - The target SQL dialect
///
/// # Returns
///
/// The converted format string appropriate for the target dialect.
///
/// # Example
///
/// ```rust
/// use sqlglot_rust::dialects::time::format_time_dialect;
/// use sqlglot_rust::Dialect;
///
/// // Convert MySQL format to PostgreSQL
/// let result = format_time_dialect("%Y-%m-%d %H:%i:%s", Dialect::Mysql, Dialect::Postgres);
/// assert_eq!(result, "YYYY-MM-DD HH24:MI:SS");
/// ```
#[must_use]
pub fn format_time_dialect(format_str: &str, source_dialect: Dialect, target_dialect: Dialect) -> String {
    let source_style = TimeFormatStyle::for_dialect(source_dialect);
    let target_style = TimeFormatStyle::for_dialect(target_dialect);
    format_time(format_str, source_style, target_style)
}

// ═══════════════════════════════════════════════════════════════════════════
// T-SQL Style Codes
// ═══════════════════════════════════════════════════════════════════════════

/// T-SQL date/time style codes used with CONVERT function.
///
/// T-SQL primarily uses numeric style codes for date formatting with CONVERT,
/// rather than format patterns. This provides mappings for common styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TsqlStyleCode {
    /// Style 100: mon dd yyyy hh:miAM (or PM)
    Default100 = 100,
    /// Style 101: mm/dd/yyyy (USA)
    UsaDate = 101,
    /// Style 102: yyyy.mm.dd (ANSI)
    AnsiDate = 102,
    /// Style 103: dd/mm/yyyy (British/French)
    BritishDate = 103,
    /// Style 104: dd.mm.yyyy (German)
    GermanDate = 104,
    /// Style 105: dd-mm-yyyy (Italian)
    ItalianDate = 105,
    /// Style 106: dd mon yyyy
    DayMonYear = 106,
    /// Style 107: Mon dd, yyyy
    MonDayYear = 107,
    /// Style 108: hh:mi:ss
    TimeOnly = 108,
    /// Style 110: mm-dd-yyyy (USA with dashes)
    UsaDashes = 110,
    /// Style 111: yyyy/mm/dd (Japan)
    JapanDate = 111,
    /// Style 112: yyyymmdd (ISO basic)
    IsoBasic = 112,
    /// Style 114: hh:mi:ss:mmm
    TimeWithMs = 114,
    /// Style 120: yyyy-mm-dd hh:mi:ss (ODBC canonical)
    OdbcCanonical = 120,
    /// Style 121: yyyy-mm-dd hh:mi:ss.mmm (ODBC with milliseconds)
    OdbcWithMs = 121,
    /// Style 126: yyyy-mm-ddThh:mi:ss.mmm (ISO8601)
    Iso8601 = 126,
    /// Style 127: yyyy-mm-ddThh:mi:ss.mmmZ (ISO8601 with timezone)
    Iso8601Tz = 127,
}

impl TsqlStyleCode {
    /// Get the equivalent format pattern for a T-SQL style code.
    ///
    /// Returns the pattern in strftime style for use in other dialects.
    #[must_use]
    pub fn to_format_pattern(&self) -> &'static str {
        match self {
            TsqlStyleCode::Default100 => "%b %d %Y %I:%M%p",
            TsqlStyleCode::UsaDate => "%m/%d/%Y",
            TsqlStyleCode::AnsiDate => "%Y.%m.%d",
            TsqlStyleCode::BritishDate => "%d/%m/%Y",
            TsqlStyleCode::GermanDate => "%d.%m.%Y",
            TsqlStyleCode::ItalianDate => "%d-%m-%Y",
            TsqlStyleCode::DayMonYear => "%d %b %Y",
            TsqlStyleCode::MonDayYear => "%b %d, %Y",
            TsqlStyleCode::TimeOnly => "%H:%M:%S",
            TsqlStyleCode::UsaDashes => "%m-%d-%Y",
            TsqlStyleCode::JapanDate => "%Y/%m/%d",
            TsqlStyleCode::IsoBasic => "%Y%m%d",
            TsqlStyleCode::TimeWithMs => "%H:%M:%S:%f",
            TsqlStyleCode::OdbcCanonical => "%Y-%m-%d %H:%M:%S",
            TsqlStyleCode::OdbcWithMs => "%Y-%m-%d %H:%M:%S.%f",
            TsqlStyleCode::Iso8601 => "%Y-%m-%dT%H:%M:%S.%f",
            TsqlStyleCode::Iso8601Tz => "%Y-%m-%dT%H:%M:%S.%fZ",
        }
    }

    /// Try to parse a T-SQL style code from a number.
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            100 => Some(TsqlStyleCode::Default100),
            101 => Some(TsqlStyleCode::UsaDate),
            102 => Some(TsqlStyleCode::AnsiDate),
            103 => Some(TsqlStyleCode::BritishDate),
            104 => Some(TsqlStyleCode::GermanDate),
            105 => Some(TsqlStyleCode::ItalianDate),
            106 => Some(TsqlStyleCode::DayMonYear),
            107 => Some(TsqlStyleCode::MonDayYear),
            108 => Some(TsqlStyleCode::TimeOnly),
            110 => Some(TsqlStyleCode::UsaDashes),
            111 => Some(TsqlStyleCode::JapanDate),
            112 => Some(TsqlStyleCode::IsoBasic),
            114 => Some(TsqlStyleCode::TimeWithMs),
            120 => Some(TsqlStyleCode::OdbcCanonical),
            121 => Some(TsqlStyleCode::OdbcWithMs),
            126 => Some(TsqlStyleCode::Iso8601),
            127 => Some(TsqlStyleCode::Iso8601Tz),
            _ => None,
        }
    }

    /// Get the numeric style code.
    pub fn code(&self) -> i32 {
        *self as i32
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Warnings for unsupported conversions
// ═══════════════════════════════════════════════════════════════════════════

/// Result of a format conversion that may include warnings.
#[derive(Debug, Clone)]
pub struct FormatConversionResult {
    /// The converted format string.
    pub format: String,
    /// Warnings about unsupported or potentially lossy conversions.
    pub warnings: Vec<String>,
}

/// Convert a time format string with warning collection.
///
/// Similar to `format_time` but collects warnings about specifiers
/// that don't have direct equivalents in the target format.
#[must_use]
pub fn format_time_with_warnings(
    format_str: &str,
    source: TimeFormatStyle,
    target: TimeFormatStyle,
) -> FormatConversionResult {
    let mut warnings = Vec::new();
    let mappings = get_format_mappings();

    // Pre-scan for problematic specifiers
    match source {
        TimeFormatStyle::Strftime | TimeFormatStyle::Mysql | TimeFormatStyle::ClickHouse => {
            let mut chars = format_str.chars().peekable();
            while let Some(ch) = chars.next() {
                if ch == '%'
                    && let Some(&next) = chars.peek()
                {
                    chars.next();
                    let spec = format!("%{}", next);
                    let mapping = mappings.iter().find(|m| m.get(source) == spec);
                    if let Some(m) = mapping
                        && m.get(target).is_empty()
                    {
                        warnings.push(format!(
                            "Format specifier '{}' has no equivalent in target format",
                            spec
                        ));
                    }
                }
            }
        }
        _ => {
            // For other styles, simplified warning check
            // Full implementation would scan for style-specific specifiers
        }
    }

    let format = format_time(format_str, source, target);
    FormatConversionResult { format, warnings }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strftime_to_postgres() {
        assert_eq!(
            format_time("%Y-%m-%d", TimeFormatStyle::Strftime, TimeFormatStyle::Postgres),
            "YYYY-MM-DD"
        );
        assert_eq!(
            format_time("%H:%M:%S", TimeFormatStyle::Strftime, TimeFormatStyle::Postgres),
            "HH24:MI:SS"
        );
        assert_eq!(
            format_time("%Y-%m-%d %H:%M:%S", TimeFormatStyle::Strftime, TimeFormatStyle::Postgres),
            "YYYY-MM-DD HH24:MI:SS"
        );
    }

    #[test]
    fn test_mysql_to_postgres() {
        // MySQL uses %i for minutes
        assert_eq!(
            format_time("%Y-%m-%d %H:%i:%s", TimeFormatStyle::Mysql, TimeFormatStyle::Postgres),
            "YYYY-MM-DD HH24:MI:SS"
        );
    }

    #[test]
    fn test_postgres_to_mysql() {
        assert_eq!(
            format_time("YYYY-MM-DD HH24:MI:SS", TimeFormatStyle::Postgres, TimeFormatStyle::Mysql),
            "%Y-%m-%d %H:%i:%s"
        );
    }

    #[test]
    fn test_postgres_to_strftime() {
        assert_eq!(
            format_time("YYYY-MM-DD", TimeFormatStyle::Postgres, TimeFormatStyle::Strftime),
            "%Y-%m-%d"
        );
    }

    #[test]
    fn test_strftime_to_java() {
        assert_eq!(
            format_time("%Y-%m-%d", TimeFormatStyle::Strftime, TimeFormatStyle::Java),
            "yyyy-MM-dd"
        );
        assert_eq!(
            format_time("%H:%M:%S", TimeFormatStyle::Strftime, TimeFormatStyle::Java),
            "HH:mm:ss"
        );
    }

    #[test]
    fn test_java_to_strftime() {
        assert_eq!(
            format_time("yyyy-MM-dd", TimeFormatStyle::Java, TimeFormatStyle::Strftime),
            "%Y-%m-%d"
        );
        assert_eq!(
            format_time("HH:mm:ss", TimeFormatStyle::Java, TimeFormatStyle::Strftime),
            "%H:%M:%S"
        );
    }

    #[test]
    fn test_strftime_to_snowflake() {
        assert_eq!(
            format_time("%Y-%m-%d", TimeFormatStyle::Strftime, TimeFormatStyle::Snowflake),
            "YYYY-MM-DD"
        );
    }

    #[test]
    fn test_same_style_noop() {
        let format = "%Y-%m-%d %H:%M:%S";
        assert_eq!(
            format_time(format, TimeFormatStyle::Strftime, TimeFormatStyle::Strftime),
            format
        );
    }

    #[test]
    fn test_dialect_conversion() {
        assert_eq!(
            format_time_dialect("%Y-%m-%d %H:%i:%s", Dialect::Mysql, Dialect::Postgres),
            "YYYY-MM-DD HH24:MI:SS"
        );
        assert_eq!(
            format_time_dialect("YYYY-MM-DD HH24:MI:SS", Dialect::Postgres, Dialect::Spark),
            "yyyy-MM-dd HH:mm:ss"
        );
    }

    #[test]
    fn test_literal_preservation() {
        // Literal characters should be preserved
        assert_eq!(
            format_time("%Y/%m/%d", TimeFormatStyle::Strftime, TimeFormatStyle::Postgres),
            "YYYY/MM/DD"
        );
        assert_eq!(
            format_time("%Y at %H:%M", TimeFormatStyle::Strftime, TimeFormatStyle::Postgres),
            "YYYY at HH24:MI"
        );
    }

    #[test]
    fn test_tsql_style_codes() {
        assert_eq!(TsqlStyleCode::OdbcCanonical.to_format_pattern(), "%Y-%m-%d %H:%M:%S");
        assert_eq!(TsqlStyleCode::UsaDate.to_format_pattern(), "%m/%d/%Y");
        assert_eq!(TsqlStyleCode::from_code(120), Some(TsqlStyleCode::OdbcCanonical));
        assert_eq!(TsqlStyleCode::from_code(999), None);
    }

    #[test]
    fn test_12hour_format() {
        assert_eq!(
            format_time("%I:%M %p", TimeFormatStyle::Strftime, TimeFormatStyle::Postgres),
            "HH12:MI AM"
        );
    }

    #[test]
    fn test_month_names() {
        assert_eq!(
            format_time("%b %d, %Y", TimeFormatStyle::Strftime, TimeFormatStyle::Postgres),
            "Mon DD, YYYY"
        );
        assert_eq!(
            format_time("%B", TimeFormatStyle::Strftime, TimeFormatStyle::Mysql),
            "%M"
        );
    }

    #[test]
    fn test_format_style_for_dialect() {
        assert_eq!(TimeFormatStyle::for_dialect(Dialect::Mysql), TimeFormatStyle::Mysql);
        assert_eq!(TimeFormatStyle::for_dialect(Dialect::Postgres), TimeFormatStyle::Postgres);
        assert_eq!(TimeFormatStyle::for_dialect(Dialect::Spark), TimeFormatStyle::Java);
        assert_eq!(TimeFormatStyle::for_dialect(Dialect::Snowflake), TimeFormatStyle::Snowflake);
        assert_eq!(TimeFormatStyle::for_dialect(Dialect::BigQuery), TimeFormatStyle::Strftime);
    }
}
