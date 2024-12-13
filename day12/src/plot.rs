use crate::coordinate::Coordinate;
use crate::node::Node;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Plot {
    pub coordinates: HashSet<Coordinate>,
    pub kind: char,
    pub perimeter: usize,
    pub edges: Vec<Node>,
}

impl Plot {
    pub fn new(kind: char) -> Plot {
        let coordinates: HashSet<Coordinate> = HashSet::new();
        let perimeter = 0;
        let edges = Vec::new();


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

    pub fn edge_count(&self) -> usize {
        self.edges.iter().map(|node| node.children.len()).sum()
    }
}


