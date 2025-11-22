use dashmap::DashSet;
use hashbrown::HashSet;
use pathfinding::prelude::astar;

advent_of_code::solution!(19);

pub struct Replacer<'a> {
    rules: Vec<(&'a str, &'a str)>,
    molecule: &'a str,
}

impl<'a> Replacer<'a> {
    pub fn parse_input(input: &'a str) -> Self {
        let mut rules: Vec<(&str, &str)> = Vec::new();
        let (ruleset, molecule) = input.split_once("\n\n").unwrap();
        let molecule = molecule.trim();
        ruleset.lines().for_each(|l| {
            let (k, v) = l.split_once(" => ").unwrap();
            rules.push((k, v));
        });

        rules.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        Self { rules, molecule }
    }

    pub fn successors(&self, input: &str, is_part_1: bool) -> HashSet<String> {
        let mut results: HashSet<String> = HashSet::new();

        self.rules.iter().for_each(|(rule, replacement)| {
            input.match_indices(rule).for_each(|(index, _)| {
                if index + rule.len() >= input.len() {
                    results.insert(format!("{}{}", &input[0..index], replacement));
                } else {
                    results.insert(format!(
                        "{}{}{}",
                        &input[0..index],
                        replacement,
                        &input[index + rule.len()..]
                    ));
                }
            });
        });

        if !is_part_1 {
            results.retain(|v| v.len() <= self.molecule.len());
        }
        results
    }
    pub fn reversed_successors(&self, input: &str) -> DashSet<(String, usize)> {
        let results: DashSet<(String, usize)> = DashSet::new();

        self.rules.iter().for_each(|(rule, replacement)| {
            if results.is_empty() {
                input.match_indices(replacement).for_each(|(index, _)| {
                    if index + replacement.len() >= input.len() {
                        results.insert((format!("{}{}", &input[0..index], rule), 1));
                    } else {
                        results.insert((
                            format!(
                                "{}{}{}",
                                &input[0..index],
                                rule,
                                &input[index + replacement.len()..]
                            ),
                            1,
                        ));
                    }
                });
            }
        });

        results
    }

    pub fn solve_1(&self) -> Option<usize> {
        Some(self.successors(self.molecule, true).len())
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let replacer = Replacer::parse_input(input);

    replacer.solve_1()
}

pub fn part_two(input: &str) -> Option<usize> {
    let replacer = Replacer::parse_input(input);

    let start = replacer.molecule.to_string();

    let result = astar(
        &start,
        |input| replacer.reversed_successors(input),
        |input| input.len() / 3,
        |input| input == "e",
    );

    Some(result.unwrap().1)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_reversed_successors() {
    //     let input = advent_of_code::template::read_file("examples", DAY);
    //     let replacer = Replacer::parse_input(&input);
    //
    //     let results = replacer.reversed_successors("HOOH");
    // }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
