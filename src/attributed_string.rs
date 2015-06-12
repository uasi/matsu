use std::ops::Range;

use attribute::Attribute;

/// Represents attributed string.
#[derive(Clone)]
pub struct AttributedString {
    attr_ranges: Vec<AttributeRange>,
    string: String,
}

impl AttributedString {
    /// Creates a new attributed string with the given string.
    pub fn new<T: Into<String>>(string: T) -> Self {
        AttributedString {
            attr_ranges: Vec::new(),
            string: string.into(),
        }
    }

    /// Adds the given attribute to the substring of the specified range in bytes.
    ///
    /// # Panics
    ///
    /// - Panics if `range.end` is greater than `range.start`.
    /// - Panics if `range` does not lie on character boundary of the underlying string.
    pub fn add_attr(&mut self, attr: Attribute, range: Range<usize>) {
        if range.start > range.end {
            panic!("reverse range not allowed");
        }
        if !byte_range_lies_on_char_boundary(&range, &self.string) {
            panic!("range {:?} in `{:?}` does not lie on character boundary", range, &self.string);
        }
        if range.start == range.end {
            return;
        }
        for ar in self.attr_ranges.iter_mut() {
            if ar.attr == attr && is_overlapping_or_adjacent(&ar.range, &range) {
                extend_to_cover(&mut ar.range, &range);
                return;
            }
        }
        self.attr_ranges.push(AttributeRange { attr: attr, range: range });
    }

    /// Returns the length of the underlying string in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use matsu::AttributedString;
    /// #
    /// let s = AttributedString::new("端末");
    /// assert_eq!(s.len(), 6);
    /// ```
    pub fn len(&self) -> usize {
        self.string.len()
    }

    /// Returns a string slice containing the entire underlying string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use matsu::Attribute;
    /// # use matsu::AttributedString;
    /// #
    /// let mut s = AttributedString::new("abc");
    /// s.add_attr(Attribute::Bold, 0..2);
    /// assert_eq!(s.as_plain_str(), "abc");
    /// ```
    pub fn as_plain_str(&self) -> &str {
        self.string.as_ref()
    }

    /// Returns a string formatted by inserting ANSI escape codes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use matsu::Attribute;
    /// # use matsu::AttributedString;
    /// #
    /// let mut s = AttributedString::new("bold and underlined");
    /// s.add_attr(Attribute::Bold, 0..4);
    /// s.add_attr(Attribute::Underline, 9..19);
    /// assert_eq!(s.to_string(), "\x1B[1mbold\x1B[22m and \x1B[4munderlined\x1B[24m");
    /// ```
    ///
    /// In a terminal, the string will be printed as: **bold** and <u>underlined</u>
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut src_idx = 0usize;
        for (idx, attr, enable) in self.attr_indices().into_iter() {
            if idx > src_idx {
                let substr = &self.string[src_idx..idx];
                buf.push_str(substr);
                src_idx = idx;
            }
            let code = if enable { attr.initiator() } else { attr.terminator() };
            buf.push_str(&code);
        }
        if src_idx < self.string.len() {
            let substr = &self.string[src_idx..];
            buf.push_str(substr);
        }
        buf
    }

    // Returns a vector of `(index, attr, enable)`.
    fn attr_indices(&self) -> Vec<(usize, Attribute, bool)> {
        let mut codes = Vec::new();
        for ar in self.attr_ranges.iter() {
            codes.push((ar.range.start, ar.attr, true));
            codes.push((ar.range.end, ar.attr, false));
        }
        codes.sort_by(|a, b| {
            if a.0 != b.0 { return a.0.cmp(&b.0); }
            a.2.cmp(&b.2) // false < true, i.e., disable < enable
        });
        codes
    }
}

#[derive(Clone)]
struct AttributeRange {
    attr: Attribute,
    range: Range<usize>,
}

fn extend_to_cover(a: &mut Range<usize>, b: &Range<usize>) {
    if a.start > b.start {
        a.start = b.start;
    }
    if a.end < b.end {
        a.end = b.end;
    }
}

fn is_overlapping_or_adjacent(a: &Range<usize>, b: &Range<usize>) -> bool {
    (a.start <= b.end && a.end >= b.start) ||
    (b.start <= a.end && b.end >= a.start)
}

fn byte_range_lies_on_char_boundary(range: &Range<usize>, s: &str) -> bool {
    if range.end > s.len() {
        return false;
    }
    let bytes = unsafe { s.slice_unchecked(range.start, range.end) };
    String::from_utf8(Vec::from(bytes)).is_ok()
}
