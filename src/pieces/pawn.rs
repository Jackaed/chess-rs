use crate::{
    board::Board, half_move::HalfMove, piece::Color, pieces::PieceType, position::Position,
};

pub static PAWN: PieceType = PieceType {
    name: "Pawn",
    piece_index: 0,
    generate_pseudo_legal_moves: generate_moves,
    piece_char: 'p',
};

fn generate_moves(board: &Board, color: Color, position: Position) -> Box<[HalfMove]> {
    println!("Pawn moves");
    todo!();
}
