use day8::city_map::CityMap;

fn main() {
    let mut city_map = CityMap::new("input.txt");
    city_map.map_influence_zone();
    city_map.draw();
    println!("Influence Zone Nodes: {}", city_map.influence_zone.len());
    println!("Width: {} Height: {}", city_map.width, city_map.height);
    println!("Antenna Pairs: {}", city_map.antenna_pairs.len());
    println!("Antenna Frequencies: {:?}", city_map.antenna_frequencies());
}
