use std::ops::Deref;
use std::collections::BTreeMap;

use component::Component;
use component::parse_component;
use property::Property;

use std::result::Result as RResult;
use error::*;

#[derive(Debug)]
pub struct Vcard(Component);

/// The Vcard object.
///
/// This type simply holds data and offers functions to access this data. It does not compute
/// anything.
impl Vcard {

    /// Parse a string to a Vcard object
    ///
    /// Returns an error if the parsed text is not a Vcard (that means that an error is returned
    /// also if this is a valid icalendar!)
    ///
    pub fn build(s: &str) -> Result<Vcard> {
        parse_component(s)
            .and_then(|c| {
                Self::from_component(c)
                    .map_err(|_| {
                        let kind = VObjectErrorKind::NotAVCard;
                        VObjectError::from_kind(kind)
                    })
            })
    }

    /// Wrap a Component into a Vcard object, or don't do it if the Component is not a Vcard.
    pub fn from_component(c: Component)-> RResult<Vcard, Component> {
        if c.name == "VCARD" {
            Ok(Vcard(c))
        } else {
            Err(c)
        }
    }

    make_getter_function_for_values!(adr            , "ADR"          , Adr);
    make_getter_function_for_optional!(anniversary  , "ANNIVERSARY"  , Anniversary);
    make_getter_function_for_optional!(bday         , "BDAY"         , BDay);
    make_getter_function_for_values!(categories     , "CATEGORIES"   , Category);
    make_getter_function_for_optional!(clientpidmap , "CLIENTPIDMAP" , ClientPidMap);
    make_getter_function_for_values!(email          , "EMAIL"        , Email);
    make_getter_function_for_values!(fullname       , "FN"           , FullName);
    make_getter_function_for_optional!(gender       , "GENDER"       , Gender);
    make_getter_function_for_values!(geo            , "GEO"          , Geo);
    make_getter_function_for_values!(impp           , "IMPP"         , IMPP);
    make_getter_function_for_values!(key            , "KEY"          , Key);
    make_getter_function_for_values!(lang           , "LANG"         , Lang);
    make_getter_function_for_values!(logo           , "LOGO"         , Logo);
    make_getter_function_for_values!(member         , "MEMBER"       , Member);
    make_getter_function_for_optional!(name         , "N"            , Name);
    make_getter_function_for_values!(nickname       , "NICKNAME"     , NickName);
    make_getter_function_for_values!(note           , "NOTE"         , Note);
    make_getter_function_for_values!(org            , "ORG"          , Organization);
    make_getter_function_for_values!(photo          , "PHOTO"        , Photo);
    make_getter_function_for_optional!(proid        , "PRIOD"        , Proid);
    make_getter_function_for_values!(related        , "RELATED"      , Related);
    make_getter_function_for_optional!(rev          , "REV"          , Rev);
    make_getter_function_for_values!(role           , "ROLE"         , Title);
    make_getter_function_for_values!(sound          , "SOUND"        , Sound);
    make_getter_function_for_values!(tel            , "TEL"          , Tel);
    make_getter_function_for_values!(title          , "TITLE"        , Title);
    make_getter_function_for_values!(tz             , "TZ"           , Tz);
    make_getter_function_for_optional!(uid          , "UID"          , Uid);
    make_getter_function_for_values!(url            , "URL"          , Url);
    make_getter_function_for_optional!(version      , "VERSION"      , Version);

    make_builder_fn!(fn with_adr building "ADR" with_params,
                     |o| o.unwrap_or(String::from("")) =>
                     pobox    : Option<String>,
                     ext      : Option<String>,
                     street   : Option<String>,
                     locality : Option<String>,
                     region   : Option<String>,
                     code     : Option<String>,
                     country  : Option<String>);

    make_builder_fn!(fn with_anniversary  building "ANNIVERSARY"        , |o| o => value: String);
    make_builder_fn!(fn with_bday         building "BDAY" with_params   , |o| o => value: String);
    make_builder_fn!(fn with_categories   building "CATEGORIES"         , |o| o.join(";") => org: Vec<String>);
    make_builder_fn!(fn with_clientpidmap building "CLIENTPIDMAP"       , |o| o => raw: String);
    make_builder_fn!(fn with_email        building "EMAIL"              , |o| o => email: String);
    make_builder_fn!(fn with_fullname     building "FN"                 , |o| o => fullname: String);
    make_builder_fn!(fn with_gender       building "GENDER" with_params , |o| o => value: String);
    make_builder_fn!(fn with_geo          building "GEO"                , |o| o => uri: String);
    make_builder_fn!(fn with_impp         building "IMPP"               , |o| o => uri: String);
    make_builder_fn!(fn with_key          building "KEY"                , |o| o => uri: String);
    make_builder_fn!(fn with_lang         building "LANG"               , |o| o => lang: String);
    make_builder_fn!(fn with_logo         building "LOGO"               , |o| o => uri: String);
    make_builder_fn!(fn with_member       building "MEMBER"             , |o| o => uri: String);

    make_builder_fn!(fn with_name building "N" with_params,
                     |o| o.unwrap_or(String::from("")) =>
                     surname            : Option<String>,
                     given_name         : Option<String>,
                     additional_name    : Option<String>,
                     honorific_prefixes : Option<String>,
                     honorific_suffixes : Option<String>);

