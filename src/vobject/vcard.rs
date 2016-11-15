//! vcard convenience functionality module

use std::ops::{Deref, DerefMut};

use vobject::Component;

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
    pub fn tels<I: Iterator<Item = Component>>(&self) -> Tels<I> {
        unimplemented!()
    }

    /// Get the emails from the vcard object
    pub fn emails<I: Iterator<Item = Component>>(&self) -> Emails<I> {
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
