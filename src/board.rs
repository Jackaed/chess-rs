use std::fmt::Display;

use enum_map::{enum_map, Enum, EnumMap};
use strum::IntoEnumIterator;

use crate::{
    bitboard::Bitboard,
    errors::{InvalidChar, InvalidFenString, InvalidMove},
    half_move::HalfMove,
    piece::{Color, Piece},
    piece_map::PieceMap,
    pieces,
    position::{self, Axis, Position},
};

#[derive(Clone, Copy)]
pub struct Board {
    data: PieceMap<Bitboard>,
    castling_rights: CastlingRights,
    full_move_clock: u32,
    half_move_clock: u32,
    current_turn: Color,
    en_passant_target: Option<Position>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub const fn current_turn(&self) -> Color {
        self.current_turn
    }

    pub fn empty() -> Self {
        Self {
            data: PieceMap::new(|_| Bitboard::new()),
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
                if self.data.get(&Piece::new(ptype, color)).get(position) {
                    return Some(Piece::new(ptype, color));
                }
            }
        }
        None
    }

    fn set(&mut self, piece: &Piece, position: Position) {
        self.data.get_mut(piece).set(position);
    }

    fn remove(&mut self, position: Position) {
        for ptype in pieces::PIECE_TYPES {
            for color in Color::iter() {
                let bitboard = self.data.get_mut(&Piece::new(ptype, color));
                if bitboard.get(position) {
                    bitboard.remove(position);
                    return;
                }
            }
        }
    }

    pub fn move_piece(&mut self, half_move: &HalfMove) -> Result<(), InvalidMove> {
        let piece = self.get(*half_move.from()).ok_or_else(|| InvalidMove {
            reason: "No piece at position 'from'".to_string(),
        })?;
        self.remove(*half_move.from());
        self.remove(*half_move.to());
        self.set(&piece, *half_move.to());
        Ok(())
    }

    pub fn new() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("FEN string for starting board failed to be parsed")
    }

    fn parse_data_fen_field(string: &str) -> Result<PieceMap<Bitboard>, InvalidFenString> {
        let rank_iter = Axis::iter().rev();
        let mut board = PieceMap::new(|_| Bitboard::new());
        let lines = string.split('/');
        for (line, rank) in lines.zip(rank_iter) {
            Self::parse_line_in_fen_data_field(line, rank, &mut board)?;
        }
        Ok(board)
    }

    fn parse_line_in_fen_data_field(
        line: &str,
        rank: Axis,
        board: &mut PieceMap<Bitboard>,
    ) -> Result<(), InvalidFenString> {
        let mut file_iter = Axis::iter();
        let mut file = file_iter.next();
        for character in line.chars() {
            if let Some(digit) = character.to_digit(10) {
                for _ in 0..digit {
                    file = file_iter.next();
                }
            } else {
                if let Some(file) = file {
                    let position = Position::new(rank, file);
                    let piece = &Piece::try_from(character).map_err(|_| InvalidFenString {})?;
                    board.get_mut(piece).set(position);
                }
                file = file_iter.next();
            };
        }
        Ok(())
    }

    fn parse_current_turn_fen_field(string: &str) -> Result<Color, InvalidFenString> {
        let character = (string.chars().next().ok_or(InvalidFenString {}))?;
        Color::try_from(character).map_err(|_| InvalidFenString {})
    }

    fn parse_castling_rights_fen_field(string: &str) -> Result<CastlingRights, InvalidFenString> {
        let mut out = CastlingRights::new();
        for character in string.chars() {
            let colour = match character {
                '-' => return Ok(out),
                c if c.is_lowercase() => Color::White,
                c if c.is_uppercase() => Color::Black,
                _ => return Err(InvalidFenString {}),
            };

            let board_side = BoardSide::try_from(character).map_err(|_| InvalidFenString {})?;
            out.data[colour][board_side] = true;
        }
        Ok(out)
    }

    fn parse_en_passant_fen_field(string: &str) -> Result<Option<Position>, InvalidFenString> {
        let mut characters = string.chars();

        let first_character = characters.next().ok_or(InvalidFenString {})?;
        if first_character == '-' {
            return Ok(None);
        };
        let second_character = characters.next().ok_or(InvalidFenString {})?;

        if characters.next().is_some() {
            return Err(InvalidFenString {});
        }

        Ok(Some(
            Position::try_from([first_character, second_character])
                .map_err(|_| InvalidFenString {})?,
        ))
    }

    fn parse_move_clock_fen_field(string: &str) -> Result<u32, InvalidFenString> {
        string.parse::<u32>().map_err(|_| InvalidFenString {})
    }

    pub fn from_fen(fen_string: &str) -> Result<Self, InvalidFenString> {
        let fen_fields: Vec<&str> = fen_string.split(' ').collect();
        Ok(Self {
            data: Self::parse_data_fen_field(fen_fields[0])?,
            current_turn: Self::parse_current_turn_fen_field(fen_fields[1])?,
            castling_rights: Self::parse_castling_rights_fen_field(fen_fields[2])?,
            en_passant_target: Self::parse_en_passant_fen_field(fen_fields[3])?,
            half_move_clock: Self::parse_move_clock_fen_field(fen_fields[4])?,
            full_move_clock: Self::parse_move_clock_fen_field(fen_fields[5])?,
        })
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

    pub const fn get_outcome(&self) -> Option<Outcome> {
        _ = self;
        None
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out: String = position::Axis::iter()
            .rev()
            .map(|rank| self.rank_to_string(rank))
            .intersperse("\n".to_string())
            .collect();

        write!(f, "{out}")
    }
}

impl Board {
    fn iterate_over_rank(&self, rank: Axis) -> impl Iterator<Item = Option<Piece>> + '_ {
        Axis::iter().map(move |file| self.get(Position::new(rank, file)))
    }

    fn rank_to_string(&self, rank: Axis) -> String {
        self.iterate_over_rank(rank)
            .map(|square| square.map_or_else(|| '-', |piece| (&piece).into()))
            .intersperse(' ')
            .collect()
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

pub enum Outcome {
    Checkmate(Color),
    Stalemate,
}
