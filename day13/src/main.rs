use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let equations: Vec<Vec<Vec<isize>>> = contents.split("\n\n")
        .map(|equation| {
            equation.split_terminator("\n")
                .map(|tuple| {
                    tuple.split(", ")
                        .map(|digit| {
                            digit.parse::<isize>().unwrap()
                        }).collect::<Vec<isize>>()
                }).collect::<Vec<Vec<isize>>>()
        }).collect::<Vec<Vec<Vec<isize>>>>();

    let total_cost_without_offset = calculate_total_cost(&equations, false);
    let total_cost_with_offset = calculate_total_cost(&equations, true);

    println!("Total cost without offset: {}", total_cost_without_offset);
    println!("Total cost with offset: {}", total_cost_with_offset);
}

fn calculate_total_cost(equations: &Vec<Vec<Vec<isize>>>, add_offset: bool) -> isize {
    let mut total_cost = 0;
    let offset = if add_offset { 10_000_000_000_000 } else { 0 };

    for equation in equations {
        let a = &equation[0];
        let b = &equation[1];
        let mut p = equation[2].clone();

        // Apply the offset if needed
        p[0] += offset;
        p[1] += offset;

        let result_a = (p[0] * b[1] - p[1] * b[0]) / (a[0] * b[1] - a[1] * b[0]);
        let result_b = (p[1] * a[0] - p[0] * a[1]) / (a[0] * b[1] - a[1] * b[0]);

        if (a[0] * result_a) + (b[0] * result_b) == p[0] && (a[1] * result_a) + (b[1] * result_b) == p[1] {
            total_cost += 3 * result_a.abs() + result_b.abs();
        }
    }

    total_cost
}
