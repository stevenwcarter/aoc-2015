use std::ops::RangeInclusive;

use advent_of_code::SmallPoint;
use hashbrown::HashMap;

advent_of_code::solution!(6);

// TODO: see if its faster to go through each instruction considering individual grid points
// (will use less memory, and don't need a container structure at all)

pub struct LightGrid {
    pub grid: HashMap<SmallPoint, bool>,
}
pub struct LightGrid2 {
    pub grid: HashMap<SmallPoint, u8>,
}

impl Default for LightGrid {
    fn default() -> Self {
        let mut grid = HashMap::new();

        for y in 0..1000 {
            for x in 0..1000 {
                let p = SmallPoint::from((x as u16, y));

                grid.insert(p, false);
            }
        }

        Self { grid }
    }
}
impl Default for LightGrid2 {
    fn default() -> Self {
        let mut grid = HashMap::new();

        for y in 0..1000 {
            for x in 0..1000 {
                let p = SmallPoint::from((x as u16, y));

                grid.insert(p, 0);
            }
        }

        Self { grid }
    }
}

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

impl LightGrid {
    pub fn process_instruction(&mut self, instruction: &Instruction) {
        for y in instruction.y_range.clone() {
            for x in instruction.x_range.clone() {
                let pt = SmallPoint::from((x, y));
                match instruction.instruction_type {
                    ON => *self.grid.get_mut(&pt).unwrap() = true,
                    OFF => *self.grid.get_mut(&pt).unwrap() = false,
                    TOGGLE => {
                        let pt = SmallPoint::from((x, y));
                        let grid_pt = self.grid.get_mut(&pt).unwrap();
                        *grid_pt = !*grid_pt;
                    }
                }
            }
        }
    }

    pub fn illuminated_count(&self) -> usize {
        self.grid.iter().filter(|(_, &s)| s).count()
    }
}
impl LightGrid2 {
    pub fn process_instruction(&mut self, instruction: &Instruction) {
        for y in instruction.y_range.clone() {
            for x in instruction.x_range.clone() {
                let pt = SmallPoint::from((x, y));
                match instruction.instruction_type {
                    ON => *self.grid.get_mut(&pt).unwrap() += 1,
                    OFF => {
                        let grid_pt = self.grid.get_mut(&pt).unwrap();
                        *grid_pt = grid_pt.saturating_sub(1);
                    }
                    TOGGLE => *self.grid.get_mut(&pt).unwrap() += 2,
                }
            }
        }
    }

    pub fn total_illumination(&self) -> u32 {
        self.grid.iter().map(|(_, &s)| s as u32).sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lights = LightGrid::default();
    input
        .lines()
        .map(Instruction::parse)
        .for_each(|i| lights.process_instruction(&i));

    Some(lights.illuminated_count())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lights = LightGrid2::default();
    input
        .lines()
        .map(Instruction::parse)
        .for_each(|i| lights.process_instruction(&i));

    Some(lights.total_illumination())
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
