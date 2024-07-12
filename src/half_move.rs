use crate::position::Position;

pub struct HalfMove {
    pub from: Position,
    pub to: Position,
}

impl HalfMove {
    pub const fn new(from: Position, to: Position) -> Self {
        Self { from, to }
    }

    pub const fn from(&self) -> &Position {
        &self.from
    }

    pub const fn to(&self) -> &Position {
        &self.to
    }
}
