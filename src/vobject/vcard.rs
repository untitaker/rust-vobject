//! vcard convenience functionality module

use std::ops::{Deref, DerefMut};

use std::error::Error;
use std::fmt::{Display, Formatter, Error as FmtError};

use Component;
use Property;

#[derive(Debug, Clone)]
pub enum VcardError {
    TypeMissing,
}

impl Display for VcardError {
     fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
         write!(fmt, "{}", self.description())
     }
}

impl Error for VcardError {
    fn description(&self) -> &str {
        match *self {
            VcardError::TypeMissing => "Type missing",
        }
    }
}

pub struct Vcard(Component);

impl Vcard {

    /// Create a new Vcard object
    pub fn new() -> Vcard {
        Vcard::new_with_version("3.0")
    }

    /// Create a new Vcard object with a given version
    pub fn new_with_version(version: &str) -> Vcard {
        let mut comp = Component::new("VCARD");
        comp.all_props_mut("VERSION").push(Property::new("VERSION", version));

        Vcard(comp)
    }

    /// Helper to get a single property as string
    fn prop_as_str(&self, propname: &str) -> Option<String> {
        self.0.single_prop(propname).map(|prop| prop.value_as_string())
    }

    /// Helper to get multiple properties as string
    fn props_as_str(&self, propname: &str) -> Vec<String> {
        Vec::from(self.0.all_props(propname))
            .into_iter()
            .map(|prop| prop.value_as_string())
            .collect()
    }

    /// Get the version of the vcard object (unescaped variant)
    pub fn unescaped_version(&self) -> Option<String> {
        self.prop_as_str("VERSION")
    }

    /// Get the uid of the vcard object (unescaped variant)
    pub fn unescaped_uid(&self) -> Option<String> {
        self.prop_as_str("UID")
    }

    /// Get the rev of the vcard object (unescaped variant)
    pub fn unescaped_rev(&self) -> Option<String> {
        self.prop_as_str("REV")
    }

    /// Get the name from the vcard object (unescaped variant)
    pub fn unescaped_name(&self) -> Option<String> {
        self.prop_as_str("N")
    }

    /// Get the fullname from the vcard object (unescaped variant)
    pub fn unescaped_fullname(&self) -> Option<String> {
        self.prop_as_str("FN")
    }

    /// Get the tel from the vcard object (unescaped variant)
    pub fn unescaped_prodid(&self) -> Option<String> {
        self.prop_as_str("PRODID")
    }

    /// Get the tel from the vcard object (unescaped variant)
    pub fn unescaped_tels(&self) -> Vec<String> {
        self.props_as_str("TEL")
    }

    /// Get the emails from the vcard object (unescaped variant)
    pub fn unescaped_emails(&self) -> Vec<String> {
        self.props_as_str("EMAIL")
    }

    /// Get the version of the vcard object
    pub fn version(&self) -> Option<Version> {
        unimplemented!()
    }

    /// Get the uid of the vcard object
    pub fn uid(&self) -> Option<Uid> {
        unimplemented!()
    }

    /// Get the rev of the vcard object
    pub fn rev(&self) -> Option<Rev> {
        unimplemented!()
    }

    /// Get the name from the vcard object
    pub fn name(&self) -> Option<Name> {
        unimplemented!()
    }

    /// Get the fullname from the vcard object
    pub fn fullname(&self) -> Option<Fullname> {
        unimplemented!()
    }

    /// Get the tel from the vcard object
    pub fn prodid(&self) -> Option<ProdId> {
        unimplemented!()
    }

    /// Get the tel from the vcard object
    pub fn tels<I: Iterator<Item = Property>>(&self) -> Tels {
        Tels::from(self.0.all_props("TEL").to_owned())
    }

    /// Get the emails from the vcard object
    pub fn emails<I: Iterator<Item = Property>>(&self) -> Emails<I> {
        unimplemented!()
    }

    /// Set the fullname of the vcard object
    pub fn set_fullname(&mut self, fullname: String) {
        let mut props = self.0.all_props_mut("FN");
        let _ = props.pop();
        props.push(Property::new("FN", &fullname));
    }

    /// Add an email to the vcard object
    pub fn push_mail(&mut self, mail: String) {
        self.0.all_props_mut("EMAIL").push(Property::new("EMAIL", &mail))
    }

}

impl Deref for Vcard {
    type Target = Component;

    fn deref(&self) -> &Self::Target {
        &self.0
    }

}

impl DerefMut for Vcard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }

}

/// A type specifier as in `TEL;TYPE=PREF,CELL;...`
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Type(String);

impl Deref for Type {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Type {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<String> for Type {
    fn from(s: String) -> Type {
        Type(s)
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Version(String);

impl Deref for Version {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Version {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Uid(String);

impl Deref for Uid {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Uid {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Rev(String);

impl Deref for Rev {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Rev {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Name(String);

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Name {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Fullname(String);

impl Deref for Fullname {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Fullname {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ProdId(String);

impl Deref for ProdId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ProdId {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

use std::vec::IntoIter as VecIntoIter;

pub struct Tels(VecIntoIter<Property>);

impl Tels {

    pub fn new(ts: Vec<Property>) -> Tels {
        Tels(ts.into_iter())
    }

}

impl From<Vec<Property>> for Tels {

    fn from(v: Vec<Property>) -> Tels {
        Tels(v.into_iter())
    }

}

impl Iterator for Tels {
    type Item = Result<Tel, VcardError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(comp) => {
                let ty = match comp.params.get("TYPE") {
                    None => return Some(Err(VcardError::TypeMissing)),
                    Some(t) => t,
                };

                let types : Vec<Type> = ty.split(",").map(ToOwned::to_owned).map(Type::from).collect();
                Some(Ok(Tel::new(types, comp.raw_value)))
            },
            None => None,
        }
    }

}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tel {
    ty: Vec<Type>,
    num: String
}

impl Tel {

    pub fn new(types: Vec<Type>, num: String) -> Tel {
        Tel { ty: types, num: num }
    }
}

pub struct Emails<I: Iterator<Item = Property>>(I);

impl<I: Iterator<Item = Property>> Emails<I> {

    pub fn new(cs: Vec<Property>) -> Emails {
        Emails(cs.into_iter())
    }

}

impl<I: Iterator<Item = Property>> From<Vec<Property>> for Emails<I> {

    fn from(v: Vec<Property>) -> Emails<I> {
        Emails(v.into_iter())
    }

}

impl<I: Iterator<Item = Property>> Iterator for Emails<I> {
    type Item = Result<Email, VcardError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(comp) => {
                let ty = match comp.params.get("TYPE") {
                    None => return Some(Err(VcardError::TypeMissing)),
                   Some(t) => t,
                };

                let types : Vec<Type> = ty.split(",").map(ToOwned::to_owned).map(Type::from).collect();
                Some(Ok(Email::new(types, comp.raw_value)))
            },
            None => None,
        }
    }

}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Email {
    ty: Vec<Type>,
    num: String
}

impl Email {

    pub fn new(types: Vec<Type>, num: String) -> Email {
        Email { ty: types, num: num }
    }

}

