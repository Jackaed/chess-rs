use crate::{
    board::Board, half_move::HalfMove, piece::Color, pieces::PieceType, position::Position,
};

pub static ROOK: PieceType = PieceType {
    name: "Rook",
    piece_index: 3,
    generate_pseudo_legal_moves: generate_moves,
    piece_char: 'r',
};

fn generate_moves(board: &Board, color: Color, position: Position) -> Box<[HalfMove]> {
    _ = (board, color, position);
    println!("Rook moves");
    todo!();
}
