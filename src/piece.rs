use std::fmt::Display;

use enum_map::Enum;
use strum_macros::EnumIter;

use crate::{
    board::Board, errors::InvalidChar, half_move::HalfMove, pieces, pieces::PieceType,
    position::Position,
};

pub struct Piece {
    ptype: &'static PieceType,
    color: Color,
}

impl Piece {
    pub const fn new(ptype: &'static PieceType, color: Color) -> Self {
        Self { ptype, color }
    }

    pub const fn index(&self) -> u8 {
        (self.color as u8) * 6 + self.ptype.index()
    }

    pub fn get_pseudo_legal_moves(
        &self,
        board: &Board,
        piece_position: Position,
    ) -> Box<[HalfMove]> {
        self.ptype
            .generate_pseudo_legal_moves(board, self.color, piece_position)
    }

    pub const fn color(&self) -> Color {
        self.color
    }

    pub const fn ptype(&self) -> &'static PieceType {
        self.ptype
    }
}

impl TryFrom<char> for Piece {
    type Error = InvalidChar;
    fn try_from(character: char) -> Result<Self, InvalidChar> {
        if !character.is_ascii_alphabetic() {
            return Err(InvalidChar {});
        }

        let colour = if character.is_lowercase() {
            Color::Black
        } else {
            Color::White
        };
        let kind = pieces::get_piece_type_from_char(character);
        kind.map_or(Err(InvalidChar {}), |kind| Ok(Self::new(kind, colour)))
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            let c = self.ptype.piece_char();
            if self.color == Color::White {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            }
        })
    }
}

#[derive(Enum, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl From<Color> for char {
    fn from(val: Color) -> Self {
        match val {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }
}

impl TryFrom<char> for Color {
    type Error = InvalidChar;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(Self::White),
            'b' => Ok(Self::Black),
            _ => Err(InvalidChar {}),
        }
    }
}
