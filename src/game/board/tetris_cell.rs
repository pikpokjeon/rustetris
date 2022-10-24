use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TetrisCell {
    Empty = "white",
    Ghost = "#697994",
    Red = "red",
    Green = "green",
    Blue = "blue",
    Purple = "purple",
    Cyan = "cyan",
    Orange = "orange",
    Yellow = "yellow",
}

impl Default for TetrisCell {
    fn default() -> Self {
        Self::Empty
    }
}

impl std::convert::TryFrom<i32> for TetrisCell {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, ()> {
        match value {
            0 => Ok(TetrisCell::Empty),
            1 => Ok(TetrisCell::Red),
            2 => Ok(TetrisCell::Green),
            3 => Ok(TetrisCell::Blue),
            4 => Ok(TetrisCell::Purple),
            5 => Ok(TetrisCell::Cyan),
            6 => Ok(TetrisCell::Orange),
            7 => Ok(TetrisCell::Yellow),
            8 => Ok(TetrisCell::Ghost),
            _ => Err(()),
        }
    }
}

impl TetrisCell {
    pub fn is_empty(&self) -> bool {
        self == &Self::Empty
    }

    pub fn into_code(&self) -> i32 {
        match self {
            Self::Empty => 0,
            Self::Red => 1,
            Self::Green => 2,
            Self::Blue => 3,
            Self::Purple => 4,
            Self::Cyan => 5,
            Self::Orange => 6,
            Self::Yellow => 7,
            Self::Ghost => 8,
            _ => 0,
        }
    }

    pub fn to_color(&self) -> &str {
        match self {
            Self::Empty => "white",
            Self::Red => "#f44336",
            Self::Green => "#53c858",
            Self::Blue => "#2157f3",
            Self::Purple => "#673ab7",
            Self::Cyan => "#32f6de",
            Self::Orange => "#ff9800",
            Self::Yellow => "#ffeb3b",
            Self::Ghost => "#697994",
            _ => "white",
        }
    }
}
