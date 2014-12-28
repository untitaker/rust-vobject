// DOCS

#![crate_name = "vobject"]
#![crate_type = "lib"]
#![license = "MIT"]
#![comment = "Parser for VObject and iCalendar."]

#![feature(phase)]
#[phase(plugin)]
extern crate peg_syntax_ext;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};



pub struct Property {
    params: HashMap<String, String>,
    raw_value: String,
}

impl Property {
    fn new(params: HashMap<String, String>, raw_value: String) -> Property {
        Property {
            params: params,
            raw_value: raw_value
        }
    }

    #[doc="Get parameters."]
    pub fn get_params(&self) -> &HashMap<String, String> {
        &self.params
    }

    #[doc="Get value as unparsed string."]
    pub fn get_raw_value(&self) -> &String {
        &self.raw_value
    }

    #[doc="Get value as unescaped string."]
    pub fn value_as_string(&self) -> String {
        unescape_chars(self.get_raw_value())
    }
}


pub struct Component {
    #[doc="The name of the component, such as `VCARD` or `VEVENT`."]
    pub name: String,

    #[doc="The component's properties."]
    pub props: HashMap<String, Vec<Property>>,

    #[doc="The component's child- or sub-components."]
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

    #[doc="Retrieve one property (from many) by key.
        Returns `None` if nothing is found."]
    pub fn single_prop(&self, key: &String) -> Option<&Property> {
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

    #[doc="Retrieve a mutable vector of properties for this key.
        Creates one (and inserts it into the component) if none exists."]
    pub fn all_props_mut(&mut self, key: String) -> &mut Vec<Property> {
        match self.props.entry(key) {
            Occupied(values) => values.into_mut(),
            Vacant(values) => values.set(vec![])
        }
    }

    #[doc="Retrieve properties by key.
        Returns an empty slice if key doesn't exist."]
    pub fn all_props(&self, key: &String) -> &[Property] {
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
    = "BEGIN:" v:value __ { v }

component_end -> String
    = "END:" v:value __ { v }

components -> Vec<Component>
    = cs:component ++ eols __ { cs }

props -> Vec<(String, Property)>
    = ps:contentline ++ eols __ { ps }

contentline -> (String, Property)
    = k:name p:params ":" v:value {
        (k, Property::new(p, v))
    }

name -> String
    = !"BEGIN" !"END" iana_token+ { match_str.into_string() }

params -> HashMap<String, String>
    = ps:(";" p:param {p})* {
        let mut rv: HashMap<String, String> = HashMap::new();
        for (k, v) in ps.into_iter() {
            rv.insert(k, v);
        };
        rv
    }

param -> (String, String)
    // FIXME: Doesn't handle comma-separated values
    = k:param_name v:("=" v:param_value { v })? {
        (k, match v {
            Some(x) => x,
            None => "".into_string()
        })
    }

param_name -> String
    = iana_token+ { match_str.into_string() }

param_value -> String
    = x:(quoted_string / param_text) { x }

param_text -> String
    = safe_char* { match_str.into_string() }

value -> String
    = value_char+ { match_str.into_string() }


quoted_string -> String
    = dquote x:quoted_content dquote { x }

quoted_content -> String
    = qsafe_char* { match_str.into_string() }

iana_token = ([a-zA-Z0-9] / "-")+
safe_char = !";" !":" !"," value_char
qsafe_char = !dquote value_char // FIXME

value_char = !eol .

eol = "\n" / "\r\n" / "\r"
dquote = "\""
eols = eol+
whitespace = " " / "\t"
__ = (eol / whitespace)*

"#);


#[doc="Parse a component. The error value is a human-readable message."]
pub fn parse_component(s: &String) -> Result<Component, String> {
    // XXX: The unfolding should be worked into the PEG
    // See feature request: https://github.com/kevinmehall/rust-peg/issues/26
    let unfolded = s
        .replace("\n ", "").replace("\n\t", "")
        .replace("\r\n ", "").replace("\r\n\t", "")
        .replace("\r ", "").replace("\r\t", "");

    parser::component(unfolded.as_slice())
}

#[doc="Escape text for a VObject property value."]
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

#[doc="Unescape text from a VObject property value."]
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
