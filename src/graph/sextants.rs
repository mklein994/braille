mod columns;
mod lines;

pub use columns::Columns;
pub use lines::Lines;
use std::fmt;

struct Char {
    inner: &'static str,
}

impl Char {
    pub fn new(dots: [[bool; 2]; 3]) -> Self {
        Self {
            inner: match dots {
                [[false, false], [false, false], [false, false]] => " ",
                [[true, false], [false, false], [false, false]] => "🬀",
                [[false, true], [false, false], [false, false]] => "🬁",
                [[true, true], [false, false], [false, false]] => "🬂",
                [[false, false], [true, false], [false, false]] => "🬃",
                [[true, false], [true, false], [false, false]] => "🬄",
                [[false, true], [true, false], [false, false]] => "🬅",
                [[true, true], [true, false], [false, false]] => "🬆",
                [[false, false], [false, true], [false, false]] => "🬇",
                [[true, false], [false, true], [false, false]] => "🬈",
                [[false, true], [false, true], [false, false]] => "🬉",
                [[true, true], [false, true], [false, false]] => "🬊",
                [[false, false], [true, true], [false, false]] => "🬋",
                [[true, false], [true, true], [false, false]] => "🬌",
                [[false, true], [true, true], [false, false]] => "🬍",
                [[true, true], [true, true], [false, false]] => "🬎",
                [[false, false], [false, false], [true, false]] => "🬏",
                [[true, false], [false, false], [true, false]] => "🬐",
                [[false, true], [false, false], [true, false]] => "🬑",
                [[true, true], [false, false], [true, false]] => "🬒",
                [[false, false], [true, false], [true, false]] => "🬓",
                [[true, false], [true, false], [true, false]] => "▌",
                [[false, true], [true, false], [true, false]] => "🬔",
                [[true, true], [true, false], [true, false]] => "🬕",
                [[false, false], [false, true], [true, false]] => "🬖",
                [[true, false], [false, true], [true, false]] => "🬗",
                [[false, true], [false, true], [true, false]] => "🬘",
                [[true, true], [false, true], [true, false]] => "🬙",
                [[false, false], [true, true], [true, false]] => "🬚",
                [[true, false], [true, true], [true, false]] => "🬛",
                [[false, true], [true, true], [true, false]] => "🬜",
                [[true, true], [true, true], [true, false]] => "🬝",
                [[false, false], [false, false], [false, true]] => "🬞",
                [[true, false], [false, false], [false, true]] => "🬟",
                [[false, true], [false, false], [false, true]] => "🬠",
                [[true, true], [false, false], [false, true]] => "🬡",
                [[false, false], [true, false], [false, true]] => "🬢",
                [[true, false], [true, false], [false, true]] => "🬣",
                [[false, true], [true, false], [false, true]] => "🬤",
                [[true, true], [true, false], [false, true]] => "🬥",
                [[false, false], [false, true], [false, true]] => "🬦",
                [[true, false], [false, true], [false, true]] => "🬧",
                [[false, true], [false, true], [false, true]] => "▐",
                [[true, true], [false, true], [false, true]] => "🬨",
                [[false, false], [true, true], [false, true]] => "🬩",
                [[true, false], [true, true], [false, true]] => "🬪",
                [[false, true], [true, true], [false, true]] => "🬫",
                [[true, true], [true, true], [false, true]] => "🬬",
                [[false, false], [false, false], [true, true]] => "🬭",
                [[true, false], [false, false], [true, true]] => "🬮",
                [[false, true], [false, false], [true, true]] => "🬯",
                [[true, true], [false, false], [true, true]] => "🬰",
                [[false, false], [true, false], [true, true]] => "🬱",
                [[true, false], [true, false], [true, true]] => "🬲",
                [[false, true], [true, false], [true, true]] => "🬳",
                [[true, true], [true, false], [true, true]] => "🬴",
                [[false, false], [false, true], [true, true]] => "🬵",
                [[true, false], [false, true], [true, true]] => "🬶",
                [[false, true], [false, true], [true, true]] => "🬷",
                [[true, true], [false, true], [true, true]] => "🬸",
                [[false, false], [true, true], [true, true]] => "🬹",
                [[true, false], [true, true], [true, true]] => "🬺",
                [[false, true], [true, true], [true, true]] => "🬻",
                [[true, true], [true, true], [true, true]] => "█",
            },
        }
    }

    pub fn as_str(&self) -> &'static str {
        self.inner
    }
}

impl fmt::Display for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
