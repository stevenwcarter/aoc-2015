advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.trim();

    let mut idx = 0;

    loop {
        let hash = md5::compute(format!("{input}{idx}"));
        if !hash[0] == 0 && !hash[1] == 0 {
            idx += 1;
            continue;
        }
        let hash = format!("{:x}", hash);
        if hash.starts_with("00000") {
            return Some(idx);
        } else {
            idx += 1;
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.trim();

    let mut idx = 254575; // part 1 ans

    loop {
        let hash = md5::compute(format!("{input}{idx}"));
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return Some(idx);
        } else {
            idx += 1;
            continue;
        }
    }
}

// #[cfg(test)]
// mod tests {
// use super::*;

// #[test]
// fn test_part_one() {
//     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//     assert_eq!(result, None);
// }
//
// #[test]
// fn test_part_two() {
//     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//     assert_eq!(result, None);
// }
// }