    make_builder_fn!(fn with_nickname building "NICKNAME" with_params , |o| o => name: String);
    make_builder_fn!(fn with_note     building "NOTE"                 , |o| o => text: String);
    make_builder_fn!(fn with_org      building "ORG"                  , |o| o.join(";") => org: Vec<String>);
    make_builder_fn!(fn with_photo    building "PHOTO" with_params    , |o| o => param: String);
    make_builder_fn!(fn with_proid    building "PRODID"               , |o| o => param: String);
    make_builder_fn!(fn with_related  building "RELATED"              , |o| o => uri: String);
    make_builder_fn!(fn with_rev      building "REV"                  , |o| o => timestamp: String);
    make_builder_fn!(fn with_role     building "ROLE"                 , |o| o => role: String);
    make_builder_fn!(fn with_sound    building "SOUND"                , |o| o => uri: String);
    make_builder_fn!(fn with_tel      building "TEL" with_params      , |o| o => value: String);
    make_builder_fn!(fn with_title    building "TITLE"                , |o| o => title: String);
    make_builder_fn!(fn with_tz       building "TZ"                   , |o| o => tz: String);
    make_builder_fn!(fn with_uid      building "UID"                  , |o| o => uri: String);
    make_builder_fn!(fn with_url      building "URL"                  , |o| o => uri: String);
    make_builder_fn!(fn with_version  building "VERSION"              , |o| o => version: String);

}

impl Default for Vcard {
    fn default() -> Self {
        Vcard(Component::new(String::from("VCARD")))
    }
}

impl Deref for Vcard {
    type Target = Component;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

create_data_type!(Adr);
create_data_type!(Anniversary);
create_data_type!(BDay);
create_data_type!(Category);
create_data_type!(ClientPidMap);
create_data_type!(Email);
create_data_type!(FullName);
create_data_type!(Gender);
create_data_type!(Geo);
create_data_type!(IMPP);
create_data_type!(Key);
create_data_type!(Lang);
create_data_type!(Logo);
create_data_type!(Member);
create_data_type!(Name);
create_data_type!(NickName);
create_data_type!(Note);
create_data_type!(Organization);
create_data_type!(PhoneNumber);
create_data_type!(Photo);
create_data_type!(Proid);
create_data_type!(Related);
create_data_type!(Rev);
create_data_type!(Sound);
create_data_type!(Tel);
create_data_type!(Title);
create_data_type!(Tz);
create_data_type!(Uid);
create_data_type!(Url);
create_data_type!(Version);

/// A Name type
///
/// offers functionality to get firstname, middlenames and lastname.
///
/// The parsing behaviour is implemented in a way that splits at whitespace, following these rules:
///
/// * If there is only one element after splitting, this is considered the lastname
/// * If there are two elements, this is firstname and lastname
/// * If there are more than two elements, firstname and lastname are the first and last elements
/// respectively, all others are middlenames.
///
impl Name {

    pub fn plain(&self) -> String {
        self.0.clone()
    }

    pub fn surname(&self) -> Option<String> {
        self.0.split(";").nth(0).map(String::from)
    }

    pub fn given_name(&self) -> Option<String> {
        self.0.split(";").nth(1).map(String::from)
    }

    pub fn additional_names(&self) -> Option<String> {
        self.0.split(";").nth(2).map(String::from)
    }

    pub fn honorific_prefixes(&self) -> Option<String> {
        self.0.split(";").nth(3).map(String::from)
    }

    pub fn honorific_suffixes(&self) -> Option<String> {
        self.0.split(";").nth(4).map(String::from)
    }

    /// Alias for Name::surname()
    #[inline]
    pub fn family_name(&self) -> Option<String> {
        self.surname()
    }

}

pub struct VcardBuilder(Component);

#[cfg(test)]
mod test {
    use super::Vcard;

    #[test]
    fn test_vcard_basic() {
        let item = Vcard::build(
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

        assert_eq!(item.adr()[0].raw(), ";;Heidestrasse 17;Koeln;;51147;Deutschland");
        assert_eq!(item.fullname()[0].raw(), "Erika Mustermann");
        assert_eq!(item.name().unwrap().plain(), "Mustermann;Erika");
        assert_eq!(item.name().unwrap().surname().unwrap()    , "Mustermann");
        assert_eq!(item.name().unwrap().given_name().unwrap() , "Erika");
        assert_eq!(item.org()[0].raw() , "Wikipedia");
        assert_eq!(item.title()[0].raw() , "Oberleutnant");
    }

    #[test]
    fn test_vcard_builder() {
        use component::write_component;

        let build = Vcard::default()
            .with_name(parameters!(),
                       None,
                       Some("Mustermann".into()),
                       None,
                       Some("Erika".into()),
                       None)
            .with_fullname("Erika Mustermann".into())
            .with_org(vec!["Wikipedia".into()])
            .with_title("Oberleutnant".into())
            .with_tel(parameters!("TYPE" => "WORK"), "(0221) 9999123".into())
            .with_tel(parameters!("TYPE" => "HOME"), "(0221) 1234567".into())
            .with_adr(parameters!("TYPE" => "HOME"),
                      None,
                      None,
                      Some("Heidestrasse 17".into()),
                      Some("Koeln".into()),
                      None,
                      Some("51147".into()),
                      Some("Deutschland".into()))
            .with_email("erika@mustermann.de".into())
            .with_rev("20140301T221110Z".into());

        let build_string = write_component(&build);

        let expected =
            "BEGIN:VCARD\r\n\
            ADR;TYPE=HOME:;;Heidestrasse 17;Koeln;;51147;Deutschland\r\n\
            EMAIL:erika@mustermann.de\r\n\
            FN:Erika Mustermann\r\n\
            N:;Mustermann;;Erika;\r\n\
            ORG:Wikipedia\r\n\
            REV:20140301T221110Z\r\n\
            TEL;TYPE=WORK:(0221) 9999123\r\n\
            TEL;TYPE=HOME:(0221) 1234567\r\n\
            TITLE:Oberleutnant\r\n\
            END:VCARD\r\n";


        assert_eq!(expected, build_string);
    }

}

