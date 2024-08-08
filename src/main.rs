#![warn(clippy::pedantic, clippy::nursery)]
#![feature(iter_intersperse)]
#![allow(dead_code)]

use std::{thread, time::Duration};

use game::Game;
use player::Player;
use position::{Axis, Position};

mod board;
mod errors;
mod game;
mod half_move;
mod piece;
mod pieces;
mod player;
mod position;

fn main() {
    let player1 = TestPlayer {};
    let player2 = TestPlayer {};
    let mut game = Game::new(player1, player2);
    let board_view = game.view();
    thread::spawn(move || {
        game.play();
    });

    loop {
        println!("{}\n", board_view.borrow().to_string());
        thread::sleep(Duration::from_millis(200));
    }
}

struct TestPlayer {}

impl Player for TestPlayer {
    fn suggest_move(&self, board: &board::Board) -> half_move::HalfMove {
        thread::sleep(Duration::from_secs(5));
        half_move::HalfMove::new(
            Position::new(Axis::A, Axis::A),
            Position::new(Axis::B, Axis::B),
        )
    }
}
