use day6::guard::Guard;
use day6::map::Map;
use day6::movement::{Direction, Position};

fn main() {
    let mut map = Map::new();

    let position: Position = Map::find_guard(&map);
    let initial_position = position.clone();
    let direction = Direction::North;
    let distinct_tiles = 0;
    let mut guard = Guard { position, initial_position, direction, distinct_tiles, looping: false, path_record: Vec::new() };
    guard.patrol(&mut map);

    let mut possible_loops = 0;

    println!("Guard walked through {} distinct tiles", guard.distinct_tiles);

    for row_index in 0..map.tiles.len() {
        for col_index in 0..map.tiles[row_index].len() {
            if map.tiles[row_index][col_index].tile == '.' {
                map.tiles[row_index][col_index].tile = '#';

                guard.position = guard.initial_position.clone();
                guard.direction = Direction::North;
                guard.path_record.clear();
                guard.looping = false;
                guard.patrol(&mut map);
                if guard.looping {
                    possible_loops += 1;
                }

                map.tiles[row_index][col_index].tile = '.';
            }
        }
    }

    println!("Possible loops: {}", possible_loops);
}
