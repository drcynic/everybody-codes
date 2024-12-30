//use std::collections::HashMap;
use std::iter::*;

//use itertools::{Either, Itertools};

fn main() {
    let input = std::fs::read_to_string("everybody_codes_e2024_q01_p1.txt").unwrap();
    let p1: i32 = eval(&input, 1);
    println!("Part1: {}", p1);

    let input = std::fs::read_to_string("everybody_codes_e2024_q01_p2.txt").unwrap();
    let p2: i32 = eval(&input, 2);
    println!("Part2: {:?}", p2);

    let input = std::fs::read_to_string("everybody_codes_e2024_q01_p3.txt").unwrap();
    let p3: i32 = eval(&input, 3);
    println!("Part3: {:?}", p3);
}

fn eval(input: &str, chunk_size: usize) -> i32 {
    input
        .as_bytes()
        .chunks(chunk_size)
        .map(String::from_utf8_lossy)
        .map(|s| {
            let sum: i32 = s
                .chars()
                .map(|c| match c {
                    'A' => 0,
                    'B' => 1,
                    'C' => 3,
                    'D' => 5,
                    'x' => 0,
                    _ => unreachable!(),
                })
                .sum();
            let count = chunk_size - s.matches("x").count();
            match count {
                3 => sum + 6,
                2 => sum + 2,
                1 | 0 => sum,
                _ => unreachable!(),
            }
        })
        .sum()
}
