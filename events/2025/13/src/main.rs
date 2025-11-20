use std::collections::HashSet;

use itertools::Itertools;
use memoize::memoize;

fn main() {
    //p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q13_p1.txt").unwrap();
    let numbers = input.trim().lines().map(|l| l.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut lock = vec![1];
    lock.extend(numbers.iter().step_by(2));
    lock.extend(numbers.iter().skip(1).step_by(2).rev());
    let turns = 2025;
    let rem = turns % lock.len();
    println!("Part 1: {}", lock[rem]);

    //p2
    let input = std::fs::read_to_string("everybody_codes_e2025_q13_p2.txt").unwrap();
    let result = solve(&input, 20252025);
    println!("Part 2: {}", result);

    //p3
    let input = std::fs::read_to_string("everybody_codes_e2025_q13_p3.txt").unwrap();
    let result = solve(&input, 202520252025);
    println!("Part 3: {}", result);
}

fn solve(input: &str, turns: usize) -> usize {
    let numbers = input
        .trim()
        .lines()
        .map(|l| {
            let (s, e) = l.split_once("-").unwrap();
            (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    let mut lock_ranges = vec![(1, 1)];
    lock_ranges.extend(numbers.iter().step_by(2));
    lock_ranges.extend(numbers.iter().skip(1).step_by(2).map(|p| (p.1, p.0)).rev());
    let lock = lock_ranges
        .iter()
        .flat_map(|r| {
            let (min, max) = (r.0.min(r.1), r.0.max(r.1));
            if r.0 <= r.1 { (min..=max).collect::<Vec<_>>() } else { (min..=max).rev().collect::<Vec<_>>() }
        })
        .collect::<Vec<_>>();
    lock[turns % lock.len()]
}
