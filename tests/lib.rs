#![allow(unstable)]
extern crate vobject;
use vobject::parse_component;
use std::borrow::ToOwned;

macro_rules! s(
    ($i:expr) => ($i.to_owned());
);


#[test]
fn test_vcard_basic() {
    let item = parse_component(
        "BEGIN:VCARD\n\
        VERSION:2.1\n\
        N:Mustermann;Erika\n\
        FN:Erika Mustermann\n\
        ORG:Wikipedia\n\
        TITLE:Oberleutnant\n\
        PHOTO;JPEG:http://commons.wikimedia.org/wiki/File:Erika_Mustermann_2010.jpg\n\
        TEL;WORK;VOICE:(0221) 9999123\n\n\n\
        TEL;HOME;VOICE:(0221) 1234567\n\
        ADR;HOME:;;Heidestrasse 17;Koeln;;51147;Deutschland\n\
        EMAIL;PREF;INTERNET:erika@mustermann.de\n\
        REV:20140301T221110Z\n\
        END:VCARD\n\r\n\n").unwrap();

    assert_eq!(item.single_prop("FN").unwrap().raw_value, s!("Erika Mustermann"));
    assert_eq!(item.single_prop("N").unwrap().raw_value,  s!("Mustermann;Erika"));

    let mut tel_values = item.all_props("TEL").iter().map(|x| x.raw_value.as_slice());
    assert_eq!(tel_values.next().unwrap(), s!("(0221) 9999123"));
    assert_eq!(tel_values.next().unwrap(), s!("(0221) 1234567"));
    assert!(tel_values.next().is_none());
}

#[test]
fn test_line_cont() {
    let item = parse_component(
        "BEGIN:VCARD\n\
        VERSION:2.1\n\
        N;ENCODING=QUOTED-PRINTABLE:Nikdo;Nikdo=\n\t\
        vic\n\
        NOTE:This ends with equal sign=\n\
        TEL;WORK:5555\n \
        4444\n\
        END:VCARD").unwrap();

    assert_eq!(item.name, s!("VCARD"));
    assert_eq!(item.single_prop("TEL").unwrap().raw_value, s!("55554444"));
    assert_eq!(item.single_prop("N").unwrap().raw_value, s!("Nikdo;Nikdo=vic"));
}

#[test]
fn test_icalendar_basic() {
    let item = parse_component(
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
            END:VCALENDAR\n").unwrap();

    assert_eq!(item.name, s!("VCALENDAR"));
    assert!(item.single_prop("LOCATION").is_none());
    assert!(item.single_prop("ORGANIZER").is_none());

    let event = &item.subcomponents[0];
    assert_eq!(event.name, s!("VEVENT"));
    assert!(event.single_prop("ORGANIZER").is_some());
    assert_eq!(event.single_prop("LOCATION").unwrap().raw_value, s!("Somewhere"));
}

#[test]
fn test_escaping() {
    let item = parse_component(
            "BEGIN:VCALENDAR\n\
            ORGANIZER;CN=\"Cott:n Eye Joe\":mailto:joe@joe.com\n\
            END:VCALENDAR\n").unwrap();
    assert_eq!(item.name, s!("VCALENDAR"));
    assert_eq!(item.single_prop("ORGANIZER").unwrap().raw_value, s!("mailto:joe@joe.com"));
}

#[test]
fn test_property_groups() {
    let item = parse_component(
            "BEGIN:VCARD\n\
            foo.EMAIL;TYPE=INTERNET:foo@example.com\n\
            foo.X-ACTUAL-TYPE:CUSTOM\n\
            END:VCARD\n").unwrap();
    assert_eq!(item.single_prop("EMAIL").unwrap().prop_group, Some("foo".to_owned()));

}
