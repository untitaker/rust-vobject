use thiserror::Error;

use ::parser::ParseErrorReason;

#[derive(Debug, Clone, Error)]
pub enum VObjectErrorKind {
    #[error("failed to parse: {}", source)]
    Parse {
        #[from]
        source: ParseErrorReason,
    },

    #[error("Not a Vcard")]
    NotAVCard,

    #[error("Not a Icalendar: {}", _0)]
    NotAnICalendar(String),

    #[cfg(feature = "timeconversions")]
    #[error("failed to parse time")]
    ChronoError {
        #[from]
        source: chrono::format::ParseError,
    },
}

pub(crate) type VObjectResult<T> = Result<T, VObjectErrorKind>;
