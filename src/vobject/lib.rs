// DOCS

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::borrow::ToOwned;
use std::str::FromStr;
use std::fmt;
use std::error::Error;


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
    pub fn new(name: &str, value: &str) -> Property {
        Property {
            name: name.to_owned(),
            params: HashMap::new(),
            raw_value: escape_chars(value),
            prop_group: None
        }
    }

    /// Get value as unescaped string.
    pub fn value_as_string(&self) -> String {
        unescape_chars(&self.raw_value[..])
    }
}

impl Clone for Property {
    fn clone(&self) -> Self {
        Property {
            name: self.name.clone(),
            params: self.params.clone(),
            raw_value: self.raw_value.clone(),
            prop_group: self.prop_group.clone()
        }
    }
}

pub struct Component {
    /// The name of the component, such as `VCARD` or `VEVENT`.
    pub name: String,

    /// The component's properties.
    pub props: HashMap<String, Vec<Property>>,

    /// The component's child- or sub-components.
    pub subcomponents: Vec<Component>
}

impl Component {
    pub fn new<T: Into<String>>(name: T) -> Component {
        Component {
            name: name.into(),
            props: HashMap::new(),
            subcomponents: vec![]
        }
    }

    /// Retrieve one property (from many) by key. Returns `None` if nothing is found.
    pub fn single_prop(&self, key: &str) -> Option<&Property> {
        match self.props.get(key) {
            Some(x) => {
                match x.len() {
                    1 => Some(&x[0]),
                    _ => None
                }
            },
            None => None
        }
    }

    /// Retrieve a mutable vector of properties for this key. Creates one (and inserts it into the
    /// component) if none exists.
    pub fn all_props_mut<T: Into<String>>(&mut self, key: T) -> &mut Vec<Property> {
        match self.props.entry(key.into()) {
            Occupied(values) => values.into_mut(),
            Vacant(values) => values.insert(vec![])
        }
    }

    /// Retrieve properties by key. Returns an empty slice if key doesn't exist.
    pub fn all_props(&self, key: &str) -> &[Property] {
        static EMPTY: &'static [Property] = &[];
        match self.props.get(key) {
            Some(values) => &values[..],
            None => EMPTY
        }
    }
}

impl FromStr for Component {
    type Err = ParseError;

    /// Same as `vobject::parse_component`, but without the error messages.
    fn from_str(s: &str) -> ParseResult<Component> {
        parse_component(s)
    }
}



struct Parser<'s> {
    pub input: &'s str,
    pub pos: usize,
}

