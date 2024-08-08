use crate::{board::Board, half_move::HalfMove};

pub trait Player: Send + Sync {
    fn suggest_move(&self, board: &Board) -> HalfMove;
}
