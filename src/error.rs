
error_chain! {

    types {
        VObjectError, VObjectErrorKind, ResultExt, Result;
    }

    foreign_links {
        ChronoParseError(::chrono::format::ParseError) #[cfg(feature = "timeconversions")];
    }

    errors {
        ParserError(desc: String) {
            description("Parser error")
            display("{}", desc)
        }

        NotAVCard {
            description("Input is not a valid VCard")
            display("Passed content string is not a VCard")
        }

        NotAnICalendar(content: String) {
            description("Input is not a valid ICalendar")
            display("Not an ICalendar: '{}'", content)
        }
    }


}
