use day6::guard::Guard;
use day6::map::Map;
use day6::movement::{Direction, Position};

fn main() {
    let mut map = Map::new();

    let position: Position = Map::find_guard(&map);
    let direction = Direction::North;
    let distinct_tiles = 0;
    let mut guard = Guard { position, direction, distinct_tiles };
    guard.patrol(&mut map);

    println!("Guard walked through {} distinct tiles", guard.distinct_tiles);
}
