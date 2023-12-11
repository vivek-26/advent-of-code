#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

impl Direction {
    pub fn get_coords(&self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::None => (0, 0),
        }
    }

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

#[derive(Debug)]
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
