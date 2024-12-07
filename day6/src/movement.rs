#[derive(Eq, Clone, Debug, Hash, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Waypoint {
    pub position: Position,
    pub direction: Direction,
}
