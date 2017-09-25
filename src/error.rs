
error_chain! {

    types {
        VObjectError, VObjectErrorKind, ResultExt, Result;
    }

    errors {
        ParserError(desc: String) {
            description("Parser error")
            display("{}", desc)
        }

        NotAVCard(content: String) {
            description("Input is not a valid VCard")
            display("Not a VCard: '{}'", content)
        }

    }


}
