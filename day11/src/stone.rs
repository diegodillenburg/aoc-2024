use std::fs;

#[derive(Clone, Copy)]
pub struct Stone {
    number: usize,
}

impl Stone {
    pub fn mutate(&mut self) -> Vec<Stone> {
        if self.number == 0 {
            self.number = 1;
        } else if self.even_digits() {
            let (number1, number2) = self.digits_pairs();
            return vec![Stone { number: number1 }, Stone { number: number2 }];
        } else {
            self.number *= 2024;
        }

        vec![*self]
    }

    pub fn even_digits(&self) -> bool {
        self.number.to_string().len() % 2 == 0
    }

    pub fn digits_pairs(&self) -> (usize, usize) {
        let digits = self.number.to_string();
        let lenght = digits.len();

        (
            digits[0..lenght / 2].parse::<usize>().unwrap(),
            digits[lenght / 2..lenght].parse::<usize>().unwrap(),
        )
    }
}

pub struct StoneSet {
    pub stones: Vec<Stone>,
}

impl StoneSet {
    pub fn new(path: &str) -> StoneSet {
        let numbers = fs::read_to_string(path)
            .expect("something wrong reading the file");

        let stones = numbers.trim().split(" ").map(|number| Stone { number: number.parse::<usize>().unwrap() }).collect();

        StoneSet { stones }
    }

    pub fn mutate(&mut self) {
        let mut new_stones: Vec<Stone> = Vec::new();
        self.stones.iter_mut().for_each(|stone| {
            stone.mutate()
                .iter()
                .for_each(|stone| new_stones.push(*stone));
        });

        self.stones = new_stones;
    }
}
