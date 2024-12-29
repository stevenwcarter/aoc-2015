advent_of_code::solution!(20);

use advent_of_code::presents_delivered_to_house_part1;
use advent_of_code::presents_delivered_to_house_part2;
use std::{
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread::available_parallelism,
};

use rayon::prelude::*;

fn binary_search_for_start(target: usize, start: usize, cycles: usize, max_below: usize) -> usize {
    let delivered_count = presents_delivered_to_house_part2(start);
    // println!(
    //     "Checking {start} - {delivered_count} - {}",
    //     delivered_count > target
    // );
    if cycles == 0 {
        return max_below;
    }

    if delivered_count > target {
        binary_search_for_start(target, start / 2, cycles - 1, max_below)
    } else {
        binary_search_for_start(
            target,
            start + (start / 10),
            cycles - 1,
            max_below.max(start),
        )
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let target = input.trim().parse::<usize>().ok()?;

    let thread_count = available_parallelism().unwrap().get().max(16);
    let found = AtomicBool::new(false);
    let steps = AtomicUsize::new(0);

    let start_offset = target / 50;

    (0..thread_count).into_par_iter().for_each(|offset| {
        let mut i = 0;
        loop {
            if found.load(Ordering::Relaxed) {
                return;
            }
            let check = i * thread_count + offset + start_offset;

            if presents_delivered_to_house_part1(check) >= target {
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

pub fn part_two(input: &str) -> Option<usize> {
    let target = input.trim().parse::<usize>().ok()?;

    let thread_count = available_parallelism().unwrap().get().max(16);
    let found = AtomicBool::new(false);
    let steps = AtomicUsize::new(0);

    let start = binary_search_for_start(target, 1000000, 20, 0);
    // println!("Found start: {start}");
    let start_offset = start / 2;

    // let start_offset = target / 50;

    (0..thread_count).into_par_iter().for_each(|offset| {
        let mut i = 0;
        loop {
            if found.load(Ordering::Relaxed) {
                return;
            }
            let check = i * thread_count + offset + start_offset;
            // if check % 10000 == 0 {
            //     println!("{check} - {}", presents_delivered_to_house_part2(check));
            // }

            if presents_delivered_to_house_part2(check) >= target {
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
