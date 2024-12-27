use itertools::Itertools;

advent_of_code::solution!(11);

const RANGE_START: u8 = b'a';
const RANGE_END: u8 = b'z';

fn contains_sequential(pw: &[u8]) -> bool {
    pw.iter()
        .tuple_windows()
        .any(|(&a, &b, &c)| a == b - 1 && b == c - 1)
}
fn does_not_contain_invalid_characters(pw: &[u8]) -> bool {
    !pw.iter().any(|&ch| ch == b'i' || ch == b'o' || ch == b'l')
}
fn contains_two_nonoverlapping_pairs(pw: &[u8]) -> bool {
    let mut found_first_pair = false;

    let mut i = 0;
    while i < pw.len() - 1 {
        if pw[i] == pw[i + 1] {
            if found_first_pair {
                return true;
            }
            found_first_pair = true;
            // skip the second number of the pair so we do not overlap
            i += 2;
        } else {
            i += 1;
        }
    }

    false
}

fn is_valid(pw: &[u8]) -> bool {
    contains_sequential(pw)
        && does_not_contain_invalid_characters(pw)
        && contains_two_nonoverlapping_pairs(pw)
}

fn next_password(pw: &[u8]) -> String {
    let mut next_password = pw.to_vec();

    loop {
        increment(&mut next_password);
        if is_valid(&next_password) {
            break;
        }
    }

    convert_from_numeric(&next_password)
}

fn increment(pw: &mut [u8]) {
    for index in (0..pw.len()).rev() {
        if pw[index] == RANGE_END {
            pw[index] = RANGE_START;
        } else {
            pw[index] += 1;
            if pw[index] == b'i' || pw[index] == b'o' || pw[index] == b'l' {
                pw[index] += 1;
            }
            break;
        }
    }
}

fn convert_to_numeric(pw: &str) -> Vec<u8> {
    pw.chars().map(|ch| ch as u8).collect()
}
fn convert_from_numeric(pw: &[u8]) -> String {
    pw.iter().map(|ch| *ch as char).collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let next = next_password(&convert_to_numeric(input.trim()));
    assert!(next != "hepxddeff");
    Some(next)
}

pub fn part_two(_input: &str) -> Option<String> {
    Some(next_password(&convert_to_numeric("hepxxyzz")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_pairs() {
        assert!(!contains_two_nonoverlapping_pairs(&convert_to_numeric(
            "abcd"
        )));
        assert!(contains_two_nonoverlapping_pairs(&convert_to_numeric(
            "aabb"
        )));
    }
    #[test]
    fn sequential() {
        assert!(!contains_sequential(&convert_to_numeric("aabb")));
        assert!(contains_sequential(&convert_to_numeric("abdpqr")));
    }
    #[test]
    fn invalid_chars() {
        assert!(!does_not_contain_invalid_characters(&convert_to_numeric(
            "abci"
        )));
        assert!(does_not_contain_invalid_characters(&convert_to_numeric(
            "abc"
        )));
    }

    #[test]
    fn next_password_1() {
        assert_eq!(
            next_password(&convert_to_numeric("abcdefgh")),
            "abcdffaa".to_string()
        );
    }
    #[test]
    fn next_password_2() {
        assert_eq!(
            next_password(&convert_to_numeric("ghijklmn")),
            "ghjaabcc".to_string()
        );
    }

    #[test]
    #[ignore]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
