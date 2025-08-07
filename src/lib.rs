#[cfg(feature = "timeconversions")]
extern crate chrono;

extern crate thiserror;

#[macro_use]
pub mod param;
#[macro_use]
mod util;

pub mod component;
pub mod error;
pub mod icalendar;
mod parser;
pub mod property;
pub mod vcard;

pub use component::parse_component;
pub use component::read_component;
pub use component::write_component;
pub use component::Component;
pub use property::escape_chars;
pub use property::unescape_chars;
pub use property::Property;

pub use icalendar::ICalendar;
pub use vcard::Vcard;
