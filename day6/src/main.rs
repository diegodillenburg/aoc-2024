use day6::guard::Guard;
use day6::map::Map;

fn main() {
    let mut map = Map::new();

    let initial_position = Map::find_guard(&map);
    let mut guard = Guard::new(initial_position);
    guard.patrol(&mut map);

    let mut possible_loops = 0;

    println!("Guard walked through {} distinct tiles", guard.distinct_tiles.len());

    let distinct_tiles = guard.distinct_tiles.clone();
    for position in distinct_tiles.iter() {
        map.tiles[position.x][position.y].tile = '#';
        guard.reset();
        guard.patrol(&mut map);
        if guard.looping {
            possible_loops += 1;
        }
        map.tiles[position.x][position.y].tile = '.';
    }

    println!("Possible loops: {}", possible_loops);
}
