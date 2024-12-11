use crate::trail_map::{Coordinate, Tile};
use std::collections::hash_set::HashSet;

#[derive(Debug)]
pub struct Trailhead {
    pub coordinate: Coordinate,
    pub peaks: HashSet<Coordinate>,
}

impl Trailhead {
    pub fn new(coordinate: Coordinate, topography: &Vec<Vec<Tile>>) -> Trailhead {
        Trailhead {
            coordinate,
            peaks: HashSet::new(),
        }
    }
}
