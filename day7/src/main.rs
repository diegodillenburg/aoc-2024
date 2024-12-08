use day7::calibration::Calibration;

fn main() {
    let mut calibration = Calibration::from_file("input_mini.txt");
    calibration.process();

    println!("The sum of the valid calibration outputs is: {}", calibration.result);
}
