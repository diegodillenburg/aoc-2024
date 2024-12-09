use day7::calibration::Calibration;

fn main() {
    let mut calibration = Calibration::from_file("input.txt");
    calibration.process(2);

    println!("The sum of the valid calibration outputs is [+, *]: {}", calibration.result);
    // calibration.process(3);
    // println!("The sum of the valid calibration outputs is [+, *, |]: {}", calibration.result);
}
