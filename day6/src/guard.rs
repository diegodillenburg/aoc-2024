use crate::map::{Map, Tile};
use crate::movement::{Direction, Position};

#[derive(Debug)]
pub struct Guard {
    pub position: Position,
    pub direction: Direction,
    pub distinct_tiles: usize,
}

impl Guard {
    pub fn patrol(&mut self, map: &mut Map) -> usize {
        while let Some(_next_movement) = Guard::walk(self, map) { }
        self.distinct_tiles
    }

    pub fn next_move(&self, map_height: usize, map_width: usize) -> Option<Position> {
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

    pub fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    pub fn perform_move(&mut self, map_height: usize, map_width: usize) {
        match self.next_move(map_height, map_width) {
            Some(next_position) => {
                self.position = next_position;
            },
            None => ()
        }
    }

    pub fn walk(guard: &mut Guard, map: &mut Map) -> Option<Tile> {
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
