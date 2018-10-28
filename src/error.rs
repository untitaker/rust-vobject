#[derive(Debug, Clone, Eq, PartialEq, Fail)]
pub enum VObjectErrorKind {
    #[fail(display = "Parser error: {}", _0)]
    ParserError(String),

    #[fail(display = "Not a Vcard")]
    NotAVCard,

    #[fail(display = "Not a Icalendar: {}", _0)]
    NotAnICalendar(String),

    #[fail(display = "{}", _0)]
    ChronoError(::chrono::format::ParseError),
}

pub type Result<T> = ::std::result::Result<T, VObjectErrorKind>;
