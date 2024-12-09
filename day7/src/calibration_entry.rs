#[derive(Debug)]
pub struct CalibrationEntry {
    pub expected_output: usize,
    pub operands: Vec<usize>,
}

impl CalibrationEntry {
    fn operators(max_size: usize, base: usize) -> Vec<Vec<char>> {
        let mut operators: Vec<Vec<char>> = Vec::new();

        for i in 0..(base.pow(max_size as u32)) {
            let mut operator_list: Vec<char> = vec![];
            let mut num = i;

            for _ in 0..max_size {
                let remainder = num % base;
                match remainder {
                    0 => operator_list.push('+'),
                    1 => operator_list.push('*'),
                    _ => operator_list.push('|'),
                }

                num /= base;
            }

            operator_list.reverse();
            operators.push(operator_list);
        }

        operators
    }

    pub fn process(&self, base: usize) -> bool {
        let operators = CalibrationEntry::operators(self.operands.len() - 1, base);

        for operator_list in operators {
            let mut operands = self.operands.clone();
            for index in 0..operands.len() - 1 {
                let result = CalibrationEntry::evaluate(
                    operands[index], operands[index + 1], operator_list[index]
                );

                operands[index + 1] = result;
            }

            if let Some(result) = operands.pop() {
                if result == self.expected_output {
                    return true;
                }
            }
        }

        false
    }

    fn evaluate(operand_1: usize, operand_2: usize, operator: char) -> usize {
        if operator == '+' {
            operand_1 + operand_2
        } else if operator == '*'{
            operand_1 * operand_2
        } else { // '|' operator
            let operand_2_len = operand_2.to_string().len();
            operand_1 * 10usize.pow(operand_2_len as u32) + operand_2
        }
    }
}
