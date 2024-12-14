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

pub struct CornerList {
    pub corners: Vec<Corner>,
}

#[derive(Debug)]
pub struct Corner {
    pub coordinates: Vec<ICoordinate>,
    pub corner_type: CornerType,
}

impl Coordinate {
    pub fn corner_list(&self) -> CornerList {
        let mut corner_list = CornerList { corners: Vec::new() };

        for corner_type in CornerType::iterator() {
            let corner = Corner {
                coordinates: self.corner_coordinates(&corner_type),
                corner_type,
            };

            corner_list.corners.push(corner);
        }

        corner_list
    }

    pub fn corner_coordinates(&self, corner_type: &CornerType) -> Vec<ICoordinate> {
        let movements = match corner_type {
            CornerType::NorthEast => vec![(0, -1), (1, -1), (1, 0)],
            CornerType::SouthEast => vec![(1, 0), (1, 1), (0, 1)],
            CornerType::SouthWest => vec![(0, 1), (-1, 1), (-1, 0)],
            CornerType::NorthWest => vec![(-1, 0), (-1, -1), (0, -1)],
        };

        movements.iter().map(|movement| {
            ICoordinate {
                x: self.x as isize + movement.0,
                y: self.y as isize + movement.1,
            }
        }).collect()
    }
}

pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Clone, Debug)]
pub enum CornerType {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

impl CornerType {
    fn iterator() -> impl Iterator<Item = CornerType> {
        [
            CornerType::NorthEast,
            CornerType::SouthEast,
            CornerType::SouthWest,
            CornerType::NorthWest,
        ].iter().cloned()
    }
}
