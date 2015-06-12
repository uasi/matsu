use escape_code::EscapeCode as C;

/// Attributes for `AttributedString`.
#[derive(Clone, Copy, PartialEq)]
pub enum Attribute {
    Blink,
    Bold,
    Inverse,
    Underline,
    FgColor256(u8),
    FgColorDefault,
    BgColor256(u8),
    BgColorDefault,
}

impl Attribute {
    /// Returns an ANSI escape code which makes a terminal enables the character attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// # use matsu::Attribute;
    /// assert_eq!(Attribute::Bold.initiator(), "\x1B[1m".to_string());
    /// ```
    pub fn initiator(&self) -> String {
        use self::Attribute::*;
        match *self {
            Blink          => C::Blink,
            Bold           => C::Bold,
            Inverse        => C::Negative,
            Underline      => C::Underline,
            FgColor256(c)  => C::FgColor256(c),
            FgColorDefault => C::FgColorDefault,
            BgColor256(c)  => C::BgColor256(c),
            BgColorDefault => C::BgColorDefault,
        }.to_string()
    }

    /// Returns an ANSI escape code which makes a terminal disables the character attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// # use matsu::Attribute;
    /// assert_eq!(Attribute::Bold.terminator(), "\x1B[22m".to_string());
    /// ```
    pub fn terminator(&self) -> String {
        use self::Attribute::*;
        match *self {
            Blink          => C::NoBlink,
            Bold           => C::NoBold,
            Inverse        => C::NoNegative,
            Underline      => C::NoUnderline,
            FgColor256(_)  => C::FgColorDefault,
            FgColorDefault => C::FgColorDefault,
            BgColor256(_)  => C::BgColorDefault,
            BgColorDefault => C::BgColorDefault,
        }.to_string()
    }
}
