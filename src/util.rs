macro_rules! make_getter_function_for_optional {
    ($fnname:ident, $name:expr, $mapper:ty) => {
        pub fn $fnname(&self) -> Option<$mapper> {
            self.0.get_only($name).cloned().map(From::from)
        }
    }
}

macro_rules! make_getter_function_for_values {
    ($fnname:ident, $name:expr, $mapper:ty) => {
        pub fn $fnname(&self) -> Vec<$mapper> {
            self.0
                .get_all($name)
                .iter()
                .map(Clone::clone)
                .map(From::from)
                .collect()
        }
    }
}

macro_rules! create_data_type {
    ( $name:ident ) => {
        #[derive(Eq, PartialEq, Debug)]
        pub struct $name(String, $crate::param::Parameters);

        impl $name {
            pub fn new(raw: String, params: $crate::param::Parameters) -> $name {
                $name(raw, params)
            }

            pub fn from_raw(raw: String) -> $name {
                $name(raw, BTreeMap::new())
            }

            pub fn raw(&self) -> &String {
                &self.0
            }

            pub fn into_raw(self) -> String {
                self.0
            }

            pub fn params(&self) -> &$crate::param::Parameters {
                &self.1
            }
        }

        impl From<Property> for $name {
            fn from(p: Property) -> $name {
                $name::new(p.raw_value, p.params)
            }
        }
    }
}

#[cfg(feature = "timeconversions")]
pub const DATE_TIME_FMT : &str = "%Y%m%dT%H%M%SZ";

#[cfg(feature = "timeconversions")]
pub const DATE_FMT      : &str = "%Y%m%d";

