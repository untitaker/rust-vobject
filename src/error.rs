use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum VObjectErrorKind {
    #[error("Parser error: {}", _0)]
    ParserError(String),

    #[error("Not a Vcard")]
    NotAVCard,

    #[error("Not a Icalendar: {}", _0)]
    NotAnICalendar(String),

    #[cfg(feature = "timeconversions")]
    #[error("{}", _0)]
    ChronoError(::chrono::format::ParseError),
}

pub(crate) type VObjectResult<T> = Result<T, VObjectErrorKind>;
