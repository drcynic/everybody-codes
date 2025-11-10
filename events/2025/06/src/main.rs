use std::cmp::{self};

use itertools::Itertools;

fn main() {
    // //p1
    let filename = "everybody_codes_e2025_q06_p1a.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let sum = sum_novices(&input, b'A');
    println!("Part1: {:?}", sum);

    // // p2
    let filename = "everybody_codes_e2025_q06_p2a.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let sum = [b'A', b'B', b'C'].iter().map(|m| sum_novices(&input, *m)).sum::<i32>();
    println!("Part2: {:?}", sum);

    // p3
    let filename = "everybody_codes_e2025_q06_p3a.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let input = input.trim().as_bytes();
    let repeat = 2;
    let max_dist = 10;
    let letters = input.iter().cycle().take(input.len() * repeat).collect::<Vec<_>>();
    let sum = letters.iter().enumerate().fold(0, |mut acc, (idx, c)| {
        if c.is_ascii_uppercase() {
            return acc;
        }
        let s = c.to_ascii_uppercase();
        for i in idx.saturating_sub(max_dist)..cmp::min(idx + max_dist + 1, letters.len()) {
            if letters[i] == &s {
                acc += 1;
            }
        }
        acc
    });
    println!("Part3: {:?}", sum);
}

fn sum_novices(input: &str, mentor: u8) -> i32 {
    let novice = mentor.to_ascii_lowercase();
    let letters = input.trim().as_bytes();
    let mut sum = 0;
    for mp in letters.iter().positions(|&x| x == mentor) {
        for i in (mp + 1)..letters.len() {
            if letters[i] == novice {
                sum += 1;
            }
        }
    }
    sum
}
