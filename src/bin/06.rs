use std::ops::RangeInclusive;

use rayon::prelude::*;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionType {
    ON,
    OFF,
    TOGGLE,
}
use InstructionType::*;

pub struct Instruction {
    pub instruction_type: InstructionType,
    pub x_range: RangeInclusive<u16>,
    pub y_range: RangeInclusive<u16>,
}

impl Instruction {
    pub fn parse(input: &str) -> Self {
        let (instruction_type, remaining) = match &input[0..7] {
            "turn on" => (ON, &input[8..]),
            "turn of" => (OFF, &input[9..]),
            "toggle " => (TOGGLE, &input[7..]),
            _ => unreachable!("Invalid instruction {input}"),
        };

        let (range_start, remainder) = remaining.split_once(' ').unwrap();
        let (x_start, y_start) = range_start.split_once(',').unwrap();
        let (x_end, y_end) = &remainder[8..].split_once(',').unwrap();

        let x_start = x_start.parse::<u16>().unwrap();
        let y_start = y_start.parse::<u16>().unwrap();
        let x_end = x_end.parse::<u16>().unwrap();
        let y_end = y_end.parse::<u16>().unwrap();

        // println!("\n{input}\n{x_start}->{x_end} {y_start}->{y_end}");

        Self {
            instruction_type,
            x_range: (x_start..=x_end),
            y_range: (y_start..=y_end),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let data: Vec<Instruction> = input.lines().map(Instruction::parse).collect();

    Some(
        (0..1000)
            .par_bridge()
            .map(|y| {
                (0..1000)
                    .filter(|&x| {
                        data.iter()
                            .filter(|inst| {
                                inst.x_range.contains(&x) && inst.y_range.contains(&(y as u16))
                            })
                            .fold(false, |state, inst| match inst.instruction_type {
                                ON => true,
                                OFF => false,
                                TOGGLE => !state,
                            })
                    })
                    .count()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let data: Vec<Instruction> = input.lines().map(Instruction::parse).collect();

    Some(
        (0..1000)
            .par_bridge()
            .map(|y| {
                (0..1000)
                    .map(|x| {
                        data.iter()
                            .filter(|inst| {
                                inst.x_range.contains(&x) && inst.y_range.contains(&(y as u16))
                            })
                            .fold(0u32, |state, inst| match inst.instruction_type {
                                ON => state + 1,
                                OFF => state.saturating_sub(1),
                                TOGGLE => state + 2,
                            })
                    })
                    .sum::<u32>()
            })
            .sum::<u32>(),
    )
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {
//         let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }
//
//     #[test]
//     fn test_part_two() {
//         let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }
// }
