use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// The error type for a value that does not match the format expected by its type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidValueError {
    expected: &'static str,
}

impl InvalidValueError {
    #[inline]
    pub(crate) const fn new(expected: &'static str) -> Self {
        Self {
            expected,
        }
    }

    /// Returns a short description of the format that the input failed to match.
    #[inline]
    pub const fn expected(&self) -> &'static str {
        self.expected
    }
}

impl Display for InvalidValueError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "the input is not a valid {}", self.expected)
    }
}

impl Error for InvalidValueError {}

/// The error type returned by [`VCard::validate`](crate::VCard::validate).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValidationError {
    /// A vCard must have at least one FN property.
    MissingFormattedName,
    /// The MEMBER property can only be used when the KIND property is set to `group`.
    MemberWithoutGroupKind,
}

impl Display for ValidationError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::MissingFormattedName => f.write_str("a vCard must have at least one FN property"),
            Self::MemberWithoutGroupKind => {
                f.write_str("the MEMBER property can only be used when KIND is group")
            },
        }
    }
}

impl Error for ValidationError {}

/// The error type produced when parsing vCard text fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// The number of the physical line where the error happened, starting from 1.
    /// For a folded line, this points to its first physical line.
    pub line: usize,
    /// The reason why parsing failed.
    pub kind: ParseErrorKind,
}

impl Display for ParseError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} (at line {})", self.kind, self.line)
    }
}

impl Error for ParseError {}

/// The reasons why parsing vCard text can fail.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseErrorKind {
    /// The input does not start with a `BEGIN:VCARD` line.
    ExpectedBegin,
    /// The line right after `BEGIN:VCARD` is not a VERSION property.
    ExpectedVersion,
    /// The VERSION property exists but its value is not `4.0`.
    UnsupportedVersion(String),
    /// The input ends before the `END:VCARD` line.
    MissingEnd,
    /// There is remaining content after the `END:VCARD` line.
    TrailingData,
    /// The line is not a well-formed content line.
    InvalidLine,
    /// The group prefix of the property is not a valid group name.
    InvalidGroupName,
    /// A parameter has an unknown format or an invalid value.
    InvalidParameter(String),
    /// The value of a property cannot be parsed.
    InvalidValue {
        /// The name of the property whose value is invalid.
        property: String,
    },
    /// A property whose cardinality is at most one appears more than once.
    DuplicateProperty(String),
    /// The vCard does not contain any FN property.
    MissingFormattedName,
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::ExpectedBegin => f.write_str("expected a BEGIN:VCARD line"),
            Self::ExpectedVersion => {
                f.write_str("expected a VERSION property right after BEGIN:VCARD")
            },
            Self::UnsupportedVersion(version) => {
                write!(f, "the vCard version {version:?} is not supported")
            },
            Self::MissingEnd => f.write_str("missing the END:VCARD line"),
            Self::TrailingData => f.write_str("unexpected content after END:VCARD"),
            Self::InvalidLine => f.write_str("the line is not a well-formed content line"),
            Self::InvalidGroupName => f.write_str("the group name is invalid"),
            Self::InvalidParameter(name) => write!(f, "the parameter {name} is invalid"),
            Self::InvalidValue {
                property,
            } => write!(f, "the value of the property {property} is invalid"),
            Self::DuplicateProperty(name) => {
                write!(f, "the property {name} appears more than once")
            },
            Self::MissingFormattedName => f.write_str("the vCard does not have any FN property"),
        }
    }
}

impl Error for ParseErrorKind {}
