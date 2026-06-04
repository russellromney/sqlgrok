use std::borrow::Cow;

/// Parser-internal SQL text that can borrow from the source SQL or own text
/// created by decoding/normalization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SqlText<'sql>(Cow<'sql, str>);

impl<'sql> SqlText<'sql> {
    pub(crate) fn borrowed(text: &'sql str) -> Self {
        Self(Cow::Borrowed(text))
    }

    pub(crate) fn owned(text: String) -> Self {
        Self(Cow::Owned(text))
    }

    pub(crate) fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn is_borrowed(&self) -> bool {
        matches!(self.0, Cow::Borrowed(_))
    }

    pub(crate) fn into_owned(self) -> String {
        self.0.into_owned()
    }
}

impl<'sql> From<&'sql str> for SqlText<'sql> {
    fn from(value: &'sql str) -> Self {
        Self::borrowed(value)
    }
}

impl<'sql> From<String> for SqlText<'sql> {
    fn from(value: String) -> Self {
        Self::owned(value)
    }
}

#[cfg(test)]
mod tests {
    use super::SqlText;

    #[test]
    fn borrowed_text_exposes_source_without_copying() {
        let source = "identifier";
        let text = SqlText::borrowed(source);

        assert_eq!(text.as_str(), "identifier");
        assert!(text.is_borrowed());
        assert_eq!(text.into_owned(), "identifier");
    }

    #[test]
    fn owned_text_carries_decoded_or_generated_strings() {
        let text = SqlText::owned("CURRENT_TIMESTAMP".to_string());

        assert_eq!(text.as_str(), "CURRENT_TIMESTAMP");
        assert!(!text.is_borrowed());
        assert_eq!(text.into_owned(), "CURRENT_TIMESTAMP");
    }
}
