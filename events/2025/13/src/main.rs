use std::collections::HashSet;

use itertools::Itertools;
use memoize::memoize;

fn main() {
    //p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q13_p1.txt").unwrap();
    let numbers = input.trim().lines().map(|l| l.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut lock = Vec::new();
    lock.resize(numbers.len() + 1, 0usize);
    let len = lock.len();
    lock[0] = 1usize;
    lock[len / 2] = *numbers.last().unwrap();
    for (i, (s, e)) in numbers.iter().tuples().enumerate() {
        lock[i + 1] = *s;
        lock[len - i - 1] = *e;
    }
    let turns = 2025;
    let rem = turns % lock.len();
    println!("Part 1: {}", lock[rem]);

    //p2
    let input = std::fs::read_to_string("everybody_codes_e2025_q13_p2.txt").unwrap();
    let numbers = input
        .trim()
        .lines()
        .map(|l| {
            let (s, e) = l.split_once("-").unwrap();
            (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    let mut lock_ranges = Vec::new();
    lock_ranges.resize(numbers.len() + 1, (0, 0));
    let len = lock_ranges.len();
    lock_ranges[0] = (1, 1);
    let last = numbers.last().unwrap();
    lock_ranges[len / 2] = last.clone();
    for (i, (s, e)) in numbers.iter().tuples().enumerate() {
        lock_ranges[i + 1] = *s;
        lock_ranges[len - i - 1] = (e.1, e.0);
    }
    let lock = lock_ranges
        .iter()
        .flat_map(|r| {
            let (min, max) = (r.0.min(r.1), r.0.max(r.1));
            if r.0 <= r.1 { (min..=max).collect::<Vec<_>>() } else { (min..=max).rev().collect::<Vec<_>>() }
        })
        .collect::<Vec<_>>();
    let turns = 20252025;
    let rem = turns % lock.len();
    println!("Part 2: {}", lock[rem]);

    //p3
    let input = std::fs::read_to_string("everybody_codes_e2025_q13_p3.txt").unwrap();
    let numbers = input
        .trim()
        .lines()
        .map(|l| {
            let (s, e) = l.split_once("-").unwrap();
            (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    let mut lock_ranges = Vec::new();
    lock_ranges.resize(numbers.len() + 1, (0, 0));
    let len = lock_ranges.len();
    lock_ranges[0] = (1, 1);
    let last = numbers.last().unwrap();
    lock_ranges[len / 2] = last.clone();
    for (i, (s, e)) in numbers.iter().tuples().enumerate() {
        lock_ranges[i + 1] = *s;
        lock_ranges[len - i - 1] = (e.1, e.0);
    }
    let lock = lock_ranges
        .iter()
        .flat_map(|r| {
            let (min, max) = (r.0.min(r.1), r.0.max(r.1));
            if r.0 <= r.1 { (min..=max).collect::<Vec<_>>() } else { (min..=max).rev().collect::<Vec<_>>() }
        })
        .collect::<Vec<_>>();
    let turns = 202520252025;
    let rem = turns % lock.len();
    println!("Part 3: {}", lock[rem]);
}
