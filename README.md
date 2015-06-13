# Matsu [![Build Status](https://travis-ci.org/uasi/matsu.svg?branch=master)](https://travis-ci.org/uasi/matsu)

This module provides an attributed string suitable for printing formatted
characters on a text terminal.

An attributed string manages a string and associated set of attributes that
apply to range of characters in the string. It can turn into a sequence of
characters interleaved with [ANSI escape codes], which encodes formatting
information in the manner that a terminal can interpret.

[ANSI escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code

Matsu stands for Manipulate Attributed, Terminal-capable Strings in UTF-8.

## Examples

```rust
use matsu::{Attribute, AttributedString};

let mut s = AttributedString::new("bold and underlined");
s.add_attr(Attribute::Bold, 0..4);
s.add_attr(Attribute::Underline, 9..19);

assert_eq!(s.to_string(), "\x1B[1mbold\x1B[22m and \x1B[4munderlined\x1B[24m");
```

In a terminal, the string will be printed as: **bold** and <u>underlined</u>

## Author

Tomoki Aonuma ([@uasi](https://twitter.com/uasi))

## License

Matsu is released under the MIT License.

## Contributing

1. Fork it
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create new Pull Request
