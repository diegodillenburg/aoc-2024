use std::fs;

#[derive(Debug)]
struct Window {
    from: usize,
    to: usize,
    tendency: usize,
}

impl Window {
    fn build(from: usize, to: usize, tendency: usize) -> Self {
        Self { from, to, tendency }
    }

    fn is_offensive(&self) -> bool {
        !self.valid_gap() || !self.respects_tendency()
    }

    fn direction(&self) -> usize {
        if self.from > self.to {
            0 // descending
        } else {
            1 // ascending
        }
    }

    fn respects_tendency(&self) -> bool {
        self.tendency == self.direction()
    }

    fn valid_gap(&self) -> bool {
        let diff = self.from.abs_diff(self.to);
        diff > 0 && diff < 4
    }

}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();

    valid_reports(lines);
}

fn direction(from: usize, to: usize) -> usize {
    if from > to {
        0 // descending
    } else {
        1 // ascending
    }
}

fn valid_reports(lines: Vec<&str>) {
    let mut pure_valid_report_numbers = 0;
    let mut dampened_valid_report_numbers = 0;

    for line in &lines {
        let report_numbers: Vec<usize> = line.split_whitespace()
            .filter_map(|num| num.parse::<usize>().ok())
            .collect();

        let offenses = process_report_numbers(report_numbers);

        match offenses {
            0 => pure_valid_report_numbers += 1,
            1 => dampened_valid_report_numbers += 1,
            _ => (),
        }
    }

    let total_valid_reports = pure_valid_report_numbers + dampened_valid_report_numbers;

    println!("Purely valid reports: {}\nDampened valid reports: {}\nTotal valid reports: {}", pure_valid_report_numbers, dampened_valid_report_numbers, total_valid_reports);
}


fn process_report_numbers(report_numbers: Vec<usize>) -> usize {
    let mut offenses = 0;

    let mut previous_direction: Option<usize> = None;

    for (index, &current) in report_numbers.iter().enumerate() {
        match report_numbers.get(index + 1) {
            Some(&next) => {
                if process_window(current, next, &mut previous_direction) {
                    offenses += 1;

                    if offenses == 1 {
                        let mut report_numbers: Vec<usize> = report_numbers.clone();
                        report_numbers.remove(index);

                        if process_report_numbers(report_numbers) == 0 {
                            return 1;
                        }

                    }
                }
            },
            None => (),
        }
    }

    offenses
}

fn process_window(current: usize, next: usize, previous_direction: &mut Option<usize>) -> bool {
    let tendency = direction(current, next);
    let tendency = previous_direction.map(|val| val).unwrap_or(tendency);
    let window = Window::build(current, next, tendency);

    *previous_direction = Some(tendency);

    window.is_offensive()
}
