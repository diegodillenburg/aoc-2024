use std::fs;

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn new() -> Map {
        Map {
            tiles: Map::build_tiles(),
        }
    }

    fn build_tiles() -> Vec<Vec<Tile>> {
        let map = fs::read_to_string("input_mini.txt")
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

    fn find_guard(map: &Map) -> Position {
        for (row_index, row) in map.tiles.iter().enumerate() {
            for (column_index, tile) in row.iter().enumerate() {
                if tile.tile == '^' {
                    return Position { x: row_index, y: column_index }
                }
            }
        }

        Position { x: 0, y: 0 } // don't care it works anyway
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }
}

#[derive(Debug)]
struct Guard {
    position: Position,
    direction: Direction,
    distinct_tiles: usize,
}

impl Guard {
    fn patrol(&mut self, map: &mut Map) -> usize {
        while let Some(_next_movement) = Guard::walk(self, map) { }
        self.distinct_tiles
    }

    fn next_move(&self, map_height: usize, map_width: usize) -> Option<Position> {
        match self.direction {
            Direction::North => {
                if self.position.x > 0 {
                    Some(Position {
                        x: self.position.x - 1,
                        y: self.position.y,
                    })
                } else {
                    None
                }
            },
            Direction::East => {
                if self.position.y + 1 < map_width {
                    Some(Position {
                        x: self.position.x,
                        y: self.position.y + 1,
                    })
                } else {
                    None
                }
            },
            Direction::South => {
                if self.position.x + 1 < map_height {
                    Some(Position {
                        x: self.position.x + 1,
                        y: self.position.y,
                    })
                } else {
                    None
                }
            },
            Direction::West => {
                if self.position.y > 0 {
                    Some(Position {
                        x: self.position.x,
                        y: self.position.y - 1,
                    })
                } else {
                    None
                }
            },
        }
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    fn perform_move(&mut self, map_height: usize, map_width: usize) {
        match self.next_move(map_height, map_width) {
            Some(next_position) => {
                self.position = next_position;
            },
            None => ()
        }
    }

    fn walk(guard: &mut Guard, map: &mut Map) -> Option<Tile> {
        let map_height = map.height();
        let map_width = map.width();
        if let Some(next_position) = guard.next_move(map_height, map_width) {
            if let Some(row) = map.tiles.get_mut(next_position.x) {
                if let Some(tile) = row.get_mut(next_position.y) {
                    if tile.walkable() {
                        guard.perform_move(map_height, map_width);
                        if !tile.visited {
                            guard.distinct_tiles += 1;
                        }

                        tile.visited = true;
                    } else {
                        guard.turn_right();
                    }

                    return Some(tile.clone());
                }
            }
        }

        None
    }
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone, Debug)]
struct Tile {
    tile: char,
    visited: bool,
}

impl Tile {
    fn walkable(&self) -> bool {
        if self.tile != '#' {
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut map = Map::new();

    let position: Position = Map::find_guard(&map);
    let direction = Direction::North;
    let distinct_tiles = 0;
    let mut guard = Guard { position, direction, distinct_tiles };
    guard.patrol(&mut map);

    println!("Guard walked through {} distinct tiles", guard.distinct_tiles);
}
