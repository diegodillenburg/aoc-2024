use std::{collections::HashMap, fs};
pub struct StoneSet {
    pub stones: HashMap<usize, usize>,
}

impl StoneSet {
    pub fn new(path: &str) -> StoneSet {
        let numbers = fs::read_to_string(path).expect("something wrong reading the file");

        let mut stones: HashMap<usize, usize> = HashMap::new();

        numbers.trim().split(" ").for_each(|number| {
            let number = number.parse::<usize>().unwrap();
            stones.entry(number).or_insert(1);
        });

        StoneSet { stones }
    }

    pub fn mutate(&mut self) {
        let mut new_stones: HashMap<usize, usize> = HashMap::new();
        self.stones.iter().for_each(|(stone, count)| {
            if *stone == 0 {
                StoneSet::add_stone(&mut new_stones, &1, count);
            } else if StoneSet::even_digits(stone) {
                let (stone_a, stone_b) = StoneSet::digits_pairs(stone);
                StoneSet::add_stone(&mut new_stones, &stone_a, count);
                StoneSet::add_stone(&mut new_stones, &stone_b, count);


            } else {
                let stone = stone * 2024;
                StoneSet::add_stone(&mut new_stones, &stone, count);
            }
        });

        self.stones = new_stones;
    }

    fn add_stone(stones: &mut HashMap<usize, usize>, stone: &usize, count: &usize) {
        stones.entry(*stone)
            .and_modify(|value| *value += count)
            .or_insert(*count);
    }

    pub fn digits_pairs(stone: &usize) -> (usize, usize) {
        let digits = stone.to_string();
        let lenght = digits.len();

        (
            digits[0..lenght / 2].parse::<usize>().unwrap(),
            digits[lenght / 2..lenght].parse::<usize>().unwrap(),
        )
    }

    fn even_digits(stone: &usize) -> bool {
        stone.to_string().len() % 2 == 0
    }
}
