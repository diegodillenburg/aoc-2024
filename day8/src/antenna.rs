use std::fmt;

#[derive(Debug, PartialEq)]
pub struct AntennaPair {
    pub antenna_a: Antenna,
    pub antenna_b: Antenna,
    pub antinodes: Vec<Antinode>,
}

impl AntennaPair {
    pub fn new(antenna_a: Antenna, antenna_b: Antenna, max_width: usize, max_height: usize) -> AntennaPair {
        let mut antinodes: Vec<Antinode> = Vec::new();
        let distance = Coordinate::calculate_distance(&antenna_a.coordinate, &antenna_b.coordinate);
        let antinode_x = antenna_a.coordinate.x as isize + distance.0;
        let antinode_y = antenna_a.coordinate.y as isize + distance.1;
        if antinode_x > 0 && antinode_y > 0 {
            let antinode_x = antinode_x as usize;
            let antinode_y = antinode_y as usize;
            if antinode_x <= max_width && antinode_y <= max_height {
                let antinode = Antinode {
                    coordinate: Coordinate {
                        x: antinode_x,
                        y: antinode_y,
                    },
                    distance,
                };
                antinodes.push(antinode);
            }
        }
        let distance = Coordinate::calculate_distance(&antenna_b.coordinate, &antenna_a.coordinate);
        let antinode_x = antenna_b.coordinate.x as isize + distance.0;
        let antinode_y = antenna_b.coordinate.y as isize + distance.1;
        if antinode_x > 0 && antinode_y > 0 {
            let antinode_x = antinode_x as usize;
            let antinode_y = antinode_y as usize;
            if antinode_x <= max_width && antinode_y <= max_height {
                let antinode = Antinode {
                    coordinate: Coordinate {
                        x: antinode_x,
                        y: antinode_y,
                    },
                    distance,
                };
                antinodes.push(antinode);
            }
        }

        AntennaPair {
            antenna_a,
            antenna_b,
            antinodes,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Antenna {
    pub coordinate: Coordinate,
    pub frequency: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Antinode {
    pub coordinate: Coordinate,
    pub distance: (isize, isize)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coordinate {
    pub fn calculate_distance(coordinate_a: &Coordinate, coordinate_b: &Coordinate) -> (isize, isize) {
        let x_distance = coordinate_a.x as isize - coordinate_b.x as isize;
        let y_distance = coordinate_a.y as isize - coordinate_b.y as isize;
        (x_distance, y_distance)
    }
}
