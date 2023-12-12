#[derive(Debug, PartialEq)]
pub enum PipeDirection {
    Vertical,    // |
    Horizontal,  // -
    NorthToEast, // L
    NorthToWest, // J
    SouthToWest, // 7
    SouthToEast, // F
    Ground,      // .
    StartingPos, // S
}

impl PipeDirection {
    pub fn can_go_to(&self) -> [Direction; 2] {
        match self {
            PipeDirection::Vertical => [Direction::Up, Direction::Down],
            PipeDirection::Horizontal => [Direction::Left, Direction::Right],
            PipeDirection::NorthToEast => [Direction::Up, Direction::Right],
            PipeDirection::NorthToWest => [Direction::Up, Direction::Left],
            PipeDirection::SouthToWest => [Direction::Down, Direction::Left],
            PipeDirection::SouthToEast => [Direction::Down, Direction::Right],
            PipeDirection::Ground => [Direction::None, Direction::None],
            PipeDirection::StartingPos => [Direction::None, Direction::None],
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::None => Direction::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate(pub isize, pub isize);

impl Coordinate {
    pub fn add(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }

    pub fn go_to(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Left => Coordinate(self.0, self.1 - 1),
            Direction::Right => Coordinate(self.0, self.1 + 1),
            Direction::Up => Coordinate(self.0 - 1, self.1),
            Direction::Down => Coordinate(self.0 + 1, self.1),
            Direction::None => Coordinate(self.0, self.1),
        }
    }
}
