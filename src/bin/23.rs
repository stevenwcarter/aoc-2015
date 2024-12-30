advent_of_code::solution!(23);

pub enum InstructionType {
    Triple,
    Increment,
    JumpIfOne,
    JumpIfEven,
    Jump,
    Half,
}
use InstructionType::*;

pub struct Instruction<'a> {
    pub arg_a: &'a str,
    pub arg_b: Option<&'a str>,
    pub instr_type: InstructionType,
}

pub struct Solver<'a> {
    pub a: u64,
    pub b: u64,
    pub pos: i16,
    pub instructions: Vec<Instruction<'a>>,
}

impl<'a> Solver<'a> {
    pub fn parse_input(input: &'a str) -> Self {
        let instructions: Vec<Instruction> = input
            .lines()
            .map(|l| {
                let (instruction_type, remainder) = l.split_once(' ').unwrap();
                let instruction_type = match instruction_type {
                    "hlf" => Half,
                    "tpl" => Triple,
                    "jmp" => Jump,
                    "jio" => JumpIfOne,
                    "jie" => JumpIfEven,
                    "inc" => Increment,
                    _ => unreachable!("Invalid insruction {instruction_type}"),
                };
                let (arg_a, arg_b) = match instruction_type {
                    JumpIfEven | JumpIfOne => {
                        let (arg_a, arg_b) = remainder.split_once(", ").unwrap();
                        (arg_a, Some(arg_b))
                    }
                    _ => (remainder, None),
                };

                Instruction {
                    arg_a,
                    arg_b,
                    instr_type: instruction_type,
                }
            })
            .collect();

        Self {
            a: 0,
            b: 0,
            pos: 0,
            instructions,
        }
    }

    pub fn solve(&mut self) -> Option<u64> {
        while self.instructions.get(self.pos as usize).is_some() {
            let instruction = &self.instructions[self.pos as usize];
            match instruction.instr_type {
                Triple => match instruction.arg_a {
                    "a" => self.a *= 3,
                    "b" => self.b *= 3,
                    _ => unreachable!("Unknown register {}", instruction.arg_a),
                },
                Increment => match instruction.arg_a {
                    "a" => self.a += 1,
                    "b" => self.b += 1,
                    _ => unreachable!("Unknown register {}", instruction.arg_a),
                },
                JumpIfOne => {
                    let skip = match instruction.arg_a {
                        "a" => self.a != 1,
                        "b" => self.b != 1,
                        _ => unreachable!("Unknown register {}", instruction.arg_a),
                    };
                    if skip {
                        self.pos += 1;
                        continue;
                    }

                    self.pos += parse_digit_with_sign(instruction.arg_b.unwrap());
                }
                JumpIfEven => {
                    let skip = match instruction.arg_a {
                        "a" => self.a % 2 != 0,
                        "b" => self.b % 2 != 0,
                        _ => unreachable!("Unknown register {}", instruction.arg_a),
                    };
                    if skip {
                        self.pos += 1;
                        continue;
                    }

                    self.pos += parse_digit_with_sign(instruction.arg_b.unwrap());
                }
                Jump => {
                    self.pos += parse_digit_with_sign(instruction.arg_a);
                }
                Half => match instruction.arg_a {
                    "a" => self.a /= 2,
                    "b" => self.b /= 2,
                    _ => unreachable!("Unknown register {}", instruction.arg_a),
                },
            }

            match instruction.instr_type {
                Triple | Increment | Half => self.pos += 1,
                _ => {}
            }
        }

        Some(self.b)
    }
}

pub fn parse_digit_with_sign(input: &str) -> i16 {
    let is_negative = &input[0..1] == "-";
    let value = &input[1..].parse::<u8>().unwrap();

    match is_negative {
        true => -(*value as i16),
        false => *value as i16,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut solver = Solver::parse_input(input);

    solver.solve()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut solver = Solver::parse_input(input);
    solver.a = 1;

    solver.solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
