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
        Self {
            white,
            black,
            board,
            sender,
            receiver,
        }
    }

    pub fn play(&mut self) -> Outcome {
        loop {
            if let Some(outcome) = self.board.get_outcome() {
                return outcome;
            }

            let current_player: &dyn Player = match self.board.current_turn() {
                Color::White => &self.white,
                Color::Black => &self.black,
            };

            // TODO: do something with these returned values
            let _ = self.board
                .move_piece(&current_player.suggest_move(&self.board));

            let _ = self.sender.send(self.board);
        }
    }
}
