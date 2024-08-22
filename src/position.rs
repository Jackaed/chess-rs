use enum_map::Enum;
use strum_macros::{EnumIter, FromRepr};

use crate::errors::{InvalidChar, PositionOutOfBounds};

#[derive(Enum, Clone, Copy, Debug)]
pub struct Position {
    rank: Axis,
    file: Axis,
}

impl Position {
    pub const fn new(rank: Axis, file: Axis) -> Self {
        Self { rank, file }
    }

    pub const fn rank(&self) -> &Axis {
        &self.rank
    }

    pub const fn file(&self) -> &Axis {
        &self.file
    }

    pub const fn index(self) -> usize {
        self.rank as usize * 8 + self.file as usize
    }
}

impl TryFrom<[char; 2]> for Position {
    type Error = InvalidChar;
    fn try_from(chars: [char; 2]) -> Result<Self, Self::Error> {
        let rank = Axis::from_repr(chars[0] as usize - 'A' as usize).ok_or(InvalidChar {})?;
        let file = Axis::from_repr(chars[1].to_digit(10).ok_or(InvalidChar {})? as usize)
            .ok_or(InvalidChar {})?;
        Ok(Self::new(rank, file))
    }
}

impl TryFrom<usize> for Position {
    type Error = PositionOutOfBounds;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self::new(
            Axis::from_repr(value % 8).ok_or(Self::Error {})?,
            Axis::from_repr(value / 8).ok_or(Self::Error {})?,
        ))
    }
}

#[derive(Enum, Clone, Copy, EnumIter, FromRepr, Debug)]
pub enum Axis {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
