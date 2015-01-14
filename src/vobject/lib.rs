// DOCS

#![allow(unstable)]
#![feature(plugin)]
#[plugin] extern crate peg_syntax_ext;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};



pub struct Property {
    params: HashMap<String, String>,
    raw_value: String,
    prop_group: Option<String>
}

impl Property {
    fn new(params: HashMap<String, String>, raw_value: String, prop_group: Option<String>) -> Property {
        Property {
            params: params,
            raw_value: raw_value,
            prop_group: prop_group
        }
    }

    /// Get property group. E.g. a contentline like `foo.FN:Markus` would result in the group being
    /// `"foo"`.
    pub fn get_prop_group(&self) -> &Option<String> {
        &self.prop_group
    }

    /// Get parameters.
    pub fn get_params(&self) -> &HashMap<String, String> {
        &self.params
    }

    /// Get value as unparsed string.
    pub fn get_raw_value(&self) -> &String {
        &self.raw_value
    }

    /// Get value as unescaped string.
    pub fn value_as_string(&self) -> String {
        unescape_chars(self.get_raw_value())
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
    fn new(name: String) -> Component {
        Component {
            name: name,
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
    pub fn all_props_mut(&mut self, key: &str) -> &mut Vec<Property> {
        match self.props.entry(String::from_str(key)) {
            Occupied(values) => values.into_mut(),
            Vacant(values) => values.insert(vec![])
        }
    }

    /// Retrieve properties by key. Returns an empty slice if key doesn't exist.
    pub fn all_props(&self, key: &str) -> &[Property] {
        static EMPTY: &'static [Property] = &[];
        match self.props.get(key) {
            Some(values) => values.as_slice(),
            None => EMPTY
        }
    }
}


peg! parser(r#"
use super::{Component,Property};
use std::collections::HashMap;

#[pub]
component -> Component
    = name:component_begin
      ps:props
      cs:components?
      component_end {
        let mut rv = Component::new(name);

        match cs {
            Some(components) => { rv.subcomponents = components; },
            None => ()
        };

        for (k, v) in ps.into_iter() {
            rv.all_props_mut(k).push(v);
        };

        rv
    }

component_begin -> String
    = "BEGIN:" v:value __ { v.to_string() }

component_end -> String
    = "END:" v:value __ { v.to_string() }

components -> Vec<Component>
    = cs:component ++ eols __ { cs }

props -> Vec<(&'input str, Property)>
    = ps:prop ++ eols __ { ps }

prop -> (&'input str, Property)
    = !"BEGIN:" !"END:" g:group? k:name p:params ":" v:value {
        (k, Property::new(p, v, g))
    }

group -> String
    = g:group_name "." { g }

group_name -> String
    = group_char+ { match_str.to_string() }

name -> &'input str
    = iana_token+ { match_str }

params -> HashMap<String, String>
    = ps:(";" p:param {p})* {
        let mut rv: HashMap<String, String> = HashMap::new();
        for (k, v) in ps.into_iter() {
            rv.insert(k.to_string(), v.to_string());
        };
        rv
    }

param -> (&'input str, &'input str)
    // FIXME: Doesn't handle comma-separated values
    = k:param_name v:("=" v:param_value { v })? {
        (k, match v {
            Some(x) => x,
            None => ""
        })
    }

param_name -> &'input str
    = iana_token+ { match_str }

param_value -> &'input str
    = x:(quoted_string / param_text) { x }

param_text -> &'input str
    = safe_char* { match_str }

value -> String
    = value_char+ { match_str.to_string() }


quoted_string -> &'input str
    = dquote x:quoted_content dquote { x }

quoted_content -> &'input str
    = qsafe_char* { match_str }

iana_token = ([a-zA-Z0-9] / "-")+
group_char = ([a-zA-Z0-9] / "-")
qsafe_char = !dquote !ctl value_char
safe_char = !";" !":" qsafe_char

value_char = !eol .

eol = "\r\n" / "\n" / "\r"
dquote = "\""
eols = eol+
ctl =
    // "\x00-1F" / "\x7F"
    // FIXME: https://github.com/kevinmehall/rust-peg/issues/41
    "\x00" / "\x01" / "\x02" / "\x03" / "\x04" / "\x05" / "\x06" / "\x07" / "\x08" / "\t" / "\n" /
    "\x0b" / "\x0c" / "\r" / "\x0e" / "\x0f" / "\x10" / "\x11" / "\x12" / "\x13" / "\x14" / "\x15"
    / "\x16" / "\x17" / "\x18" / "\x19" / "\x1a" / "\x1b" / "\x1c" / "\x1d" / "\x1e" / "\x7f"

whitespace = " " / "\t"
__ = (eol / whitespace)*

"#);


/// Parse a component. The error value is a human-readable message.
pub fn parse_component(s: &String) -> Result<Component, String> {
    // XXX: The unfolding should be worked into the PEG
    // See feature request: https://github.com/kevinmehall/rust-peg/issues/26
    let unfolded = s
        .replace("\r\n ", "").replace("\r\n\t", "")
        .replace("\n ", "").replace("\n\t", "")
        .replace("\r ", "").replace("\r\t", "");

    parser::component(unfolded.as_slice())
}

/// Escape text for a VObject property value.
pub fn escape_chars(s: &String) -> String {
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
pub fn unescape_chars(s: &String) -> String {
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
