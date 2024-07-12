use crate::{
    board::Board, half_move::HalfMove, piece::Color, pieces::PieceType, position::Position,
};

pub static BISHOP: PieceType = PieceType {
    name: "BISHOP",
    piece_index: 2,
    generate_pseudo_legal_moves: generate_moves,
    piece_char: 'b',
};

fn generate_moves(board: &Board, color: Color, position: Position) -> Box<[HalfMove]> {
    println!("Bishop moves");
    todo!();
}
