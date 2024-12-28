use cached::proc_macro::cached;

advent_of_code::solution!(10);

#[cached]
fn look_and_say(input: String) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(current_char) = chars.next() {
        let mut count = 1;

        while let Some(&next_char) = chars.peek() {
            if next_char == current_char {
                count += 1;
                chars.next();
            } else {
                break;
            }
        }

        result.push_str(&format!("{}{}", count, current_char));
    }

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = input.trim().to_owned();
    for _ in 0..40 {
        input = look_and_say(input);
    }

    Some(input.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input = input.trim().to_owned();
    for _ in 0..50 {
        input = look_and_say(input);
    }

    Some(input.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        assert_eq!(look_and_say("1".to_string()), "11".to_string());
        assert_eq!(look_and_say("11".to_string()), "21".to_string());
        assert_eq!(look_and_say("1211".to_string()), "111221".to_string());
        assert_eq!(look_and_say("21".to_string()), "1211".to_string());
        assert_eq!(look_and_say("111221".to_string()), "312211".to_string());
    }

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some());
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
