
error_chain! {

    types {
        VObjectError, VObjectErrorKind, ResultExt, Result;
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

    }


}
