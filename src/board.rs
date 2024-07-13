use std::fmt::Display;

use enum_map::{enum_map, Enum, EnumMap};
use strum::IntoEnumIterator;

use crate::{
    errors::{InvalidChar, InvalidFenString, InvalidMove},
    half_move::HalfMove,
    piece::{Color, Piece},
    pieces,
    position::{self, Axis, Position},
};

pub struct Board {
    bitboard: EnumMap<Color, [Bitboard; 8]>,
    castling_rights: CastlingRights,
    full_move_clock: u32,
    half_move_clock: u32,
    current_turn: Color,
    en_passant_target: Option<Position>,
}

impl Board {
    pub const fn current_turn(&self) -> Color {
        self.current_turn
    }

    pub fn empty() -> Self {
        Self {
            bitboard: enum_map! { _ => [Bitboard::new(); 8] },
            castling_rights: CastlingRights::new(),
            full_move_clock: 0,
            half_move_clock: 0,
            current_turn: Color::White,
            en_passant_target: None,
        }
    }

    fn get(&self, position: Position) -> Option<Piece> {
        for ptype in pieces::PIECE_TYPES {
            for color in Color::iter() {
                if self.bitboard[color][ptype.index() as usize].get(position) {
                    return Some(Piece::new(ptype, color));
                }
            }
        }
        None
    }

    fn set(&mut self, piece: &Piece, position: Position) {
        self.bitboard[piece.color()][piece.ptype().index() as usize].set(position);
    }

    fn remove(&mut self, position: Position) {
        for color in Color::iter() {
            for bitboard in &mut self.bitboard[color] {
                if bitboard.get(position) {
                    bitboard.remove(position);
                    return;
                }
            }
        }
    }

    pub fn move_piece(&mut self, half_move: &HalfMove) -> Result<(), InvalidMove> {
        let piece = self.get(*half_move.from()).ok_or(InvalidMove {
            reason: "No piece at position 'from'".to_string(),
        })?;
        self.remove(*half_move.from());
        self.set(&piece, *half_move.to());
        Ok(())
    }

    pub fn new() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn from_fen(fen_string: &str) -> Result<Self, InvalidFenString> {
        let mut rank_iter = Axis::iter();
        let mut file_iter = Axis::iter();
        let mut rank = rank_iter.next();
        let mut file = file_iter.next();
        let mut field_num: u8 = 0;
        let mut board: Self = Self::empty();
        board.castling_rights = CastlingRights {
            data: enum_map! { _ => enum_map! {_ => false} },
        };

        let mut fen_string_iter = fen_string.chars();
        while let Some(character) = fen_string_iter.next() {
            if character == ' ' {
                field_num += 1;
                continue;
            }

            match field_num {
                0 => Self::parse_char_in_first_fen_field(
                    character,
                    &mut rank,
                    &mut rank_iter,
                    &mut file_iter,
                    &mut file,
                    &mut board,
                )?,
                1 => {
                    board.current_turn =
                        Color::try_from(character).map_err(|_| InvalidFenString {})?
                }
                2 => {
                    if character == '-' {
                        continue;
                    }

                    let colour = if character.is_lowercase() {
                        Color::White
                    } else {
                        Color::Black
                    };

                    let board_side = BoardSide::try_from(character).map_err(|_| InvalidFenString{})?;
                    board.castling_rights.data[colour][board_side] = true;
                }
                3 => {
                    if character == '-' {
                        continue;
                    }

                    board.en_passant_target = Some(
                        Position::try_from([
                            character,
                            fen_string_iter.next().ok_or(InvalidFenString {})?,
                        ])
                        .map_err(|_| InvalidFenString {})?,
                    );
                }
                4 | 5 => {
                    let val = character.to_digit(10).ok_or(InvalidFenString {})?;

                    if field_num == 4 {
                        board.half_move_clock = val;
                    } else if field_num == 5 {
                        board.full_move_clock = val;
                    }
                }

                _ => return Err(InvalidFenString {}),
            }
        }

        Ok(board)
    }

    fn parse_char_in_first_fen_field(
        character: char,
        rank: &mut Option<position::Axis>,
        rank_iter: &mut position::AxisIter,
        file_iter: &mut position::AxisIter,
        file: &mut Option<position::Axis>,
        board: &mut Self,
    ) -> Result<(), InvalidFenString> {
        if character == '/' {
            *rank = rank_iter.next();
            *file_iter = position::Axis::iter();
            *file = file_iter.next();
        } else if let Some(digit) = character.to_digit(10) {
            for _ in 0..digit {
                *file = file_iter.next();
            }
        } else {
            if let Some(rank) = *rank {
                if let Some(file) = *file {
                    board.set(
                        &Piece::try_from(character).map_err(|_| InvalidFenString {})?,
                        Position::new(rank, file),
                    );
                }
            }
            *file = file_iter.next();
        };
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Bitboard {
    data: u64,
}

impl Default for Bitboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Bitboard {
    pub const fn new() -> Self {
        Self { data: 0 }
    }

    pub const fn get(self, position: Position) -> bool {
        self.data & (1 << position.index()) != 0
    }

    pub fn set(&mut self, position: Position) {
        self.data |= 1 << (position.index());
    }

    pub fn remove(&mut self, position: Position) {
        self.data &= !(1 << (position.index()));
    }

    pub const fn from_data(data: u64) -> Self {
        Self { data }
    }

    pub const fn data(self) -> u64 {
        self.data
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for rank in position::Axis::iter() {
            for file in position::Axis::iter() {
                let square = self.get(Position::new(rank, file));
                out = format!(
                    "{}{}",
                    out,
                    square.map_or_else(|| "-".to_owned(), |piece| piece.to_string())
                );
                out += " ";
            }
            out += "\n";
        }
        out += "\n";

        write!(f, "{out}")
    }
}

#[derive(Enum, Clone, Copy)]
enum BoardSide {
    QueenSide,
    KingSide,
}

impl TryFrom<char> for BoardSide {
    type Error = InvalidChar;
    fn try_from(character: char) -> Result<Self, Self::Error> {
        match character.to_ascii_uppercase() {
            'K' => Ok(Self::KingSide),
            'Q' => Ok(Self::QueenSide),
            _ => Err(InvalidChar {}),
        }
    }
}

#[derive(Clone, Copy)]
struct CastlingRights {
    data: EnumMap<Color, EnumMap<BoardSide, bool>>,
}

impl CastlingRights {
    pub fn new() -> Self {
        Self {
            data: enum_map! { _ => enum_map! { _ => true} },
        }
    }

    pub fn get(&self, colour: Color, side: BoardSide) -> &bool {
        &self.data[colour][side]
    }

    pub fn remove(&mut self, colour: Color, side: BoardSide) {
        self.data[colour][side] = false;
    }
}
