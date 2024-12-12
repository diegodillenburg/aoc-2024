use day11::stone::StoneSet;

fn main() {
    let mut stone_set = StoneSet::new("input.txt");
    for _ in 0..75 {
        stone_set.mutate();
    }

    println!("{}", stone_set.stones.values().sum::<usize>());
}
