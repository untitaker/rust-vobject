#![feature(globs,macro_rules)]
extern crate vobject;
use vobject::parse_item;

macro_rules! s(
    ($i:expr) => (&$i.into_string());
)


#[test]
fn test_wikipedia_1() {
    let item = parse_item(s!(
        "BEGIN:VCARD\n\
        VERSION:2.1\n\
        N:Mustermann;Erika\n\
        FN:Erika Mustermann\n\
        ORG:Wikipedia\n\
        TITLE:Oberleutnant\n\
        PHOTO;JPEG:http://commons.wikimedia.org/wiki/File:Erika_Mustermann_2010.jpg\n\
        TEL;WORK;VOICE:(0221) 9999123\n\
        TEL;HOME;VOICE:(0221) 1234567\n\
        ADR;HOME:;;Heidestrasse 17;Koeln;;51147;Deutschland\n\
        EMAIL;PREF;INTERNET:erika@mustermann.de\n\
        REV:20140301T221110Z\n\
        END:VCARD")).unwrap();

    assert_eq!(item.single_value(s!("FN")), Some(s!("Erika Mustermann")));
    assert_eq!(item.single_value(s!("N")),  Some(s!("Mustermann;Erika")));

    let mut tel_values = item.all_props(s!("TEL")).iter().map(|x| x.get_raw_value());
    assert_eq!(tel_values.next().unwrap(), s!("(0221) 9999123"));
    assert_eq!(tel_values.next().unwrap(), s!("(0221) 1234567"));
    assert!(tel_values.next().is_none());
}

#[test]
fn test_line_cont() {
    let item = parse_item(s!(
        "BEGIN:VCARD\n\
        VERSION:2.1\n\
        N;ENCODING=QUOTED-PRINTABLE:Nikdo;Nikdo=\n\t\
        vic\n\
        NOTE:This ends with equal sign=\n\
        TEL;WORK:5555\n \
        4444\n\
        END:VCARD")).unwrap();

    assert_eq!(item.single_value(s!("TEL")), Some(s!("55554444")));
    assert_eq!(item.single_value(s!("N")), Some(s!("Nikdo;Nikdo=vic")));
}
