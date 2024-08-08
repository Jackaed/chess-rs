use tokio::sync::watch::{self, Receiver, Sender};

use crate::{
    board::{Board, Outcome},
    piece::Color,
    player::Player,
};

pub struct Game<W: Player, B: Player> {
    white: W,
    black: B,
    board: Board,
    sender: Sender<Board>,
    receiver: Receiver<Board>,
}

impl<W: Player, B: Player> Game<W, B> {
    pub fn view(&self) -> Receiver<Board> {
        self.receiver.clone()
    }

    pub fn new(white: W, black: B) -> Self {
        let board = Board::new();
        let (sender, receiver) = watch::channel(board);
        Game {
            white,
            black,
            board,
            sender,
            receiver,
        }
    }

    pub fn play(&mut self) -> Outcome {
        loop {
            if let Some(Outcome) = self.board.get_outcome() {
                return Outcome;
            }

            let current_player: &dyn Player = match self.board.current_turn() {
                Color::White => &self.white,
                Color::Black => &self.black,
            };

            self.board
                .move_piece(&current_player.suggest_move(&self.board));

            self.sender.send(self.board);
        }
    }
}
