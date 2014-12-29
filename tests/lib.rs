#![feature(globs,macro_rules)]
extern crate vobject;
use vobject::parse_component;

macro_rules! s(
    ($i:expr) => (&$i.into_string());
);


#[test]
fn test_vcard_basic() {
    let item = parse_component(s!(
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
        END:VCARD\n\r\n\n")).unwrap();

    assert_eq!(item.single_prop(s!("FN")).unwrap().get_raw_value(), s!("Erika Mustermann"));
    assert_eq!(item.single_prop(s!("N")).unwrap().get_raw_value(),  s!("Mustermann;Erika"));

    let mut tel_values = item.all_props(s!("TEL")).iter().map(|x| x.get_raw_value());
    assert_eq!(tel_values.next().unwrap(), s!("(0221) 9999123"));
    assert_eq!(tel_values.next().unwrap(), s!("(0221) 1234567"));
    assert!(tel_values.next().is_none());
}

#[test]
fn test_line_cont() {
    let item = parse_component(s!(
        "BEGIN:VCARD\n\
        VERSION:2.1\n\
        N;ENCODING=QUOTED-PRINTABLE:Nikdo;Nikdo=\n\t\
        vic\n\
        NOTE:This ends with equal sign=\n\
        TEL;WORK:5555\n \
        4444\n\
        END:VCARD")).unwrap();

    assert_eq!(&item.name, s!("VCARD"));
    assert_eq!(item.single_prop(s!("TEL")).unwrap().get_raw_value(), s!("55554444"));
    assert_eq!(item.single_prop(s!("N")).unwrap().get_raw_value(), s!("Nikdo;Nikdo=vic"));
}

#[test]
fn test_icalendar_basic() {
    let item = parse_component(s!(
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
            END:VCALENDAR\n")).unwrap();

    assert_eq!(&item.name, s!("VCALENDAR"));
    assert!(item.single_prop(s!("LOCATION")).is_none());
    assert!(item.single_prop(s!("ORGANIZER")).is_none());

    let event = &item.subcomponents[0];
    assert_eq!(&event.name, s!("VEVENT"));
    assert!(event.single_prop(s!("ORGANIZER")).is_some());
    assert_eq!(event.single_prop(s!("LOCATION")).unwrap().get_raw_value(), s!("Somewhere"));
}

#[test]
fn test_escaping() {
    let item = parse_component(s!(
            "BEGIN:VCALENDAR\n\
            ORGANIZER;CN=\"Cott:n Eye Joe\":mailto:joe@joe.com\n\
            END:VCALENDAR\n")).unwrap();
    assert_eq!(&item.name, s!("VCALENDAR"));
    assert_eq!(item.single_prop(s!("ORGANIZER")).unwrap().get_raw_value(), s!("mailto:joe@joe.com"));
}
