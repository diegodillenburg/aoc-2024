use crate::trailhead::Trailhead;
use std::fs;

#[derive(Debug)]
pub struct TrailMap {
    pub trailheads: Vec<Trailhead>,
    pub topography: Vec<Vec<Tile>>,
}

impl TrailMap {
    pub fn new(path: &str) -> TrailMap {
        let topography = TrailMap::read_topographic_map(&path);
        let mut trailheads: Vec<Trailhead> = vec![];

        topography.iter().flatten().filter(|tile| tile.height == 0).for_each(|tile| trailheads.push(Trailhead::new(tile.coordinate, &topography)));

        TrailMap {
            trailheads,
            topography,
        }
    }

    pub fn score(&self) -> usize {
        self.trailheads.iter().map(|trailhead| trailhead.score()).sum::<usize>()
    }

    fn read_topographic_map(path: &str) -> Vec<Vec<Tile>> {
        let tiles = fs::read_to_string(path).expect("error reading the file");

        let tiles = tiles
            .split_terminator("\n")
            .enumerate()
            .map(|(j, row)| {
                row.chars()
                    .enumerate()
                    .map(|(i, tile)|
                        Tile {
                        coordinate: Coordinate { x: i, y: j },
                        height: tile.to_digit(10).unwrap() as usize,
                    })
                    .collect()
            })
            .collect();

        tiles
    }
}

#[derive(Debug)]
pub struct Tile {
    pub coordinate: Coordinate,
    pub height: usize,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}
