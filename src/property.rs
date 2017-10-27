use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Property {
    /// Key in component.
    pub name: String,

    /// Parameters.
    pub params: HashMap<String, String>,

    /// Value as unparsed string.
    pub raw_value: String,

    /// Property group. E.g. a contentline like `foo.FN:Markus` would result in the group being
    /// `"foo"`.
    pub prop_group: Option<String>
}

impl Property {
    /// Create property from unescaped string.
    pub fn new<N, V>(name: N, value: V) -> Property
        where N: Into<String>,
              V: AsRef<str>
    {
        Property {
            name: name.into(),
            params: HashMap::new(),
            raw_value: escape_chars(value.as_ref()),
            prop_group: None
        }
    }

    /// Get value as unescaped string.
    pub fn value_as_string(&self) -> String {
        unescape_chars(&self.raw_value)
    }
}

/// Escape text for a VObject property value.
pub fn escape_chars(s: &str) -> String {
    // Order matters! Lifted from icalendar.parser
    // https://github.com/collective/icalendar/
    s
        .replace("\\N", "\n")
        .replace("\\", "\\\\")
        .replace(";", "\\;")
        .replace(",", "\\,")
        .replace("\r\n", "\\n")
        .replace("\n", "\\n")
}

/// Unescape text from a VObject property value.
pub fn unescape_chars(s: &str) -> String {
    // Order matters! Lifted from icalendar.parser
    // https://github.com/collective/icalendar/
    s
        .replace("\\N", "\\n")
        .replace("\r\n", "\n")
        .replace("\\n", "\n")
        .replace("\\,", ",")
        .replace("\\;", ";")
        .replace("\\\\", "\\")
}

