use crate::{
    board::Board, half_move::HalfMove, piece::Color, pieces::PieceType, position::Position,
};

pub static KING: PieceType = PieceType {
    name: "King",
    piece_index: 5,
    generate_pseudo_legal_moves: generate_moves,
    piece_char: 'k',
};

fn generate_moves(board: &Board, color: Color, position: Position) -> Box<[HalfMove]> {
    println!("King moves");
    todo!();
}
