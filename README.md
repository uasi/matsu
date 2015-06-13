# Matsu [![Build Status](https://travis-ci.org/uasi/matsu.svg?branch=master)](https://travis-ci.org/uasi/matsu)

This module provides an attributed string suitable for printing formatted
characters on a text terminal.

An attributed string manages a string and associated set of attributes that
apply to range of characters in the string. It can turn into a sequence of
characters interleaved with [ANSI escape codes], which encodes formatting
information in the manner that a terminal can interpret.

[ANSI escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code

Matsu stands for Manipulate Attributed, Terminal-capable Strings in UTF-8.

## License

Matsu is released under the MIT License.

## Contributing

1. Fork it
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create new Pull Request
