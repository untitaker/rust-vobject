use std::collections::BTreeMap;

pub type Parameters = BTreeMap<String, String>;

#[macro_export]
macro_rules! parameters(
    { $($key:expr => $value:expr),* } => {
        #[allow(unused_mut)]
        {
            let mut m : ::std::collections::BTreeMap<String, String> =
                ::std::collections::BTreeMap::new();
            $( m.insert($key.into(), $value.into()); )*
            m
        }
     };
);
