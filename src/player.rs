use crate::{board::Board, half_move::HalfMove};

pub trait Player {
    fn suggest_move(&self, board: &Board) -> HalfMove;
}
