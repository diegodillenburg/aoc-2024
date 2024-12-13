#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct ICoordinate {
    pub x: isize,
    pub y: isize,
}
