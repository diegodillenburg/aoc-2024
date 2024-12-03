use regex::Regex;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();

    let mut instructions: Vec<String> = vec![];
    process_lines(&lines, &mut instructions);
    validate_instructions(&mut instructions);
    execute_instructions(&instructions);
}

fn process_lines(lines: &[&str], instructions: &mut Vec<String>) {
    for line in lines {
        let mut chars_iter = line.chars();

        while let Some(c) = chars_iter.next() {
            if c == 'm' || c == 'd' {
                let mut chars_iter = chars_iter.clone();
                let mut instruction: Vec<char> = vec![];
                instruction.push(c);

                while let Some(lookahead) = chars_iter.next() {
                    instruction.push(lookahead);

                    if lookahead == ')' {
                        break;
                    }
                }

                let instruction = instruction.iter().collect::<String>();
                instructions.push(instruction);
            }
        }
    }
}

fn validate_instructions(instructions: &mut Vec<String>) {
    let mul_instr = Regex::new(r"^mul\(\d{1, 3},\d{1, 3}\)$").unwrap();
    let do_instr = Regex::new(r"^do\(\)$").unwrap();
    let dont_instr = Regex::new(r"^don't\(\)$").unwrap();

    instructions.retain(|instruction| mul_instr.is_match(instruction)
        || do_instr.is_match(instruction)
        || dont_instr.is_match(instruction));
}

fn execute_instructions(instructions: &Vec<String>) {
    let mul_operands = Regex::new(r"(\d{1, 3}),(\d{1, 3})").unwrap();
    let do_instr = Regex::new(r"^do\(\)$").unwrap();
    let dont_instr = Regex::new(r"^don't\(\)$").unwrap();

    let mut control: u8 = 1; // 0 => don't, 1 => do

    let mut acc: isize = 0;

    for instruction in instructions {
        if do_instr.is_match(instruction) {
            control = 1;
            continue;
        }

        if dont_instr.is_match(instruction) {
            control = 0;
            continue;
        }

        let operands = mul_operands.captures(instruction).unwrap();
        let left = operands.get(1).unwrap().as_str().parse::<isize>().unwrap();
        let right = operands.get(2).unwrap().as_str().parse::<isize>().unwrap();

        if control == 1 {
            acc += left * right;
        } else { continue; }
    }

    println!("Resulting valid mul(x,y) sum: {}\n", acc);
}
