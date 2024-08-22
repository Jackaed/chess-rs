#![warn(clippy::pedantic)]
#![feature(iter_intersperse)]
#![allow(dead_code)]

use std::array;

use bitboard::Bitboard;
use errors::PositionOutOfBounds;
use position::{Axis, Position};

mod bitboard;
mod board;
mod errors;
mod game;
mod half_move;
mod piece;
mod piece_map;
mod pieces;
mod player;
mod position;

fn main() {
    let knight_moves: [Bitboard; 64] = array::from_fn(|i| {
        let mut bitboard = Bitboard::new();
        for offset in [
            (1, 2),
            (2, 1),
            (2, -1),
            (1, -2),
            (-1, -2),
            (-2, -1),
            (-2, 1),
            (-1, 2),
        ] {
            let _ = add_to_bitboard_at_offset(
                Position::try_from(i).unwrap(),
                offset.0,
                offset.1,
                &mut bitboard,
            );
        }
        bitboard
    });

    let king_moves: [Bitboard; 64] = array::from_fn(|i| {
        let mut bitboard = Bitboard::new();
        for offset in [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ] {
            let _ = add_to_bitboard_at_offset(
                Position::try_from(i).unwrap(),
                offset.0,
                offset.1,
                &mut bitboard,
            );
        }
        bitboard
    });

    println!(
        "Knight: {:b}\n King: {:b}",
        knight_moves[Position::new(Axis::H, Axis::H).index()].data(),
        king_moves[Position::new(Axis::H, Axis::H).index()].data()
    );
}

fn add_to_bitboard_at_offset(
    src: Position,
    x_offset: i32,
    y_offset: i32,
    bitboard: &mut Bitboard,
) -> Result<(), PositionOutOfBounds> {
    let new_x: usize = (*src.rank() as i32 + x_offset)
        .try_into()
        .map_err(|_| PositionOutOfBounds)?;
    let new_y: usize = (*src.file() as i32 + y_offset)
        .try_into()
        .map_err(|_| PositionOutOfBounds)?;
    let new_rank = Axis::from_repr(new_x).ok_or(PositionOutOfBounds)?;
    let new_file = Axis::from_repr(new_y).ok_or(PositionOutOfBounds)?;
    println!("{src:#?}, {new_rank:#?}, {new_file:#?}");
    bitboard.set(Position::new(new_rank, new_file));
    Ok(())
}
