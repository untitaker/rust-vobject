use std::result::Result as RResult;

use component::Component;
use component::parse_component;
use property::Property;
use param::Parameters;
use error::*;
use util::*;

#[cfg(feature = "timeconversions")]
use chrono::NaiveDateTime;

/// An Icalendar representing type
#[derive(Debug)]
pub struct Icalendar(Component);

impl Icalendar {

    /// Parse a string to a Icalendar object
    ///
    /// Returns an error if the parsed text is not a Icalendar (that means that an error is
    /// returned also if this is a valid vcard!)
    ///
    pub fn build(s: &str) -> Result<Icalendar> {
        parse_component(s)
            .and_then(|c| {
                Self::from_component(c)
                    .map_err(|_| {
                        let kind = VObjectErrorKind::NotAnIcalendar(s.to_owned());
                        VObjectError::from_kind(kind)
                    })
            })
    }

    /// Wrap a Component into a Vcard object, or don't do it if the Component is not a Vcard.
    pub fn from_component(c: Component)-> RResult<Icalendar, Component> {
        if c.name == "VCALENDAR" {
            Ok(Icalendar(c))
        } else {
            Err(c)
        }
    }

    /// Get an iterator over the events in this calendar
    ///
    /// The iterator creates Ok(&Event) instances on the fly, or Err(&Component) instances if the
    /// item cannot be parsed as an Event, not forgetting any data.
    ///
    /// # Getting actual objects
    ///
    /// For getting a Event-instance iterator from this, one can use this as follows:
    ///
    /// ```
    /// # use std::collections::BTreeMap;
    /// # use vobject::component::Component;
    /// # use vobject::icalendar::Event;
    /// # use vobject::icalendar::Icalendar;
    /// # let icalendar = Icalendar::from_component(Component {
    /// #     name:          "VCALENDAR".to_owned(),
    /// #     props:         BTreeMap::new(),
    /// #     subcomponents: vec![]
    /// # }).unwrap();
    /// icalendar
    ///     .events()
    ///     .filter_map(Result::ok)
    ///     .map(|ev| ev.clone())
    ///     .collect::<Vec<Event>>();
    /// ```
    ///
    pub fn events<'a>(&'a self) -> EventIterator<'a> {
        EventIterator::new(self.0.subcomponents.iter())
    }

    make_getter_function_for_optional!(get_version, "VERSION", Version);
    make_getter_function_for_optional!(get_prodid, "PRODID", Prodid);
}

create_data_type!(Version);
create_data_type!(Prodid);

pub struct EventIterator<'a>(::std::slice::Iter<'a, Component>);

impl<'a> EventIterator<'a> {
    fn new(i: ::std::slice::Iter<'a, Component>) -> EventIterator<'a> {
        EventIterator(i)
    }
}

impl<'a> Iterator for EventIterator<'a> {
    type Item = RResult<Event<'a>, &'a Component>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Event::from_component)
    }

}

#[derive(Debug, Clone)]
pub struct Event<'a>(&'a Component);

impl<'a> Event<'a> {
    fn from_component(c: &'a Component) -> RResult<Event<'a>, &'a Component> {
        if c.name == "VEVENT" {
            Ok(Event(c))
        } else {
            Err(c)
        }
    }

    make_getter_function_for_optional!(get_dtend       , "DTEND"       , Dtend);
    make_getter_function_for_optional!(get_dtstart     , "DTSTART"     , Dtstart);
    make_getter_function_for_optional!(get_dtstamp     , "DTSTAMP"     , Dtstamp);
    make_getter_function_for_optional!(get_uid         , "UID"         , Uid);
    make_getter_function_for_optional!(get_description , "DESCRIPTION" , Description);
    make_getter_function_for_optional!(get_summary     , "SUMMARY"     , Summary);
    make_getter_function_for_optional!(get_url         , "URL"         , Url);
    make_getter_function_for_optional!(get_location    , "LOCATION"    , Location);
    make_getter_function_for_optional!(get_class       , "CLASS"       , Class);
    make_getter_function_for_optional!(get_categories  , "CATEGORIES"  , Categories);
    make_getter_function_for_optional!(get_transp      , "TRANSP"      , Transp);
    make_getter_function_for_optional!(get_rrule       , "RRULE"       , Rrule);
}

create_data_type!(Dtend);
create_data_type!(Dtstart);
create_data_type!(Dtstamp);
create_data_type!(Uid);
create_data_type!(Description);
create_data_type!(Summary);
create_data_type!(Url);
create_data_type!(Location);
create_data_type!(Class);
create_data_type!(Categories);
create_data_type!(Transp);
create_data_type!(Rrule);

#[cfg(feature = "timeconversions")]
pub trait AsDateTime {
    fn as_datetime(&self) -> Result<NaiveDateTime>;
}

#[cfg(feature = "timeconversions")]
impl AsDateTime for Dtend {

    fn as_datetime(&self) -> Result<NaiveDateTime> {
        NaiveDateTime::parse_from_str(&self.0, DATE_TIME_FMT).map_err(From::from)
    }

}

#[cfg(feature = "timeconversions")]
impl AsDateTime for Dtstart {

    fn as_datetime(&self) -> Result<NaiveDateTime> {
        NaiveDateTime::parse_from_str(&self.0, DATE_TIME_FMT).map_err(From::from)
    }

}

#[cfg(feature = "timeconversions")]
impl AsDateTime for Dtstamp {

    fn as_datetime(&self) -> Result<NaiveDateTime> {
        NaiveDateTime::parse_from_str(&self.0, DATE_TIME_FMT).map_err(From::from)
    }

}

