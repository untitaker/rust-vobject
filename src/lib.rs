// DOCS

#[macro_use]
extern crate error_chain;

pub mod component;
pub mod error;
mod parser;
pub mod property;

pub use component::Component;
pub use component::parse_component;
pub use component::write_component;
pub use property::Property;
pub use property::escape_chars;
pub use property::unescape_chars;

#[cfg(test)]
mod tests {
    use error::*;
    use component::fold_line;
    use parser::Parser;

    #[test]
    fn test_unfold1() {
        let mut p = Parser{input: "ab\r\n c", pos: 2};
        assert_eq!(p.consume_char(), Some('c'));
        assert_eq!(p.pos, 6);
    }

    #[test]
    fn test_unfold2() {
        let mut p = Parser{input: "ab\n\tc\nx", pos: 2};
        assert_eq!(p.consume_char(), Some('c'));
        assert_eq!(p.consume_char(), Some('\n'));
        assert_eq!(p.consume_char(), Some('x'));
    }

    #[test]
    fn test_fold() {
        let line = "This should be multiple lines and fold on char boundaries. 毎害止\
                   加食下組多地将写館来局必第。東証細再記得玲祉込吉宣会法授";
        let expected = "This should be multiple lines and fold on char boundaries. 毎害止\
                       加食\r\n 下組多地将写館来局必第。東証細再記得玲祉込吉宣会法\r\n 授";
        assert_eq!(expected, fold_line(line));
        assert_eq!("ab", fold_line("ab"));
    }

    #[test]
    fn test_consume_while() {
        let mut p = Parser{input:"af\n oo:bar", pos: 1};
        assert_eq!(p.consume_while(|x| x != ':'), "foo");
        assert_eq!(p.consume_char(), Some(':'));
        assert_eq!(p.consume_while(|x| x != '\n'), "bar");
    }

    #[test]
    fn test_consume_while2() {
        let mut p = Parser{input:"af\n oo\n\t:bar", pos: 1};
        assert_eq!(p.consume_while(|x| x != ':'), "foo");
        assert_eq!(p.consume_char(), Some(':'));
        assert_eq!(p.consume_while(|x| x != '\n'), "bar");
    }

    #[test]
    fn test_consume_while3() {
        let mut p = Parser{input:"af\n oo:\n bar", pos: 1};
        assert_eq!(p.consume_while(|x| x != ':'), "foo");
        assert_eq!(p.consume_char(), Some(':'));
        assert_eq!(p.consume_while(|x| x != '\n'), "bar");
    }

    #[test]
    fn test_consume_only_char() {
        let mut p = Parser{input:"\n \"bar", pos: 0};
        assert!(p.consume_only_char('"'));
        assert_eq!(p.pos, 3);
        assert!(!p.consume_only_char('"'));
        assert_eq!(p.pos, 3);
        assert!(p.consume_only_char('b'));
        assert_eq!(p.pos, 4);
    }

    #[test]
    fn mismatched_begin_end_tags_returns_error() {
        // Test for infinite loops as well
        use std::sync::mpsc::{channel, RecvTimeoutError};
        use std::time::Duration;
        let mut p = Parser {input: "BEGIN:a\nBEGIN:b\nEND:a", pos: 0};

        let (tx, rx) = channel();
        ::std::thread::spawn(move|| { tx.send(p.consume_component()) });

        match rx.recv_timeout(Duration::from_millis(50)) {
            Err(RecvTimeoutError::Timeout) => assert!(false),
            Ok(Err(VObjectError(VObjectErrorKind::ParserError{..}, _ ))) => assert!(true),
            _ => assert!(false),
        }
    }
}

