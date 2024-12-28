use std::{
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread::available_parallelism,
};

use rayon::prelude::*;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.trim();

    let thread_count = available_parallelism().unwrap().get().max(16);
    let found = AtomicBool::new(false);
    let steps = AtomicUsize::new(0);

    (0..thread_count).into_par_iter().for_each(|offset| {
        let mut i = 0;
        loop {
            if found.load(Ordering::Relaxed) {
                return;
            }
            let check = i * thread_count + offset + 254575;
            let hash = md5::compute(format!("{input}{check}"));
            if !hash[0] == 0 && !hash[1] == 0 {
                i += 1;
                continue;
            }
            let hash = format!("{:x}", hash);
            if hash.starts_with("00000") {
                found.store(true, Ordering::Relaxed);
                steps.store(check, Ordering::Relaxed);
                break;
            } else {
                i += 1;
            }
        }
    });

    Some(steps.load(Ordering::Relaxed))
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.trim();

    let thread_count = available_parallelism().unwrap().get().max(16);
    let found = AtomicBool::new(false);
    let steps = AtomicUsize::new(0);

    (0..thread_count).into_par_iter().for_each(|offset| {
        let mut i = 0;
        loop {
            if found.load(Ordering::Relaxed) {
                return;
            }
            let check = i * thread_count + offset + 254575;
            let hash = md5::compute(format!("{input}{check}"));
            if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
                found.store(true, Ordering::Relaxed);
                steps.store(check, Ordering::Relaxed);
                break;
            } else {
                i += 1;
            }
        }
    });

    Some(steps.load(Ordering::Relaxed))
}
