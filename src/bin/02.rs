advent_of_code::solution!(2);

#[derive(Debug, Clone, Copy)]
pub struct Dimensions(u32, u32, u32);

impl Dimensions {
    pub fn needed_paper(&self) -> u32 {
        3 * self.0 * self.1 + 2 * self.1 * self.2 + 2 * self.2 * self.0
    }
    pub fn needed_ribbon_total(&self) -> u32 {
        self.needed_bow() + self.needed_ribbon_wrap()
    }
    pub fn needed_ribbon_wrap(&self) -> u32 {
        2 * self.0 + 2 * self.1
    }
    pub fn needed_bow(&self) -> u32 {
        self.0 * self.1 * self.2
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let mut dimensions: Vec<u32> =
                    l.split('x').map(|d| d.parse::<u32>().unwrap()).collect();
                dimensions.sort();
                Dimensions(dimensions[0], dimensions[1], dimensions[2])
            })
            .map(|d| d.needed_paper())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let mut dimensions: Vec<u32> =
                    l.split('x').map(|d| d.parse::<u32>().unwrap()).collect();
                dimensions.sort();
                Dimensions(dimensions[0], dimensions[1], dimensions[2])
            })
            .map(|d| d.needed_ribbon_total())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(101));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
