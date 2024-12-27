use itertools::Itertools;

advent_of_code::solution!(5);

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const BAD_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn has_three_vowels(input: &str) -> bool {
    input.chars().filter(|ch| VOWELS.contains(ch)).count() >= 3
}

fn has_one_letter_twice(input: &str) -> bool {
    input.chars().tuple_windows().any(|(a, b)| a == b)
}

fn does_not_contain_bad_strings(input: &str) -> bool {
    !BAD_STRINGS.iter().any(|s| input.contains(s))
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|s| has_three_vowels(s))
            .filter(|s| has_one_letter_twice(s))
            .filter(|s| does_not_contain_bad_strings(s))
            .count(),
    )
}

fn has_non_overlapping(input: &str) -> bool {
    if input.len() <= 3 {
        return false;
    }
    (0..input.len() - 2).any(|idx| {
        let source = &input[idx..idx + 2];

        input[idx + 2..].contains(source)
    })
}
fn has_letters_with_one_between(input: &str) -> bool {
    input.chars().tuple_windows().any(|(a, _, c)| a == c)
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|s| has_non_overlapping(s))
            .filter(|s| has_letters_with_one_between(s))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_non_overlapping() {
        assert!(has_non_overlapping("qhirixgwkkccuzlp"));
        assert!(has_non_overlapping("aabcdefgaa"));
        assert!(has_non_overlapping("xyxy"));
        assert!(!has_non_overlapping("aaa"));
        assert!(has_non_overlapping("qjhvhtzxzqqjkmpb"));
        assert!(has_non_overlapping("xxyxx"));
        assert!(has_non_overlapping("uurcxstgmygtbstg"));
        assert!(!has_non_overlapping("ieodomkazucvgmuy"));
    }
    #[test]
    fn test_has_letters_with_one_between() {
        assert!(has_letters_with_one_between("qhirixgwkkccuzlp"));
        assert!(has_letters_with_one_between("abcdefeghi"));
        assert!(has_letters_with_one_between("qjhvhtzxzqqjkmpb"));
        assert!(has_letters_with_one_between("xxyxx"));
        assert!(!has_letters_with_one_between("uurcxstgmygtbstg"));
        assert!(has_letters_with_one_between("ieodomkazucvgmuy"));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
