use itertools::Itertools;

advent_of_code::solution!(17);

fn count_combinations(containers: &[u8], target: i32) -> usize {
    fn helper(containers: &[u8], target: i32, index: usize) -> usize {
        // Base cases
        if target == 0 {
            return 1; // Found a valid combination
        }
        if target < 0 || index >= containers.len() {
            return 0; // Invalid combination
        }

        // Recursive case: Include the current container or exclude it
        helper(containers, target - containers[index] as i32, index + 1)
            + helper(containers, target, index + 1)
    }

    helper(containers, target, 0)
}

pub fn part_one(input: &str) -> Option<usize> {
    let containers: Vec<u8> = input.lines().map(|l| l.parse::<u8>().unwrap()).collect();
    let target = 150;

    Some(count_combinations(&containers, target))
}

pub fn part_two(input: &str) -> Option<usize> {
    let containers: Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();

    let target = 150;

    for k in 0..containers.len() {
        let result = containers
            .iter()
            .combinations(k)
            .filter(|it| it.iter().copied().sum::<usize>() == target)
            .count();
        if result > 0 {
            return Some(result);
        }
    }

    None
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
