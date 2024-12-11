use crate::trail_map::{Coordinate, Tile};
use std::collections::hash_set::HashSet;

#[derive(Debug)]
pub struct Trailhead {
    pub coordinate: Coordinate,
    pub peaks: Vec<Coordinate>,
}

impl Trailhead {
    pub fn new(coordinate: Coordinate, topography: &Vec<Vec<Tile>>) -> Trailhead {
        let mut peaks = Vec::new();
        Trailhead::traverse(coordinate, topography, 1, &mut peaks);
        Trailhead {
            coordinate,
            peaks,
        }
    }

    pub fn score(&self) -> usize {
        self.peaks.len()
    }

    pub fn traverse(coordinate: Coordinate, topography: &Vec<Vec<Tile>>, step: usize, peaks: &mut Vec<Coordinate>) {
        // N E S W
        let movements = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
        let height = topography.len() as isize;
        let width = topography[0].len() as isize;

        for (dy, dx) in movements {
            let dx = coordinate.x as isize + dx;
            let dy = coordinate.y as isize + dy;
            if dx >= 0 && dx < width && dy >= 0 && dy < height {
                let tile = &topography[dy as usize][dx as usize];
                if tile.height == step {
                    if step == 9 {
                        peaks.push(tile.coordinate);
                        continue;
                    }

                    Trailhead::traverse(tile.coordinate, topography, step + 1, peaks);
                }
            }
        }
    }
}
