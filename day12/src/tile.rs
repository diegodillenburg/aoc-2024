use crate::coordinate::Coordinate;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Tile {
    pub coordinate: Coordinate,
    pub kind: char,
}
