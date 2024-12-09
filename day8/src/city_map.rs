use crate::antenna::{Antenna, AntennaPair, Coordinate};
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
pub struct CityMap {
    pub antennas: HashMap<char, Vec<Antenna>>,
    pub antenna_pairs: Vec<AntennaPair>,
    pub influence_zone: HashSet<Coordinate>,
    pub width: usize,
    pub height: usize,
}

impl CityMap {
    pub fn new(path: &str) -> CityMap {
        let contents = fs::read_to_string(path)
            .expect("something wrong reading file");
        let rows: Vec<&str> = contents.split("\n").collect();
        let height = rows.len() - 1;
        let width = rows[0].len();
        let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();

        for (row_index, row) in rows.iter().enumerate() {
            for (column_index, column) in row.chars().into_iter().enumerate() {
                if column != '.' {
                    let coordinate = Coordinate {
                        x: column_index + 1,
                        y: row_index + 1,
                    };

                    let frequency = column;
                    let antenna = Antenna {
                        coordinate,
                        frequency,
                    };

                    antennas
                        .entry(frequency)
                        .or_insert(Vec::new())
                        .push(antenna);
                }
            }
        }

        CityMap {
            antennas,
            antenna_pairs: Vec::new(),
            influence_zone: HashSet::new(),
            width,
            height,
        }
    }

    pub fn map_influence_zone(&mut self) {
        self.antennas.clone().iter().for_each(|(_, antennas)| {
            for i in 0..antennas.len() {
                for j in (i + 1)..antennas.len() {
                    let antenna_a = &antennas[i];
                    let antenna_b = &antennas[j];

                    let antenna_pair = AntennaPair::new(
                        antenna_a.clone(),
                        antenna_b.clone(),
                        self.width,
                        self.height,
                    );

                    antenna_pair.antinodes.iter().for_each(|antinode| {
                        self.influence_zone.insert(antinode.coordinate.clone());
                    });

                    self.antenna_pairs.push(antenna_pair);
                }
            }
        });
    }

    pub fn antenna_frequencies(&self) -> Vec<char> {
        self.antennas.keys().cloned().collect()
    }

    pub fn frequency_antennas(&self, frequency: char) -> Option<&Vec<Antenna>> {
        self.antennas.get(&frequency)
    }

    pub fn frequency_coordinates(&self, frequency: char) -> Vec<Coordinate> {
        let mut antennas = self.frequency_antennas(frequency)
            .map(|antennas| antennas.iter().map(|antenna| antenna.coordinate).collect())
            .unwrap_or(Vec::new());
        antennas.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

        antennas
    }

pub fn draw(&self) {
    print!("   ");
    for x in 1..=self.width {
        print!("{:2} ", x);
    }
    print!("\n");

    for y in 1..=self.height {
        print!("{:2}  ", y);

        for x in 1..=self.width {
            let coordinate = Coordinate { x, y };
            let mut antenna_found = false;
            let mut char_to_print = '.';

            for antennas in self.antennas.values() {
                for antenna in antennas {
                    if antenna.coordinate == coordinate {
                        char_to_print = antenna.frequency;
                        antenna_found = true;
                        break;
                    }
                }
                if antenna_found {
                    break;
                }
            }

            if !antenna_found && self.influence_zone.contains(&coordinate) {
                char_to_print = '#';
            }

            print!("{:2} ", char_to_print);
        }

        print!("\n");
    }
}
}
