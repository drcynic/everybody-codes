use std::{collections::HashMap, iter::*};

use itertools::Itertools;

fn main() {
    let filename = "everybody_codes_e2025_q03_p1a.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let r1 = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).unique().sum::<i64>();
    println!("Part1: {}", r1);

    let filename = "everybody_codes_e2025_q03_p2a.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let r2 = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).sorted().dedup().take(20).sum::<i64>();
    println!("Part2: {}", r2);

    let filename = "everybody_codes_e2025_q03_p3a.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let r3 = *input
        .trim()
        .split(',')
        .fold(HashMap::new(), |mut acc, s| {
            *acc.entry(s.parse::<i64>().unwrap()).or_insert(0) += 1;
            acc
        })
        .values()
        .max()
        .unwrap();
    println!("Part3: {}", r3);
}
