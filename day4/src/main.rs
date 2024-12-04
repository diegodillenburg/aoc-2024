use std::fs;

#[derive(Debug)]
enum Direction {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
    East,
    West,
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let mut x_mas_count = 0;
    let mut xmas_count = 0;

    let enumerated_lines = lines.iter().enumerate();
    for (row_index, row) in enumerated_lines {
        let enumerated_columns = row.iter().enumerate();

        for (column_index, column) in enumerated_columns {
            if column == &'A' {
                let diagonal_1: String = [
                    get_diagonal(&lines, row_index, column_index, -1, 1),
                    get_diagonal(&lines, row_index, column_index, 1, -1),
                ].iter().filter_map(|&c| c).collect();

                let diagonal_2: String = [
                    get_diagonal(&lines, row_index, column_index, -1, -1),
                    get_diagonal(&lines, row_index, column_index, 1, 1),
                ].iter().filter_map(|&c| c).collect();

                if (diagonal_1 == "MS" || diagonal_1 == "SM") && (diagonal_2 == "MS" || diagonal_2 == "SM") {
                    x_mas_count += 1;
                }

            }

            if column == &'X'{
                for direction in directions_vec() {
                    let mut row_index: isize = row_index as isize;
                    let mut column_index: isize = column_index as isize;
                    let (destination_row, destination_column) = destination_coordinates(row_index, column_index, direction);
                    let traverse_row_head = destination_row;
                    let traverse_column_head = destination_column;

                    let mut word_acc: Vec<char> = vec![*column];

                    if traverse_row_head < 0 || traverse_column_head < 0 {
                        continue;
                    }

                    if traverse_row_head >= lines.len() as isize || traverse_column_head >= row.len() as isize {
                        continue;
                    }

                    while !(row_index == destination_row && column_index == destination_column) {
                        if row_index < destination_row {
                            row_index += 1;
                        } else if row_index > destination_row {
                            row_index -= 1;
                        }

                        if column_index < destination_column {
                            column_index += 1;
                        } else if column_index > destination_column {
                            column_index -= 1;
                        }

                        if let Some(row) = lines.get(row_index as usize) {
                            if let Some(&column) = row.get(column_index as usize) {
                                word_acc.push(column);
                            }
                        }
                    }

                    let word: String = word_acc.iter().collect();

                    if word == "XMAS" {
                        xmas_count += 1;
                    }

                }

            }
        }
    }

    println!("Total {} XMAS", xmas_count);
    println!("Total {} X-MAS", x_mas_count);
}

fn destination_coordinates(row: isize, column: isize, direction: Direction) -> (isize, isize) {
    match direction {
        Direction::North => (row + 3, column),
        Direction::NorthEast => (row + 3, column + 3),
        Direction::NorthWest => (row - 3, column - 3),
        Direction::South => (row - 3, column),
        Direction::SouthEast => (row - 3, column + 3),
        Direction::SouthWest => (row + 3, column - 3),
        Direction::East => (row, column + 3),
        Direction::West => (row, column - 3),
    }
}

fn directions_vec() -> Vec<Direction> {
    vec![
        Direction::North,
        Direction::NorthEast,
        Direction::NorthWest,
        Direction::South,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::East,
        Direction::West,
    ]
}

fn get_diagonal(lines: &[Vec<char>], row_index: usize, column_index: usize,
    row_offset: isize, col_offset: isize) -> Option<char> {
    let new_row_index = (row_index as isize + row_offset) as usize;
    let new_col_index = (column_index as isize + col_offset) as usize;

    lines.get(new_row_index).and_then(|row| row.get(new_col_index).copied())
}
