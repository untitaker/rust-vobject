use std::str::FromStr;
use std::collections::HashMap;

use property::Property;
use parser::Parser;
use error::*;

#[derive(Clone, Debug)]
pub struct Component {
    /// The name of the component, such as `VCARD` or `VEVENT`.
    pub name: String,

    /// The component's properties.
    pub props: HashMap<String, Vec<Property>>,

    /// The component's child- or sub-components.
    pub subcomponents: Vec<Component>
}

impl Component {
    pub fn new<N: Into<String>>(name: N) -> Component {
        Component {
            name: name.into(),
            props: HashMap::new(),
            subcomponents: vec![]
        }
    }

    /// Append the given property, preserve other same-named properties.
    pub fn push(&mut self, prop: Property) {
        self.props.entry(prop.name.clone()).or_insert_with(Vec::new).push(prop);
    }

    /// Set the given property, remove other same-named properties.
    pub fn set(&mut self, prop: Property) {
        self.props.insert(prop.name.clone(), vec![prop]);
    }

    /// Retrieve one property by key. Returns `None` if not exactly one property was found.
    pub fn get_only<P: AsRef<str>>(&self, name: P) -> Option<&Property> {
        match self.props.get(name.as_ref()) {
            Some(x) if x.len() == 1 => Some(&x[0]),
            _ => None
        }
    }

    /// Retrieve properties by key. Returns an empty slice if key doesn't exist.
    pub fn get_all<P: AsRef<str>>(&self, name: P) -> &[Property] {
        static EMPTY: &'static [Property] = &[];
        match self.props.get(name.as_ref()) {
            Some(values) => &values[..],
            None => EMPTY
        }
    }

    /// Remove a single property.
    pub fn pop<P: AsRef<str>>(&mut self, name: P) -> Option<Property> {
        match self.props.get_mut(name.as_ref()) {
            Some(values) => values.pop(),
            None => None
        }
    }

    /// Remove all properties
    pub fn remove<P: AsRef<str>>(&mut self, name: P) -> Option<Vec<Property>> {
        self.props.remove(name.as_ref())
    }
}

impl FromStr for Component {
    type Err = VObjectError;

    /// Same as `vobject::parse_component`
    fn from_str(s: &str) -> Result<Component> {
        parse_component(s)
    }
}

/// Parse exactly one component. Trailing data generates errors.
pub fn parse_component(s: &str) -> Result<Component> {
    let mut parser = Parser::new(s);
    let rv = try!(parser.consume_component());
    if !parser.eof() {
        let s = format!("Trailing data: `{}`", &parser.input[parser.pos..]);
        let kind = VObjectErrorKind::ParserError(s);
        Err(VObjectError::from_kind(kind))
    } else {
        Ok(rv)
    }
}

/// Write a component to a String.
pub fn write_component(c: &Component) -> String {
    fn inner(buf: &mut String, c: &Component) {
        buf.push_str("BEGIN:");
        buf.push_str(&c.name);
        buf.push_str("\r\n");

        for (prop_name, props) in &c.props {
            for prop in props.iter() {
                if let Some(ref x) = prop.prop_group {
                    buf.push_str(&x);
                    buf.push('.');
                };
                buf.push_str(&prop_name);
                for (param_key, param_value) in &prop.params {
                    buf.push(';');
                    buf.push_str(&param_key);
                    buf.push('=');
                    buf.push_str(&param_value);
                }
                buf.push(':');
                buf.push_str(&fold_line(&prop.raw_value));
                buf.push_str("\r\n");
            }
        }

        for subcomponent in &c.subcomponents {
            inner(buf, subcomponent);
        }

        buf.push_str("END:");
        buf.push_str(&c.name);
        buf.push_str("\r\n");
    }

    let mut buf = String::new();
    inner(&mut buf, c);
    buf
}

/// Fold contentline to 75 bytes or less. This function assumes the input
/// to be unfolded, which means no '\n' or '\r' in it.
pub fn fold_line(line: &str) -> String {
    let limit = 75;
    let len = line.len();
    let mut bytes_remaining = len;
    let mut ret = String::with_capacity(len + (len / limit * 3));

    let mut pos = 0;
    let mut next_pos = limit;
    while bytes_remaining > limit {
        while line.is_char_boundary(next_pos) == false {
            next_pos -= 1;
        }
        ret.push_str(&line[pos..next_pos]);
        ret.push_str("\r\n ");

        bytes_remaining -= next_pos - pos;
        pos = next_pos;
        next_pos += limit;
    }

    ret.push_str(&line[len - bytes_remaining..]);
    ret
}


#[cfg(test)]
mod tests {
    use component::fold_line;

    #[test]
    fn test_fold() {
        let line = "This should be multiple lines and fold on char boundaries. 毎害止\
                   加食下組多地将写館来局必第。東証細再記得玲祉込吉宣会法授";
        let expected = "This should be multiple lines and fold on char boundaries. 毎害止\
                       加食\r\n 下組多地将写館来局必第。東証細再記得玲祉込吉宣会法\r\n 授";
        assert_eq!(expected, fold_line(line));
        assert_eq!("ab", fold_line("ab"));
    }

}
