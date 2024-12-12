use day11::stone::StoneSet;

fn main() {
    let mut stone_set = StoneSet::new("input_mini.txt");
    for _ in 0..25 {
        stone_set.mutate();
    }

    println!("{}", stone_set.stones.len());
}
