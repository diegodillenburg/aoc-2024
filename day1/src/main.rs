use std::collections::HashMap;
use std::fs;

fn main() {
    calculate_points_distance();
    calculate_similarity_score();
}

fn calculate_similarity_score() {
    let zipped = parse_input();

    let mut hash_map: HashMap<usize, usize> = HashMap::new();

    zipped.iter().for_each(|(a, _)| {
        hash_map.insert(*a, 0);
    });

    zipped.iter().for_each(|(_, b)| {
        if hash_map.contains_key(&b) {
            hash_map.entry(*b).and_modify(|value| *value += 1);
        }
    });

    let mut result = 0;
    hash_map.iter().for_each(|(key, value)| result += key * value);

    println!("{}", result);
}

fn calculate_points_distance() {
    let zipped = parse_input();

    let distances: Vec<usize> = zipped.into_iter().map(|(a, b)| a.abs_diff(b.clone())).collect();

    println!("{}", distances.iter().sum::<usize>());
}

fn parse_input() -> Vec<(usize, usize)> {
    let file = fs::read_to_string("input.txt").unwrap();
    let lines = file.lines();

    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();

    for line in lines {
        let entries: Vec<_> = line.split("   ").collect();
        let entry:usize = entries[0].parse().unwrap();
        list1.push(entry);
        let entry:usize = entries[1].parse().unwrap();
        list2.push(entry);
    }

    list1.sort();
    list2.sort();

    let zipped: Vec<(usize, usize)> = list1.iter().zip(list2.iter()).map(|(&x, &y)| (x, y)).collect();

    zipped
}
