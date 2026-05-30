mod sql_generator;

pub use sql_generator::Generator;

use crate::ast::Statement;
use crate::dialects::Dialect;

/// Generate a SQL string from a [`Statement`] AST for the given dialect.
#[must_use]
pub fn generate(statement: &Statement, dialect: Dialect) -> String {
    let mut generator = Generator::with_dialect(dialect);
    let s = generator.generate(statement);
    // Python SQLGlot strips() the final generator output. Mirror it so
    // forms like `SELECT e'\n'` (sqlite/mysql target render bare value
    // without quotes) match exactly.
    let trimmed = s.trim_matches(|c: char| c == ' ' || c == '\t' || c == '\n' || c == '\r');
    if trimmed.len() == s.len() {
        s
    } else {
        trimmed.to_string()
    }
}

/// Generate a pretty-printed SQL string from a [`Statement`] AST.
///
/// Produces formatted SQL with newlines and indentation for readability.
#[must_use]
pub fn generate_pretty(statement: &Statement, dialect: Dialect) -> String {
    let mut generator = Generator::pretty_with_dialect(dialect);
    generator.generate(statement)
}
