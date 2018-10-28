#[macro_use]
extern crate failure;

#[cfg(feature = "timeconversions")]
extern crate chrono;

#[macro_use] pub mod param;
#[macro_use] mod util;

pub mod component;
pub mod error;
mod parser;
pub mod property;
pub mod vcard;
pub mod icalendar;

pub use component::Component;
pub use component::parse_component;
pub use component::read_component;
pub use component::write_component;
pub use property::Property;
pub use property::escape_chars;
pub use property::unescape_chars;

pub use vcard::Vcard;
pub use icalendar::ICalendar;
