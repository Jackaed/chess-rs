use crate::position::Position;

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
