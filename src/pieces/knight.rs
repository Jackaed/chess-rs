use crate::{
    board::Board, half_move::HalfMove, piece::Color, pieces::PieceType, position::Position,
};

pub static KNIGHT: PieceType = PieceType {
    name: "Knight",
    piece_index: 1,
    generate_pseudo_legal_moves: generate_moves,
    piece_char: 'n',
};

fn generate_moves(board: &Board, color: Color, position: Position) -> Box<[HalfMove]> {
    println!("Knight moves");
    todo!();
}
