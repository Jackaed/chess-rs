use crate::{board::Board, piece::Color, player::Player};

pub struct Game<'a> {
    white: &'a dyn Player,
    black: &'a dyn Player,
    board: Board,
}

impl<'a> Game<'a> {
    pub fn new(white: &'a dyn Player, black: &'a dyn Player) -> Self {
        Game {
            white,
            black,
            board: Board::new(),
        }
    }

    pub fn play(&mut self) {
        loop {
            let piece_move = if self.board.current_turn() == Color::White {
                self.white.suggest_move(&self.board)
            } else {
                self.black.suggest_move(&self.board)
            };
            self.board.move_piece(&piece_move).unwrap();
        }
    }
}
