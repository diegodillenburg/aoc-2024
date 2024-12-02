use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();

    valid_reports(lines);
}

fn valid_reports(lines: Vec<&str>) {
    let mut pure_valid_report_numbers = 0;
    let mut dampened_valid_report_numbers = 0;

    for line in &lines {
        let report_numbers: Vec<usize> = line.split_whitespace()
            .filter_map(|num| num.parse::<usize>().ok())
            .collect();

        let mut offenses = 0;

        let mut previous_direction: Option<usize> = None;
        let mut direction;

        for window in report_numbers.windows(2) {
            let current = window[0];
            let next = window[1];

            let mut window_offenses = 0;

            if current.abs_diff(next) < 1  || current.abs_diff(next) > 3 {
                window_offenses += 1;
            }

            if current > next {
                direction = 0;
            } else {
                direction = 1
            }

            if let Some(prev) = previous_direction {
                if prev != direction {
                    window_offenses += 1;
                }
            }

            if window_offenses != 0 {
                offenses += 1;
            }

            previous_direction = Some(direction);
        }


        match offenses {
            0 => pure_valid_report_numbers += 1,
            1 => dampened_valid_report_numbers += 1,
            _ => (),
        }
    }

    let total_valid_reports = pure_valid_report_numbers + dampened_valid_report_numbers;

    println!("Purely valid reports: {}\nDampened valid reports: {}\nTotal valid reports: {}", pure_valid_report_numbers, dampened_valid_report_numbers, total_valid_reports);
}
