use day10::trail_map::TrailMap;

fn main() {
    let trail_map = TrailMap::new("input.txt");
    println!("{}", trail_map.score());
}
