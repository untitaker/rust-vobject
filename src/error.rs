
error_chain! {

    types {
        VObjectError, VObjectErrorKind, ResultExt, Result;
    }

    errors {
        ParserError(desc: String) {
            description("Parser error")
            display("{}", desc)
        }
    }


}
