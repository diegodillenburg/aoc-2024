#[derive(Debug)]
pub struct CalibrationEntry {
    pub expected_output: usize,
    pub operands: Vec<usize>,
}

impl CalibrationEntry {
    fn operators(max_size: usize) -> Vec<Vec<char>> {
        let mut operators: Vec<Vec<char>> = Vec::new();

        for i in 0..(1 << max_size) {
            let mut operator_list: Vec<char> = vec![];
            for j in (0..max_size).rev() {
                if (i & (1 << j)) == 0 {
                    operator_list.push('+');
                } else {
                    operator_list.push('*');
                }
            }

            operators.push(operator_list);
        }

        operators
    }

    pub fn process(&self) -> bool {
        let operators = CalibrationEntry::operators(self.operands.len() - 1);

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
        } else {
            operand_1 * operand_2
        }
    }
}
