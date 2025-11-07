use std::iter::*;

use itertools::Itertools;
use num::Integer;

#[allow(unstable_name_collisions)]
fn main() {
    let filename = "everybody_codes_e2025_q04_p1a.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let gears = input.lines().map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let r1 = 2025 * gears[0] / gears.last().unwrap();
    println!("Part1: {}", r1);

    let filename = "everybody_codes_e2025_q04_p2a.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let gears = input.lines().map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let r2 = (10000000000000i64 * gears.last().unwrap()).div_ceil(&gears[0]);
    println!("Part2: {}", r2);

    let filename = "everybody_codes_e2025_q04_p3a.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let gears = input.lines().flat_map(|s| s.trim().split("|").map(|s| s.parse::<i64>().unwrap())).collect::<Vec<i64>>();
    let r3 = gears[1..].iter().tuples().fold(gears[0] * 100, |acc, (l, r)| acc * r / l) / gears.last().unwrap();
    println!("Part3: {}", r3);
}
