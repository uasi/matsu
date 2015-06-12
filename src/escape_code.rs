macro_rules! code {
    ( $( $e:expr ),* ) => {
        concat!( "\x1B[", $( $e, )* "m" ).to_string()
    }
}

#[derive(Clone, Copy)]
pub enum EscapeCode {
    Reset,       // 0
    Bold,        // 1
    Underline,   // 4
    Blink,       // 5
    Negative,    // 7
    NoBold,      // 22
    NoUnderline, // 24
    NoBlink,     // 25
    NoNegative,  // 27

    // FgColor0 to FgColor7,     // 30-37
    // FgColorRgb((u8, u8, u8)), // 38;2;<r>;<g>;<b>, not widely supported
    FgColor256(u8),              // 38;5;<u8>
    FgColorDefault,              // 39

    // BgColor0 to BgColor7,     // 40-47
    // BgColorRgb((u8, u8, u8)), // 48;2;<r>;<g>;<b>, not widely supported
    BgColor256(u8),              // 48;5;<u8>
    BgColorDefault,              // 49
}

impl EscapeCode {
    pub fn to_string(&self) -> String {
        use self::EscapeCode::*;
        match *self {
            Reset             => code!(0),
            Bold              => code!(1),
            Underline         => code!(4),
            Blink             => code!(5),
            Negative          => code!(7),
            NoBold            => code!(22),
            NoUnderline       => code!(24),
            NoBlink           => code!(25),
            NoNegative        => code!(27),
            FgColor256(color) => format!("\x1B[38;5;{}m", color),
            FgColorDefault    => code!(39),
            BgColor256(color) => format!("\x1B[48;5;{}m", color),
            BgColorDefault    => code!(49),
        }
    }
}
