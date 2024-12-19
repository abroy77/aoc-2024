use std::{env, fmt::Debug, fs::read_to_string, path::PathBuf, str::FromStr};

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{newline, u64},
    multi::separated_list1,
    IResult,
};

fn main() -> std::io::Result<()> {
    // get the data filepath
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Filepath not provided");
    }
    let data_path = PathBuf::from_str(&args[1]).expect("Failed to convert input to filepath");

    assert!(data_path.exists(), "data path does not exist");
    let data = read_to_string(data_path).expect("could not read datapath");
    let (_, mut comp) = parse_input(&data).unwrap();
    let result = comp.result();
    println!("Solution is {}", result);

    Ok(())
}

struct Computer {
    registers: [usize; 3],
    program: Vec<usize>,
    instruction_pointer: usize,
    output: Vec<usize>,
}

impl Debug for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Reg A: {}\nReg B: {}\nReg C: {}\nProgram: {:?}\nPointer: {}\nOutput: {:?}",
            self.registers[0],
            self.registers[1],
            self.registers[2],
            self.program,
            self.instruction_pointer,
            self.output,
        )
    }
}

impl Computer {
    fn new(program: Vec<usize>, reg_a: usize, reg_b: usize, reg_c: usize) -> Self {
        Computer {
            registers: [reg_a, reg_b, reg_c],
            program,
            instruction_pointer: 0,
            output: Vec::new(),
        }
    }

    fn run(&mut self) {
        while self.instruction_pointer < self.program.len() - 1 {
            let opcode = self.program[self.instruction_pointer];
            let operand_code = self.program[self.instruction_pointer + 1];

            match opcode {
                0 => self.adv(operand_code),
                1 => self.bxl(operand_code),
                2 => self.bst(operand_code),
                3 => self.jnz(operand_code),
                4 => self.bxc(operand_code),
                5 => self.out(operand_code),
                6 => self.bdv(operand_code),
                7 => self.cdv(operand_code),
                _ => panic!("unknown opcode read: {}", opcode),
            };

            if opcode != 3 || self.registers[0] == 0 {
                self.instruction_pointer += 2;
            }
        }
    }

    fn get_combo_operand(&self, operand_code: usize) -> usize {
        match operand_code {
            0..=3 => operand_code,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => panic!("Unsupported operand code"),
        }
    }

    fn adv(&mut self, operand_code: usize) {
        self.registers[0] >>= self.get_combo_operand(operand_code);
        // self.registers[0] /= 2_usize.pow(self.get_combo_operand(operand_code) as u32);
    }

    fn bxl(&mut self, operand_code: usize) {
        self.registers[1] ^= operand_code;
    }
    fn bst(&mut self, operand_code: usize) {
        self.registers[1] = self.get_combo_operand(operand_code) % 8;
    }

    fn jnz(&mut self, operand_code: usize) {
        if self.registers[0] != 0 {
            self.instruction_pointer = operand_code;
        }
    }

    fn bxc(&mut self, _operand_code: usize) {
        self.registers[1] ^= self.registers[2];
    }
    fn out(&mut self, operand_code: usize) {
        self.output.push(self.get_combo_operand(operand_code) % 8);
    }
    fn bdv(&mut self, operand_code: usize) {
        self.registers[1] = self.registers[0] >> self.get_combo_operand(operand_code);
    }
    fn cdv(&mut self, operand_code: usize) {
        self.registers[2] = self.registers[0] >> self.get_combo_operand(operand_code);
    }
    fn result(&mut self) -> String {
        self.run();
        // now get the output a a comma separated string
        self.output
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

fn parse_register(input: &str) -> IResult<&str, usize> {
    let (input, _) = take_till(|c: char| c.is_numeric())(input)?;
    let (input, num) = u64(input)?;
    Ok((input, num as usize))
}
fn parse_program(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = take_till(|c: char| c.is_numeric())(input)?;
    let (input, program) = separated_list1(tag(","), u64)(input)?;
    let program = program.into_iter().map(|p| p as usize).collect();

    Ok((input, program))
}

fn parse_input(input: &str) -> IResult<&str, Computer> {
    let (input, reg_1) = parse_register(input)?;
    let (input, _) = newline(input)?;
    let (input, reg_2) = parse_register(input)?;
    let (input, _) = newline(input)?;
    let (input, reg_3) = parse_register(input)?;
    let (input, _) = newline(input)?;

    let (input, _) = newline(input)?;

    let (input, program) = parse_program(input)?;
    Ok((input, Computer::new(program, reg_1, reg_2, reg_3)))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> String {
        r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
            .into()
    }

    #[test]
    fn test_parsing() {
        let input = input();
        let (_, comp) = parse_input(&input).unwrap();
        assert_eq!(comp.registers, [729, 0, 0]);
        assert_eq!(comp.instruction_pointer, 0);
        assert_eq!(comp.program, vec![0, 1, 5, 4, 3, 0]);
        assert_eq!(comp.output, vec![]);
    }

    #[test]
    fn test_processing() {
        let (_, mut comp) = parse_input(&input()).unwrap();
        assert_eq!(comp.result(), "4,6,3,5,6,3,5,2,1,0");
    }
}
