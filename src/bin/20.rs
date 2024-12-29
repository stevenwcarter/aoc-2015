advent_of_code::solution!(20);

use advent_of_code::presents_delivered_to_house_part1 as presents_delivered_to_house;
use std::{
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread::available_parallelism,
};

use rayon::prelude::*;

// got me in the ballpark
fn binary_search_for_start(target: usize, start: usize, min: usize, max: usize) -> usize {
    println!("Checking {start} Min: {min} Max: {max}");
    let delivered_count = presents_delivered_to_house(start);
    // if delivered_count >= target && presents_delivered_to_house(start - 1) < target {
    //     return start;
    // }

    if delivered_count > target && presents_delivered_to_house(start + 1000) > target {
        let max = max.min(start) - 1;
        let new = ((start - min) / 2) + min;
        binary_search_for_start(target, new, min, max)
    } else if delivered_count < target && presents_delivered_to_house(start - 1000) < target {
        let min = min.max(start);
        let new = ((max - start) / 2) + start;
        binary_search_for_start(target, new, min, max)
    } else {
        return start;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let target = input.trim().parse::<usize>().ok()?;

    let thread_count = available_parallelism().unwrap().get().max(16);
    let found = AtomicBool::new(false);
    let steps = AtomicUsize::new(0);

    let start = binary_search_for_start(target, 1000000, target / 30, 1179540);
    println!("Found start: {start}");
    let start_offset = start * 63 / 100;

    (0..thread_count).into_par_iter().for_each(|offset| {
        let mut i = 0;
        loop {
            if found.load(Ordering::Relaxed) {
                return;
            }
            let check = i * thread_count + offset + start_offset;
            if check % 10000 == 0 {
                println!("{check}");
            }

            if presents_delivered_to_house(check) >= target {
                found.store(true, Ordering::Relaxed);
                steps.store(check, Ordering::Relaxed);
                break;
            } else {
                i += 1;
            }
        }
    });

    // 776160
    Some(steps.load(Ordering::Relaxed))
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
