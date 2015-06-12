# matsu

This module provides an attributed string suitable for printing formatted
characters on a text terminal.

An attributed string manages a string and associated set of attributes that
apply to range of characters in the string. It can turn into a sequence of
characters interleaved with [ANSI escape codes], which encodes formatting
information in the manner that a terminal can interpret.

[ANSI escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code

Matsu stands for Manipulate Attributed, Terminal-capable Strings in UTF-8.