impl<'s> Parser<'s> {
    pub fn new<'a>(input: &'a str) -> Parser<'a> {
        Parser {
            input: input,
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn assert_char(&self, c: char) -> ParseResult<()> {
        let real_c = match self.peek() {
            Some(x) => x,
            None => return Err(ParseError::new(format!("Expected {}, found EOL", c))),
        };

        if real_c != c {
            return Err(ParseError::new(format!("Expected {}, found {}", c, real_c)))
        };

        Ok(())
    }

    fn consume_char(&mut self) -> Option<char> {
        match self.peek() {
            Some(x) => { self.pos += x.len_utf8(); Some(x) },
            None => None
        }
    }

    fn consume_eol(&mut self) -> ParseResult<()> {
        
        let start_pos = self.pos;

        let consumed = match self.consume_char() {
            Some('\n') => true,
            Some('\r') => match self.consume_char() {
                Some('\n') => true,
                _ => false,
            },
            _ => false,
        };
            
        if consumed {
            Ok(())
        } else {
            self.pos = start_pos;
            Err(ParseError::new("Expected EOL."))
        }
    }

    fn sloppy_terminate_line(&mut self) -> ParseResult<()> {
        if !self.eof() {
            try!(self.consume_eol());
            while let Ok(_) = self.consume_eol() {};
        };

        Ok(())
    }

    fn consume_while<'a, F: Fn(char) -> bool>(&'a mut self, test: F) -> &'a str {
        let start_pos = self.pos;
        while !self.eof() && test(self.peek().unwrap()) {
            self.consume_char();
        }
        &self.input[start_pos..self.pos]
    }

    pub fn consume_property(&mut self) -> ParseResult<Property> {
        let group = self.consume_property_group().ok();
        let name = try!(self.consume_property_name());
        let params = self.consume_params();

        try!(self.assert_char(':'));
        self.consume_char();

        let value = try!(self.consume_property_value());

        Ok(Property {
            name: name,
            params: params,
            raw_value: value,
            prop_group: group,
        })
    }

    fn consume_property_name<'a>(&'a mut self) -> ParseResult<String> {
        let rv = self.consume_while(|x| x == '-' || x.is_alphanumeric());
        if rv.len() == 0 {
            Err(ParseError::new("No property name found."))
        } else {
            Ok(rv.to_owned())
        }
    }

    fn consume_property_group<'a>(&'a mut self) -> ParseResult<String> {
        let start_pos = self.pos;
        let name = self.consume_property_name();

        let e = match name {
            Ok(name) => match self.assert_char('.') {
                Ok(_) => {
                    self.consume_char();
                    return Ok(name);
                },
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        };

        self.pos = start_pos;
        e
    }

    fn consume_property_value<'a>(&'a mut self) -> ParseResult<String> {
        let mut rv = String::new();
        loop {
            rv.push_str(self.consume_while(|x| x != '\r' && x != '\n'));
            try!(self.sloppy_terminate_line());

            match self.peek() {
                Some(' ') | Some('\t') => self.consume_char(),
                _ => break,
            };
        }
        Ok(rv)
    }

    fn consume_param_name<'a>(&'a mut self) -> ParseResult<String> {
        match self.consume_property_name() {
            Ok(x) => Ok(x),
            Err(e) => Err(ParseError::new(format!("No param name found: {}", e))),
        }
    }

    fn consume_param_value<'a>(&'a mut self) -> ParseResult<String> {
        let qsafe = |x| {
            x != '"' &&
            x != '\r' &&
            x != '\n' &&
            x != '\u{7F}' &&
            x > '\u{1F}'
        };

        if self.peek() == Some('"') {
            self.consume_char();
            let rv = self.consume_while(qsafe).to_owned();
            try!(self.assert_char('"'));
            self.consume_char();
            Ok(rv)
        } else {
            Ok(self.consume_while(|x| qsafe(x) && x != ';' && x != ':').to_owned())
        }
    }

    fn consume_param<'a>(&'a mut self) -> ParseResult<(String, String)> {
        let name = try!(self.consume_param_name());
        let value = if self.peek() == Some('=') {
            let start_pos = self.pos;
            self.consume_char();
            match self.consume_param_value() {
                Ok(x) => x,
                Err(e) => { self.pos = start_pos; return Err(e); }
            }
        } else {
            String::new()
        };

        Ok((name, value))
    }

    fn consume_params(&mut self) -> HashMap<String, String> {
        let mut rv: HashMap<String, String> = HashMap::new();
        while self.peek() == Some(';') {
            self.consume_char();
            match self.consume_param() {
                Ok((name, value)) => { rv.insert(name.to_owned(), value.to_owned()); },
                Err(_) => break,
            }
        };
        rv
    }

    fn consume_component(&mut self) -> ParseResult<Component> {
        let begin_pos = self.pos;
        let mut property = try!(self.consume_property());
        if property.name != "BEGIN" {
            self.pos = begin_pos;
            return Err(ParseError::new("Expected BEGIN tag."));
        };

        let c_name = property.raw_value;
        let mut component = Component::new(&c_name[..]);

        loop {
            let previous_pos = self.pos;
            property = try!(self.consume_property());
            if property.name == "BEGIN" {
                self.pos = previous_pos;
                while let Ok(subcomponent) = self.consume_component() {
                    component.subcomponents.push(subcomponent);
                };
            } else if property.name == "END" {
                if property.raw_value != c_name {
                    self.pos = begin_pos;
                    return Err(ParseError::new(format!(
                        "Mismatched tags: BEGIN:{} vs END:{}",
                        c_name, property.raw_value
                    )));
                };

                break;
            } else {
                component.all_props_mut(property.name.to_owned()).push(property);
            }
        };

        Ok(component)
    }
}

/// Parse exactly one component. Trailing data generates errors.
pub fn parse_component(s: &str) -> ParseResult<Component> {
    let mut parser = Parser::new(s);
    let rv = try!(parser.consume_component());
    if !parser.eof() {
        Err(ParseError::new(format!("Trailing data: `{}`", &parser.input[parser.pos..])))
    } else {
        Ok(rv)
    }
}

/// Write a component. The error value is a human-readable message.
pub fn write_component(c: &Component) -> String {
    fn inner(buf: &mut String, c: &Component) {
        buf.push_str("BEGIN:");
        buf.push_str(&c.name[..]);
        buf.push_str("\r\n");

        for (prop_name, props) in c.props.iter() {
            for prop in props.iter() {
                match prop.prop_group {
                    Some(ref x) => { buf.push_str(&x[..]); buf.push('.'); },
                    None => ()
                };
                buf.push_str(&prop_name[..]);
                for (param_key, param_value) in prop.params.iter() {
                    buf.push(';');
                    buf.push_str(&param_key[..]);
                    buf.push('=');
                    buf.push_str(&param_value[..]);
                };
                buf.push(':');
                buf.push_str(&fold_line(&prop.raw_value[..])[..]);
                buf.push_str("\r\n");
            };
        };

        for subcomponent in c.subcomponents.iter() {
            inner(buf, subcomponent);
        };

        buf.push_str("END:");
        buf.push_str(&c.name[..]);
        buf.push_str("\r\n");
    }

    let mut buf = String::new();
    inner(&mut buf, c);
    buf
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

/// Fold contentline to 75 chars. This function assumes the input to be unfolded, which means no
/// '\n' or '\r' in it.
pub fn fold_line(s: &str) -> String {
    let mut rv = String::new();
    for (i, c) in s.chars().enumerate() {
        rv.push(c);
        if i != 0 && i % 75 == 0 {
            rv.push_str("\r\n ");
        };
    };
    rv
}

#[derive(PartialEq, Eq, Debug)]
pub struct ParseError {
    desc: String
}

pub type ParseResult<T> = Result<T, ParseError>;

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.desc[..]
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl ParseError {
    pub fn new<T: Into<String>>(desc: T) -> Self {
        ParseError {
            desc: desc.into(),
        }
    }

    pub fn into_string(self) -> String {
        self.desc
    }
}
