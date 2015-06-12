//! This module provides an attributed string suitable for printing formatted characters on a text terminal.
//!
//! An attributed string manages a string and associated set of attributes that apply to range of characters
//! in the string. It can turn into a sequence of characters interleaved with [ANSI escape codes], which
//! encodes formatting information in the manner that a terminal can interpret.
//!
//! [ANSI escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code
//!
//! Matsu stands for Manipulate Attributed, Terminal-capable Strings in UTF-8.

mod attribute;
mod attributed_string;
mod escape_code;

pub use attribute::Attribute;
pub use attributed_string::AttributedString;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = AttributedString::new("it works!!!");
        s.add_attr(Attribute::Bold, 3..5);
        s.add_attr(Attribute::Bold, 5..8);
        s.add_attr(Attribute::Underline, 0..2);
        s.add_attr(Attribute::Underline, 3..11);
        assert_eq!(s.to_string(), "\x1B[4mit\x1B[24m \x1B[1m\x1B[4mworks\x1B[22m!!!\x1B[24m".to_string());
        //                                           ^^^^^^^^^^^^^^
        // Note that the order of escape codes is not guaranteed when two or more attributes starts or ends at
        // the same index. Currently they are ordered on a first-come-first-served basis.
    }

    #[test]
    #[should_panic]
    fn range_must_lie_on_char_boundary_1() {
        let mut s = AttributedString::new("abc");
        s.add_attr(Attribute::Bold, 0..99);
    }

    #[test]
    #[should_panic]
    fn range_must_lie_on_char_boundary_2() {
        let mut s = AttributedString::new("😄");
        s.add_attr(Attribute::Bold, 0..1);
    }
}
