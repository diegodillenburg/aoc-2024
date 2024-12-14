use crate::coordinate::Coordinate;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Plot {
    pub coordinates: HashSet<Coordinate>,
    pub kind: char,
    pub perimeter: usize,
    pub edges: usize,
}

impl Plot {
    pub fn new(kind: char) -> Plot {
        let coordinates: HashSet<Coordinate> = HashSet::new();
        let perimeter = 0;
        let edges = 0;

        Plot {
            coordinates,
            kind,
            perimeter,
            edges,
        }
    }

    pub fn area(&self) -> usize {
        self.coordinates.len()
    }

    pub fn safe_perimeter(&self) -> usize {
        if self.coordinates.len() == 1 {
            4
        } else {
            self.perimeter
        }
    }
}


