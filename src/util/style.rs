use std::fmt::{Display, Formatter, Result, Write as _};

pub struct StyleTracker {
    enabled: bool,
    style: Style,
}

impl Default for StyleTracker {
    fn default() -> Self {
        Self::new(true)
    }
}

impl StyleTracker {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            style: Style::default(),
        }
    }

    pub fn style(&mut self, style: Style) -> StyleDiff {
        if !self.enabled {
            return StyleDiff::None;
        }

        let diff = style.diff(&self.style);
        self.style = style;
        diff
    }

    pub fn clear(&mut self) -> StyleDiff {
        if !self.enabled || self.style == Style::CLEAR {
            StyleDiff::None
        } else {
            self.style = Style::CLEAR;
            StyleDiff::Clear
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
}

impl Style {
    const CLEAR: Self = Self {
        fg: Color::Default,
        bg: Color::Default,
    };

    pub const fn new(fg: Color, bg: Color) -> Self {
        Self { fg, bg }
    }

    pub const fn clear() -> Self {
        Self::CLEAR
    }

    pub const fn fg(color: Color) -> Self {
        Self::new(color, Color::Default)
    }

    pub const fn bg(color: Color) -> Self {
        Self::new(Color::Default, color)
    }

    pub const fn with_fg(self, color: Color) -> Self {
        Self { fg: color, ..self }
    }

    pub const fn with_bg(self, color: Color) -> Self {
        Self { bg: color, ..self }
    }

    pub fn diff(&self, prev: &Self) -> StyleDiff {
        if *self == *prev {
            StyleDiff::None
        } else if *self == Self::CLEAR {
            StyleDiff::Clear
        } else {
            let fg = if self.fg != prev.fg {
                Some(self.fg)
            } else {
                None
            };

            let bg = if self.bg != prev.bg {
                Some(self.bg)
            } else {
                None
            };

            StyleDiff::new(fg, bg)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleDiff {
    None,
    Clear,
    Foreground(Color),
    Background(Color),
    Both { fg: Color, bg: Color },
}

impl StyleDiff {
    const fn new(fg: Option<Color>, bg: Option<Color>) -> Self {
        match (fg, bg) {
            (Some(fg), Some(bg)) => Self::Both { fg, bg },
            (Some(fg), None) => Self::Foreground(fg),
            (None, Some(bg)) => Self::Background(bg),
            (None, None) => Self::None,
        }
    }

    fn write(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::None => Ok(()),
            Self::Clear => f.write_char('0'),
            Self::Foreground(fg) => fg.write_fg(f),
            Self::Background(bg) => bg.write_bg(f),
            Self::Both { fg, bg } => {
                fg.write_fg(f)?;
                f.write_char(';')?;
                bg.write_bg(f)
            }
        }
    }
}

impl Display for StyleDiff {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::None => Ok(()),
            _ => {
                f.write_str("\x1b[")?;
                self.write(f)?;
                f.write_char('m')
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    #[default]
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
}

impl Color {
    fn write_fg(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Default => write!(f, "39"),
            Self::Black => write!(f, "30"),
            Self::Red => write!(f, "31"),
            Self::Green => write!(f, "32"),
            Self::Yellow => write!(f, "33"),
            Self::Blue => write!(f, "34"),
            Self::Magenta => write!(f, "35"),
            Self::Cyan => write!(f, "36"),
            Self::White => write!(f, "37"),
            Self::BrightBlack => write!(f, "90"),
            Self::BrightRed => write!(f, "91"),
            Self::BrightGreen => write!(f, "92"),
            Self::BrightYellow => write!(f, "93"),
            Self::BrightBlue => write!(f, "94"),
            Self::BrightMagenta => write!(f, "95"),
            Self::BrightCyan => write!(f, "96"),
            Self::BrightWhite => write!(f, "97"),
            Self::Rgb { r, g, b } => write!(f, "38;5;{r};{g};{b}"),
        }
    }

    fn write_bg(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Default => write!(f, "49"),
            Self::Black => write!(f, "40"),
            Self::Red => write!(f, "41"),
            Self::Green => write!(f, "42"),
            Self::Yellow => write!(f, "43"),
            Self::Blue => write!(f, "44"),
            Self::Magenta => write!(f, "45"),
            Self::Cyan => write!(f, "46"),
            Self::White => write!(f, "47"),
            Self::BrightBlack => write!(f, "100"),
            Self::BrightRed => write!(f, "101"),
            Self::BrightGreen => write!(f, "102"),
            Self::BrightYellow => write!(f, "103"),
            Self::BrightBlue => write!(f, "104"),
            Self::BrightMagenta => write!(f, "105"),
            Self::BrightCyan => write!(f, "106"),
            Self::BrightWhite => write!(f, "107"),
            Self::Rgb { r, g, b } => write!(f, "48;5;{r};{g};{b}"),
        }
    }
}

pub struct Styled<T>
where
    T: Display,
{
    value: T,
    style: Style,
}

impl<T> Styled<T>
where
    T: Display,
{
    fn with_fg(self, color: Color) -> Self {
        Self {
            style: self.style.with_fg(color),
            ..self
        }
    }

    fn with_bg(self, color: Color) -> Self {
        Self {
            style: self.style.with_bg(color),
            ..self
        }
    }
}

impl<T> std::fmt::Display for Styled<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let style = self.style.diff(&Style::CLEAR);
        let clear = Style::CLEAR.diff(&self.style);
        write!(f, "{}{}{}", style, self.value, clear)
    }
}

pub trait ToStyled: Display {
    fn wrap(&self) -> Styled<&Self> {
        Styled {
            value: self,
            style: Style::default(),
        }
    }

    fn with_fg(&self, color: Color) -> Styled<&Self> {
        self.wrap().with_fg(color)
    }

    fn with_bg(&self, color: Color) -> Styled<&Self> {
        self.wrap().with_bg(color)
    }
}

impl<T> ToStyled for T where T: Display {}
