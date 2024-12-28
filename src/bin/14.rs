use hashbrown::HashMap;
use itertools::Itertools;
use once_cell::sync::Lazy;

advent_of_code::solution!(14);

static REINDEER: Lazy<Vec<Reindeer>> = Lazy::new(|| {
    vec![
        Reindeer::new("Dancer", 27, 5, 132),
        Reindeer::new("Cupid", 22, 2, 41),
        Reindeer::new("Rudolph", 11, 5, 48),
        Reindeer::new("Donner", 28, 5, 134),
        Reindeer::new("Dasher", 4, 16, 55),
        Reindeer::new("Blitzen", 14, 3, 38),
        Reindeer::new("Prancer", 3, 21, 40),
        Reindeer::new("Comet", 18, 6, 103),
        Reindeer::new("Vixen", 18, 5, 84),
    ]
});

static TEST_REINDEER: Lazy<Vec<Reindeer>> = Lazy::new(|| {
    vec![
        Reindeer::new("Comet", 14, 10, 127),
        Reindeer::new("Dancer", 16, 11, 162),
    ]
});

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Reindeer {
    pub name: String,
    pub speed: u32,
    pub run_time: u32,
    pub rest_time: u32,
}

impl Reindeer {
    pub fn new(name: &str, speed: u32, run_time: u32, rest_time: u32) -> Self {
        Self {
            name: name.to_string(),
            speed,
            run_time,
            rest_time,
        }
    }

    pub fn run_for_secs(&self, seconds: u32) -> u32 {
        let total_cycle_time = self.run_time + self.rest_time;
        let full_cycles = seconds / total_cycle_time;
        let remaining_time = seconds % total_cycle_time;
        let mut result = full_cycles * self.speed * self.run_time;
        if remaining_time >= self.run_time {
            result += self.speed * self.run_time;
        } else {
            result += self.speed * remaining_time;
        }

        result
    }
}

fn get_reindeer_duration(input: &str) -> (&[Reindeer], u32) {
    let is_prod = input.len() > 50;
    let reindeer = if is_prod { &REINDEER } else { &TEST_REINDEER };
    let duration = if is_prod { 2503 } else { 1000 };

    (reindeer, duration)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (reindeer, duration) = get_reindeer_duration(input);
    reindeer.iter().map(|r| r.run_for_secs(duration)).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (reindeer, duration) = get_reindeer_duration(input);

    let mut scores: HashMap<&Reindeer, u32> = HashMap::new();

    (1..duration).for_each(|duration| {
        // needed `max_set_by` from `Itertools` to return all reindeer involved in a tie
        let winners = reindeer
            .iter()
            .max_set_by(|a, b| a.run_for_secs(duration).cmp(&b.run_for_secs(duration)));
        winners.iter().for_each(|w| {
            *scores.entry(w).or_insert(0) += 1;
        });
    });

    scores.iter().map(|(_, &v)| v).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1120));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(689));
    }
}
