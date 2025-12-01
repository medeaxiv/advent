use std::convert::Infallible;

use crate::util::style::Style;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StyledChar {
    pub ch: char,
    pub style: Style,
}

impl From<char> for StyledChar {
    fn from(value: char) -> Self {
        Self {
            ch: value,
            style: Style::default(),
        }
    }
}

pub trait ToStyledChar {
    fn to_styled_char(&self) -> StyledChar;
}

impl ToStyledChar for char {
    fn to_styled_char(&self) -> StyledChar {
        (*self).into()
    }
}

impl ToStyledChar for StyledChar {
    fn to_styled_char(&self) -> StyledChar {
        *self
    }
}

pub trait FromChar: Sized {
    type Err;

    fn from_char(ch: char) -> Result<Self, Self::Err>;
}

impl FromChar for char {
    type Err = Infallible;

    fn from_char(ch: char) -> Result<Self, Self::Err> {
        Ok(ch)
    }
}
