use serde_json::Value;

advent_of_code::solution!(12);

fn sum_numbers(value: &Value) -> i64 {
    let mut total: i64 = 0;

    if value.is_object() {
        total += value
            .as_object()
            .unwrap()
            .iter()
            .map(|(_, sv)| sum_numbers(sv))
            .sum::<i64>();
    } else if value.is_array() {
        total += value
            .as_array()
            .unwrap()
            .iter()
            .map(sum_numbers)
            .sum::<i64>();
    } else if value.is_i64() {
        total += value.as_i64().unwrap();
    }

    total
}
fn sum_numbers_ignore_red(value: &Value) -> i64 {
    let mut total: i64 = 0;

    if value.is_object() {
        let obj = value.as_object().unwrap();
        let has_red = obj
            .iter()
            .any(|(_, sv)| sv.is_string() && sv.as_str().unwrap() == "red");
        if !has_red {
            total += obj
                .iter()
                .map(|(_, sv)| sum_numbers_ignore_red(sv))
                .sum::<i64>();
        }
    } else if value.is_array() {
        total += value
            .as_array()
            .unwrap()
            .iter()
            .map(sum_numbers_ignore_red)
            .sum::<i64>();
    } else if value.is_i64() {
        total += value.as_i64().unwrap();
    }

    total
}

pub fn part_one(input: &str) -> Option<u64> {
    let data: Value = serde_json::from_str(input).unwrap();

    Some(sum_numbers(&data) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data: Value = serde_json::from_str(input).unwrap();

    Some(sum_numbers_ignore_red(&data) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
