use std::result::Result as RResult;

use component::Component;
use component::parse_component;
use property::Property;
use error::*;
use util::*;

#[cfg(feature = "timeconversions")]
use chrono::NaiveDateTime;

#[cfg(feature = "timeconversions")]
use chrono::NaiveDate;

/// An ICalendar representing type
#[derive(Debug)]
pub struct ICalendar(Component);

impl ICalendar {

    /// Parse a string to a ICalendar object
    ///
    /// Returns an error if the parsed text is not a ICalendar (that means that an error is
    /// returned also if this is a valid Vcard!)
    ///
    pub fn build(s: &str) -> Result<ICalendar> {
        let c = parse_component(s)?;
        Self::from_component(c)
            .map_err(|_| {
                let kind = VObjectErrorKind::NotAnICalendar(s.to_owned());
                VObjectError::from_kind(kind)
            })
    }

    /// Wrap a Component into a Vcard object, or don't do it if the Component is not a Vcard.
    pub fn from_component(c: Component)-> RResult<ICalendar, Component> {
        if c.name == "VCALENDAR" {
            Ok(ICalendar(c))
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
    /// # use vobject::icalendar::ICalendar;
    /// # let icalendar = ICalendar::from_component(Component {
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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Time {
    Date(NaiveDate),
    DateTime(NaiveDateTime),
}

#[cfg(feature = "timeconversions")]
pub trait AsDateTime {
    fn as_datetime(&self) -> Result<Time>;
}

#[cfg(feature = "timeconversions")]
impl AsDateTime for Dtend {

    fn as_datetime(&self) -> Result<Time> {
        match NaiveDateTime::parse_from_str(&self.0, DATE_TIME_FMT) {
            Ok(dt) => Ok(Time::DateTime(dt)),
            Err(_) => NaiveDate::parse_from_str(&self.0, DATE_FMT)
                .map(Time::Date)
                .map_err(From::from),
        }
    }

}

#[cfg(feature = "timeconversions")]
impl AsDateTime for Dtstart {

    fn as_datetime(&self) -> Result<Time> {
        match NaiveDateTime::parse_from_str(&self.0, DATE_TIME_FMT) {
            Ok(dt) => Ok(Time::DateTime(dt)),
            Err(_) => NaiveDate::parse_from_str(&self.0, DATE_FMT)
                .map(Time::Date)
                .map_err(From::from),
        }
    }

}

#[cfg(feature = "timeconversions")]
impl AsDateTime for Dtstamp {

    fn as_datetime(&self) -> Result<Time> {
        match NaiveDateTime::parse_from_str(&self.0, DATE_TIME_FMT) {
            Ok(dt) => Ok(Time::DateTime(dt)),
            Err(_) => NaiveDate::parse_from_str(&self.0, DATE_FMT)
                .map(Time::Date)
                .map_err(From::from),
        }
    }

}

#[cfg(all(test, feature = "timeconversions"))]
mod tests {
    use chrono::NaiveDate;
    use chrono::NaiveDateTime;
    use util::*;

    use super::*;

    const TEST_ENTRY : &'static str =
            "BEGIN:VCALENDAR\n\
            VERSION:2.0\n\
            PRODID:http://www.example.com/calendarapplication/\n\
            METHOD:PUBLISH\n\
            BEGIN:VEVENT\n\
            UID:461092315540@example.com\n\
            ORGANIZER;CN=\"Alice Balder, Example Inc.\":MAILTO:alice@example.com\n\
            LOCATION:Somewhere\n\
            SUMMARY:Eine Kurzinfo\n\
            DESCRIPTION:Beschreibung des Termines\n\
            CLASS:PUBLIC\n\
            DTSTART:20060910T220000Z\n\
            DTEND:20060919T215900Z\n\
            DTSTAMP:20060812T125900Z\n\
            END:VEVENT\n\
            END:VCALENDAR\n";

    const TEST_ENTRY_OC : &'static str = // Lets see how owncloud foo works here
        "BEGIN:VCALENDAR\n\
        VERSION:2.0\n\
        PRODID:ownCloud Calendar\n\
        CALSCALE:GREGORIAN\n\
        BEGIN:VEVENT\n\
        UID:ff411055a5\n\
        DTSTAMP:20160128T223013Z\n\
        CREATED:20160128T223013Z\n\
        LAST-MODIFIED:20160128T223013Z\n\
        SUMMARY:Amon Amarth - Jomsviking\n\
        DTSTART;VALUE=DATE:20160325\n\
        DTEND;VALUE=DATE:20160326\n\
        LOCATION:\n\
        DESCRIPTION:\n\
        CATEGORIES:\n\
        END:VEVENT\n\
        END:VCALENDAR\n\
        ";

    #[test]
    fn test_parse() {
        let cal = ICalendar::build(TEST_ENTRY);
        assert!(cal.is_ok(), "Not okay: {:?}\n in '{}'", cal, TEST_ENTRY);
    }

    #[test]
    fn test_iter() {
        let ical = ICalendar::build(TEST_ENTRY).unwrap();
        assert_eq!(ical.events().count(), 1);
    }

    #[test]
    fn test_icalendar_attributes() {
        let ical = ICalendar::build(TEST_ENTRY).unwrap();
        assert_eq!(ical.get_version().unwrap().raw(), "2.0");
        assert_eq!(ical.get_prodid().unwrap().raw(), "http://www.example.com/calendarapplication/");
    }

    #[test]
    fn test_event_attributes() {
        let ical = ICalendar::build(TEST_ENTRY).unwrap();
        let ev = ical.events().next().unwrap().unwrap();
        assert_eq!(ev.get_dtend().map(|e| e.raw().clone())       , Some("20060919T215900Z".to_owned()));
        assert_eq!(ev.get_dtstart().map(|e| e.raw().clone())     , Some("20060910T220000Z".to_owned()));
        assert_eq!(ev.get_dtstamp().map(|e| e.raw().clone())     , Some("20060812T125900Z".to_owned()));
        assert_eq!(ev.get_uid().map(|e| e.raw().clone())         , Some("461092315540@example.com".to_owned()));
        assert_eq!(ev.get_description().map(|e| e.raw().clone()) , Some("Beschreibung des Termines".to_owned()));
        assert_eq!(ev.get_summary().map(|e| e.raw().clone())     , Some("Eine Kurzinfo".to_owned()));
        assert_eq!(ev.get_url()                                  , None);
        assert_eq!(ev.get_location().map(|e| e.raw().clone())    , Some("Somewhere".to_owned()));
        assert_eq!(ev.get_class().map(|e| e.raw().clone())       , Some("PUBLIC".to_owned()));
        assert_eq!(ev.get_categories()                           , None);
        assert_eq!(ev.get_transp()                               , None);
        assert_eq!(ev.get_rrule()                                , None);
    }

    #[test]
    fn test_event_attributes_oc() {
        let ical = ICalendar::build(TEST_ENTRY_OC).unwrap();
        assert_eq!(ical.get_version().unwrap().raw(), "2.0");
        assert_eq!(ical.get_prodid().unwrap().raw(), "ownCloud Calendar");
        let ev = ical.events().next().unwrap().unwrap();
        assert_eq!(ev.get_dtend().map(|e| e.raw().clone())       , Some("20160326".to_owned()));
        assert_eq!(ev.get_dtstart().map(|e| e.raw().clone())     , Some("20160325".to_owned()));
        assert_eq!(ev.get_dtstamp().map(|e| e.raw().clone())     , Some("20160128T223013Z".to_owned()));
        assert_eq!(ev.get_uid().map(|e| e.raw().clone())         , Some("ff411055a5".to_owned()));
        assert_eq!(ev.get_description().map(|e| e.raw().clone()) , Some("".to_owned()));
        assert_eq!(ev.get_summary().map(|e| e.raw().clone())     , Some("Amon Amarth - Jomsviking".to_owned()));
        assert_eq!(ev.get_url()                                  , None);
        assert_eq!(ev.get_location().map(|e| e.raw().clone())    , Some("".to_owned()));
        assert_eq!(ev.get_class().map(|e| e.raw().clone())       , None);
        assert_eq!(ev.get_categories().map(|e| e.raw().clone())  , Some("".to_owned()));
        assert_eq!(ev.get_transp()                               , None);
        assert_eq!(ev.get_rrule()                                , None);
    }

    #[cfg(feature = "timeconversions")]
    #[test]
    fn test_event_attributes_with_conversions() {
        let ical = ICalendar::build(TEST_ENTRY).unwrap();
        let ev = ical.events().next().unwrap().unwrap();
        assert_eq!(ev.get_dtend().map(|e| e.as_datetime().unwrap()).unwrap(), Time::DateTime(NaiveDateTime::parse_from_str("20060919T215900Z", DATE_TIME_FMT).unwrap()));
        assert_eq!(ev.get_dtstart().map(|e| e.as_datetime().unwrap()).unwrap(), Time::DateTime(NaiveDateTime::parse_from_str("20060910T220000Z", DATE_TIME_FMT).unwrap()));
        assert_eq!(ev.get_dtstamp().map(|e| e.as_datetime().unwrap()).unwrap(), Time::DateTime(NaiveDateTime::parse_from_str("20060812T125900Z", DATE_TIME_FMT).unwrap()));
    }

    #[cfg(feature = "timeconversions")]
    #[test]
    fn test_event_attributes_oc_with_conversions() {
        let ical = ICalendar::build(TEST_ENTRY_OC).unwrap();
        assert_eq!(ical.get_version().unwrap().raw(), "2.0");
        assert_eq!(ical.get_prodid().unwrap().raw(), "ownCloud Calendar");
        let ev = ical.events().next().unwrap().unwrap();
        assert_eq!(ev.get_dtend().map(|e| e.as_datetime().unwrap()).unwrap(), Time::Date(NaiveDate::parse_from_str("20160326", DATE_FMT).unwrap()));
        assert_eq!(ev.get_dtstart().map(|e| e.as_datetime().unwrap()).unwrap(), Time::Date(NaiveDate::parse_from_str("20160325", DATE_FMT).unwrap()));
        assert_eq!(ev.get_dtstamp().map(|e| e.as_datetime().unwrap()).unwrap(), Time::DateTime(NaiveDateTime::parse_from_str("20160128T223013Z", DATE_TIME_FMT).unwrap()));
    }

}
