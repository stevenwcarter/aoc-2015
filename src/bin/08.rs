advent_of_code::solution!(8);
use rayon::prelude::*;
use unescaper::unescape;

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .par_bridge()
            .map(|l| {
                let orig_len = l.len();
                let new = unescape(&l[1..orig_len - 1]).unwrap();
                let new_len = new.chars().collect::<Vec<_>>().len();
                orig_len - new_len
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .par_bridge()
            .map(|l| {
                let orig_len = l.chars().collect::<Vec<_>>().len();
                let new = l.escape_debug().to_string();
                let new_len = new.chars().collect::<Vec<_>>().len();
                new_len - orig_len + 2
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}
