use crate::calibration_entry::CalibrationEntry;
use std::fs;

#[derive(Debug)]
pub struct Calibration {
    pub calibration_entries: Vec<CalibrationEntry>,
    pub max_size: usize,
    pub result: usize,
}

impl Calibration {
    pub fn from_file(path: &str) -> Calibration {
        let mut contents = fs::read_to_string(path)
            .expect("something wrong reading the file");
        contents.pop();

        let calibration_entries: Vec<&str> = contents.split("\n").collect();
        let mut max_size = 0;

        let calibration_entries: Vec<CalibrationEntry> =
            calibration_entries.iter().map(|entry| {
                let entry_parts: Vec<&str> = entry.split(":").collect();
                let expected_output = entry_parts[0].parse::<usize>().unwrap();

                let operands: Vec<usize> = entry_parts[1]
                    .trim()
                    .split(" ")
                    .map(|operand| operand.parse::<usize>().unwrap())
                    .collect();

                if operands.len() > max_size {
                    max_size = operands.len();
                }

                CalibrationEntry {
                    expected_output,
                    operands,
                }
            }).collect();

        Calibration {
            calibration_entries,
            max_size,
            result: 0,
        }
    }

    pub fn process(&mut self) {
        for calibration_entry in self.calibration_entries.iter() {
            if calibration_entry.process() {
                self.result += calibration_entry.expected_output;
            }
        }
    }
}
