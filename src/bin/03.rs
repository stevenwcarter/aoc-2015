use hashbrown::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
    let mut houses: HashMap<(i32, i32), u32> = HashMap::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    *houses.entry((x, y)).or_insert(0) += 1;
    input.trim().chars().for_each(|ch| {
        match ch {
            '^' => y -= 1,
            '>' => x += 1,
            'v' => y += 1,
            '<' => x -= 1,
            _ => unreachable!("Could not handle character {ch}"),
        };

        *houses.entry((x, y)).or_insert(0) += 1;
    });

    Some(houses.iter().filter(|(_, t)| **t > 0).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut houses: HashMap<(i32, i32), u32> = HashMap::new();
    let mut santa_x: i32 = 0;
    let mut santa_y: i32 = 0;
    let mut robot_x: i32 = 0;
    let mut robot_y: i32 = 0;
    *houses.entry((santa_x, santa_y)).or_insert(0) += 1;
    *houses.entry((robot_x, robot_y)).or_insert(0) += 1;
    input.trim().chars().enumerate().for_each(|(index, ch)| {
        if index % 2 == 0 {
            match ch {
                '^' => santa_y -= 1,
                '>' => santa_x += 1,
                'v' => santa_y += 1,
                '<' => santa_x -= 1,
                _ => unreachable!("Could not handle character {ch} for santa"),
            };
            *houses.entry((santa_x, santa_y)).or_insert(0) += 1;
        } else {
            match ch {
                '^' => robot_y -= 1,
                '>' => robot_x += 1,
                'v' => robot_y += 1,
                '<' => robot_x -= 1,
                _ => unreachable!("Could not handle character {ch} for robot"),
            };
            *houses.entry((robot_x, robot_y)).or_insert(0) += 1;
        }
    });

    Some(houses.iter().filter(|(_, t)| **t > 0).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
