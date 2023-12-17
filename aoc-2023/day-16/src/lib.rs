pub type Position = (usize, usize);

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Beam {
    pub pos: Position,
    pub dir: Direction,
}
