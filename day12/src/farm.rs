use crate::coordinate::{Coordinate, ICoordinate};
use crate::plot::Plot;
use crate::tile::Tile;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
pub struct Farm {
    pub plots: Vec<Plot>,
    pub tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
    pub outer_coordinates_map: HashMap<Coordinate, Vec<ICoordinate>>,
}

impl Farm {
    pub fn new(path: &str) -> Farm {
        let contents = fs::read_to_string(path).expect("something wrong reading file");
        let rows: Vec<&str> = contents.split_terminator("\n").collect();
        let height = rows.len();
        let width = rows[0].len();
        let outer_coordinates_map: HashMap<Coordinate, Vec<ICoordinate>> = HashMap::new();

        let plots: Vec<Plot> = Vec::new();
        let mut tiles: Vec<Tile> = Vec::new();

        for (i, row) in rows.iter().enumerate() {
            for (j, char) in row.chars().enumerate() {
                tiles.push(Tile {
                    coordinate: Coordinate { x: j, y: i },
                    kind: char,
                });
            }
        }

        Farm {
            plots,
            tiles,
            width,
            height,
            outer_coordinates_map,
        }
    }

    pub fn fence_price(&self, by_edge: bool) -> usize {
        let mut price = 0;
        if by_edge {
            for plot in &self.plots {
                price += plot.edges * plot.area();
            }
        } else {
            for plot in &self.plots {
                price += plot.safe_perimeter() * plot.area();
            }
        }

        price
    }

    pub fn populate(&mut self) {
        for tile in &self.tiles {
            if self
                .plots
                .iter()
                .any(|plot| plot.coordinates.contains(&tile.coordinate))
            {
                continue;
            }

            let mut plot = Plot {
                coordinates: HashSet::new(),
                perimeter: 0,
                kind: tile.kind,
                edges: 0,
            };

            self.traverse(&mut plot, tile.kind, tile.coordinate.x, tile.coordinate.y);

            if plot.coordinates.len() > 0 {
                if !self
                    .plots
                    .iter()
                    .any(|p| p.coordinates.is_superset(&plot.coordinates))
                {
                    self.plots.push(plot.clone());
                }
            };
        }
    }

    fn traverse(&self, plot: &mut Plot, char: char, x: usize, y: usize) {
        let coordinate = Coordinate { x, y };

        if plot.coordinates.contains(&coordinate) {
            return;
        }

        if let Some(valid_movements) = self.valid_movements(&coordinate, char) {
            if plot.coordinates.insert(coordinate.clone()) {
                plot.perimeter += 4 - valid_movements.len();
                for movement in valid_movements {
                    if movement == coordinate {
                        continue;
                    }
                    self.traverse(plot, char, movement.x, movement.y);
                }
            }
        } else {
            if plot.coordinates.insert(coordinate.clone()) {
                plot.perimeter += 3;
            }
        }
    }

    fn outer_coordinates(&self, coordinate: &Coordinate, char: char) -> Option<Vec<ICoordinate>> {
        let movements = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
        let mut outer_coordinates: Vec<ICoordinate> = Vec::new();

        for (y, x) in movements {
            let x = coordinate.x as isize + x;
            let y = coordinate.y as isize + y;
            if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
                let x = x as usize;
                let y = y as usize;
                let coordinate = Coordinate { x, y };
                let kind = char;
                let expected_tile = Tile {
                    coordinate: coordinate.clone(),
                    kind,
                };
                if !self.tiles.contains(&expected_tile) {
                    let coordinate = ICoordinate { x: x as isize, y: y as isize };
                    outer_coordinates.push(coordinate);
                }
            } else {
                outer_coordinates.push(ICoordinate { x, y });
            }
        }

        if outer_coordinates.len() > 0 {
            Some(outer_coordinates)
        } else {
            None
        }
    }

    fn valid_movements(&self, coordinate: &Coordinate, char: char) -> Option<Vec<Coordinate>> {
        let movements = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
        let mut valid_movements: Vec<Coordinate> = Vec::new();

        for (y, x) in movements {
            let x = coordinate.x as isize + x;
            let y = coordinate.y as isize + y;
            if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
                let x = x as usize;
                let y = y as usize;
                let coordinate = Coordinate { x, y };
                let kind = char;
                let expected_tile = Tile {
                    coordinate: coordinate.clone(),
                    kind,
                };
                if self.tiles.contains(&expected_tile) {
                    valid_movements.push(coordinate);
                }
            }
        }

        if valid_movements.len() > 0 {
            Some(valid_movements)
        } else {
            None
        }
    }
}
