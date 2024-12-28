advent_of_code::solution!(18);

use rayon::prelude::*;

pub struct LightGrid {
    grid: Vec<bool>,
    width: isize,
    height: isize,
}

const DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1), // Top row
    (-1, 0),
    (1, 0), // Middle row
    (-1, 1),
    (0, 1),
    (1, 1), // Bottom row
];

impl LightGrid {
    pub fn parse_input(input: &str) -> Self {
        let height = input.lines().collect::<Vec<_>>().len() as isize;
        let width = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<_>>()
            .len() as isize;
        let grid: Vec<bool> = input
            .lines()
            .flat_map(|l| l.chars().map(|c| matches!(c, '#')).collect::<Vec<bool>>())
            .collect();

        Self {
            grid,
            width,
            height,
        }
    }

    pub fn neighbors(&self, pos: usize) -> Vec<usize> {
        let mut result = Vec::new();

        let x = (pos as isize) % self.width;
        let y = (pos as isize) / self.width;

        for &(dx, dy) in &DELTAS {
            let nx = x + dx;
            let ny = y + dy;

            if nx >= 0 && nx < self.width && ny >= 0 && ny < self.height {
                result.push((ny * self.width + nx) as usize);
            }
        }

        result
    }

    pub fn neighbor_illuminated_count(&self, pos: usize) -> u8 {
        self.neighbors(pos)
            .iter()
            .filter(|&&p| self.grid[p])
            .count() as u8
    }

    pub fn step(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.grid = self
                .grid
                .par_iter()
                .enumerate()
                .map(|(pos, &value)| {
                    if value {
                        (2..4).contains(&self.neighbor_illuminated_count(pos))
                    } else {
                        self.neighbor_illuminated_count(pos) == 3
                    }
                })
                .collect()
        }
    }
    pub fn step2(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.grid = self
                .grid
                .par_iter()
                .enumerate()
                .map(|(pos, &value)| {
                    if pos == 0
                        || pos as isize == self.width - 1
                        || pos as isize == self.width * self.height - 1
                        || pos as isize == self.width * (self.height - 1)
                    {
                        return true;
                    }
                    if value {
                        (2..4).contains(&self.neighbor_illuminated_count(pos))
                    } else {
                        self.neighbor_illuminated_count(pos) == 3
                    }
                })
                .collect()
        }
    }

    pub fn count_lights(&self) -> usize {
        self.grid.iter().filter(|&&l| l).count()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lightgrid = LightGrid::parse_input(input);

    lightgrid.step(100);

    Some(lightgrid.count_lights())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lightgrid = LightGrid::parse_input(input);

    lightgrid.step2(100);

    Some(lightgrid.count_lights())
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
