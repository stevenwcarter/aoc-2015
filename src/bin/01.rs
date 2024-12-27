advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .trim()
            .chars()
            .map(|ch| match ch {
                '(' => 1,
                ')' => -1,
                _ => 0,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut current_floor = 0;
    for (index, ch) in input.trim().chars().enumerate() {
        let offset = match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        current_floor += offset;
        if current_floor < 0 {
            return Some(index + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
