use crate::{board::Board, half_move::HalfMove, piece::Color, position::Position};
mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;
pub use bishop::BISHOP;
pub use king::KING;
pub use knight::KNIGHT;
pub use pawn::PAWN;
pub use queen::QUEEN;
pub use rook::ROOK;

pub struct PieceType {
    pub name: &'static str,
    pub generate_pseudo_legal_moves: fn(&Board, Color, Position) -> Box<[HalfMove]>,
    pub piece_index: usize,
    pub piece_char: char,
}

pub static PIECE_TYPES: [&PieceType; 6] = [&PAWN, &KNIGHT, &BISHOP, &ROOK, &QUEEN, &KING];

pub fn get_piece_type_from_char(character: char) -> Option<&'static PieceType> {
    match character.to_ascii_lowercase() {
        'p' => Some(&PAWN),
        'n' => Some(&KNIGHT),
        'b' => Some(&BISHOP),
        'r' => Some(&ROOK),
        'q' => Some(&QUEEN),
        'k' => Some(&KING),
        _ => None,
    }
}

impl PieceType {
    pub const fn piece_char(&self) -> char {
        self.piece_char
    }

    pub fn generate_pseudo_legal_moves(
        &self,
        board: &Board,
        color: Color,
        position: Position,
    ) -> Box<[HalfMove]> {
        (self.generate_pseudo_legal_moves)(board, color, position)
    }

    pub const fn index(&self) -> usize {
        self.piece_index
    }
}
