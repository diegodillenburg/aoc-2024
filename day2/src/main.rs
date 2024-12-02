use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();

    let mut invalid_report_numbers = 0;

    for line in &lines {
        let report_numbers: Vec<usize> = line.split_whitespace()
            .filter_map(|num| num.parse::<usize>().ok())
            .collect();

        let mut ascending = 0;
        let mut descending = 0;

        for window in report_numbers.windows(2) {
            let current = window[0];
            let next = window[1];

            if current.abs_diff(next) > 3 {
                invalid_report_numbers += 1;
                break;
            }

            if current.abs_diff(next) < 1 {
                invalid_report_numbers += 1;
                break;
            }

            if current > next {
                ascending = 1;
            } else {
                descending = 1;
            }

            if ascending == 1 && descending == 1 {
                invalid_report_numbers += 1;
                break;
            }
        }
    }

    let valid_report_numbers = lines.len() - invalid_report_numbers;

    println!("{}", valid_report_numbers);
}
