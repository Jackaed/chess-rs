#[derive(Debug)]
pub struct InvalidChar;
#[derive(Debug)]
pub struct InvalidFenString;
#[derive(Debug)]
pub struct InvalidMove {
    pub reason: String,
}
#[derive(Debug)]
pub struct PositionOutOfBounds;
