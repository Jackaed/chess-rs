use crate::{
    board::Board, half_move::HalfMove, piece::Color, pieces::PieceType, position::Position,
};

pub static QUEEN: PieceType = PieceType {
    name: "Queen",
    piece_index: 4,
    generate_pseudo_legal_moves: generate_moves,
    piece_char: 'q',
};

fn generate_moves(board: &Board, color: Color, position: Position) -> Box<[HalfMove]> {
    _ = (board, color, position);
    println!("Queen moves");
    todo!();
}
