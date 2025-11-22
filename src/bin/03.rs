use aoc_mine::Coord;
use hashbrown::HashMap;

advent_of_code::solution!(3);

fn handle_direction(coord: &mut Coord<i32>, ch: char) {
    match ch {
        '^' => coord.move_up(),
        '>' => coord.move_right(),
        'v' => coord.move_down(),
        '<' => coord.move_left(),
        _ => unreachable!("Could not handle character {ch}"),
    };
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut houses: HashMap<Coord<i32>, u32> = HashMap::new();
    let mut coord = Coord::new(0, 0);
    *houses.entry(coord).or_insert(0) += 1;
    input.trim().chars().for_each(|ch| {
        handle_direction(&mut coord, ch);

        *houses.entry(coord).or_insert(0) += 1;
    });

    Some(houses.iter().filter(|(_, t)| **t > 0).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut houses: HashMap<Coord<i32>, u32> = HashMap::new();
    let mut santa = Coord::new(0, 0);
    let mut robot = Coord::new(0, 0);
    *houses.entry(santa).or_insert(0) += 1;
    *houses.entry(robot).or_insert(0) += 1;
    input.trim().chars().enumerate().for_each(|(index, ch)| {
        if index.is_multiple_of(2) {
            handle_direction(&mut santa, ch);
            *houses.entry(santa).or_insert(0) += 1;
        } else {
            handle_direction(&mut robot, ch);
            *houses.entry(robot).or_insert(0) += 1;
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
