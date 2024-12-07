use std::fs;
use crate::movement::Position;

#[derive(Clone, Debug)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            tiles: Map::build_tiles(),
        }
    }

    fn build_tiles() -> Vec<Vec<Tile>> {
        let map = fs::read_to_string("input.txt")
            .expect("there was an issue reading the file");

        let map: Vec<&str> = map.split("\n").collect();
        let tiles: Vec<Vec<Tile>> = map
            .iter()
            .map(|row| {
                row.chars()
                    .map(|tile| Tile {
                        tile,
                        visited: false,
                    }).collect()
            }).collect();

        tiles
    }

    pub fn find_guard(map: &Map) -> Position {
        for (row_index, row) in map.tiles.iter().enumerate() {
            for (column_index, tile) in row.iter().enumerate() {
                if tile.tile == '^' {
                    return Position { x: row_index, y: column_index }
                }
            }
        }

        Position { x: 0, y: 0 } // don't care it works anyway
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub tile: char,
    pub visited: bool,
}

impl Tile {
    pub fn walkable(&self) -> bool {
        if self.tile != '#' {
            true
        } else {
            false
        }
    }
}

