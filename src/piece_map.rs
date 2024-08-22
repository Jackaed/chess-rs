use enum_map::{enum_map, EnumMap};

use crate::{
    piece::{Color, Piece},
    pieces::PIECE_TYPES,
};

#[derive(Clone, Copy)]
pub struct PieceMap<T: Clone + Copy> {
    map: EnumMap<Color, [T; 6]>,
}

impl<T: Clone + Copy> PieceMap<T> {
    pub fn get(&self, piece: &Piece) -> &T {
        &self.map[piece.color()][piece.ptype().index()]
    }

    pub fn get_mut(&mut self, piece: &Piece) -> &mut T {
        &mut self.map[piece.color()][piece.ptype().index()]
    }

    pub fn set(&mut self, piece: &Piece, value: T) {
        self.map[piece.color()][piece.ptype().index()] = value;
    }

    pub fn new<F>(created_value: F) -> Self
    where
        F: Fn(Piece) -> T,
    {
        Self {
            map: enum_map! {
                color => {
                    PIECE_TYPES.map(|piece_type| created_value(Piece::new(piece_type, color)))
                }
            },
        }
    }
}
