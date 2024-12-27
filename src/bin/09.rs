use rayon::prelude::*;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use hashbrown::HashSet;

advent_of_code::solution!(9);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dest {
    pub src: u64,
    pub dst: u64,
    pub dist: u16,
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

impl Dest {
    pub fn new(l: &str) -> Self {
        let (dests, distance) = l.split_once(" = ").unwrap();
        let (source, dest) = dests.split_once(" to ").unwrap();

        Self {
            src: calculate_hash(&source),
            dst: calculate_hash(&dest),
            dist: distance.parse::<u16>().unwrap(),
        }
    }
}

pub struct Solver {
    pub dests: Vec<Dest>,
    pub seen: HashSet<Dest>,
}

pub fn shortest(
    distance: u32,
    unique_dests: HashSet<u64>,
    src: u64,
    dests: &[Dest],
) -> Option<u32> {
    let mut unique_dests = unique_dests.clone();
    unique_dests.remove(&src);

    if unique_dests.is_empty() {
        return Some(distance);
    }

    dests
        .iter()
        .filter(|d| d.src == src || d.dst == src)
        .filter(|d| {
            if d.src.eq(&src) {
                unique_dests.contains(&d.dst)
            } else {
                unique_dests.contains(&d.src)
            }
        })
        .filter_map(|d| {
            let new_src = if d.src.eq(&src) { d.dst } else { d.src };
            shortest(
                distance + d.dist as u32,
                unique_dests.clone(),
                new_src,
                dests,
            )
        })
        .min()
}
pub fn longest(distance: u32, unique_dests: HashSet<u64>, src: u64, dests: &[Dest]) -> Option<u32> {
    let mut unique_dests = unique_dests.clone();
    unique_dests.remove(&src);

    if unique_dests.is_empty() {
        return Some(distance);
    }

    dests
        .iter()
        .filter(|d| d.src.eq(&src) || d.dst.eq(&src))
        .filter(|d| {
            if d.src.eq(&src) {
                unique_dests.contains(&d.dst)
            } else {
                unique_dests.contains(&d.src)
            }
        })
        .filter_map(|d| {
            let new_src = if d.src.eq(&src) { d.dst } else { d.src };
            longest(
                distance + d.dist as u32,
                unique_dests.clone(),
                new_src,
                dests,
            )
        })
        .max()
}

impl Solver {
    pub fn parse_input(input: &str) -> Self {
        let dests: Vec<Dest> = input.lines().map(Dest::new).collect();
        Self {
            dests,
            seen: HashSet::new(),
        }
    }

    pub fn shortest(&self) -> Option<u32> {
        let mut unique_dests: HashSet<u64> = self.dests.iter().map(|d| d.dst).collect();
        self.dests.iter().for_each(|d| {
            unique_dests.insert(d.src);
        });
        unique_dests
            .par_iter()
            .filter_map(|d| {
                let mut unique_dests = unique_dests.clone();
                unique_dests.remove(d);
                shortest(0, unique_dests, *d, &self.dests)
            })
            .min()
    }
    pub fn longest(&self) -> Option<u32> {
        let mut unique_dests: HashSet<u64> = self.dests.iter().map(|d| d.dst).collect();
        self.dests.iter().for_each(|d| {
            unique_dests.insert(d.src);
        });
        unique_dests
            .par_iter()
            .filter_map(|d| {
                let mut unique_dests = unique_dests.clone();
                unique_dests.remove(d);
                longest(0, unique_dests, *d, &self.dests)
            })
            .max()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let solver = Solver::parse_input(input);

    solver.shortest()
}

pub fn part_two(input: &str) -> Option<u32> {
    let solver = Solver::parse_input(input);

    solver.longest()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(982));
    }
}
