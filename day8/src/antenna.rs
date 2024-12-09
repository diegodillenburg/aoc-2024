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

        Self::generate_antinodes(&mut antinodes, &antenna_a, &antenna_b, max_width, max_height);

        Self::generate_antinodes(&mut antinodes, &antenna_b, &antenna_a, max_width, max_height);

        AntennaPair {
            antenna_a,
            antenna_b,
            antinodes,
        }
    }

    fn generate_antinodes(
        antinodes: &mut Vec<Antinode>,
        start_antenna: &Antenna,
        other_antenna: &Antenna,
        max_width: usize,
        max_height: usize,
    ) {
        let distance = Coordinate::calculate_distance(&start_antenna.coordinate, &other_antenna.coordinate);
        let mut current_x = start_antenna.coordinate.x as isize;
        let mut current_y = start_antenna.coordinate.y as isize;

        while current_x > 0 && current_y > 0 && (current_x as usize) <= max_width && (current_y as usize) <= max_height {
            let antinode_x = current_x as usize;
            let antinode_y = current_y as usize;
            antinodes.push(Antinode {
                coordinate: Coordinate { x: antinode_x, y: antinode_y },
                distance,
            });
            current_x += distance.0;
            current_y += distance.1;
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
