//use std::collections::HashMap;
use std::{
    collections::{HashMap, VecDeque},
    iter::*,
};

//use itertools::{Either, Itertools};

fn main() {
    let input = std::fs::read_to_string("everybody_codes_e1_q01_p1.txt").unwrap();
    let highest = input
        .lines()
        .map(|l| {
            let e = l.split_whitespace().map(|e| e.split_once('=').unwrap().1.parse::<u64>().unwrap()).collect::<Vec<_>>();
            eni(1, e[0], e[3], e[6]) + eni(1, e[1], e[4], e[6]) + eni(1, e[2], e[5], e[6])
        })
        .max()
        .unwrap();
    println!("p1: {}", highest);

    let input = std::fs::read_to_string("everybody_codes_e1_q01_p2.txt").unwrap();
    let highest = input
        .lines()
        .map(|l| {
            let e = l.split_whitespace().map(|e| e.split_once('=').unwrap().1.parse::<u64>().unwrap()).collect::<Vec<_>>();
            eni2(e[0], e[3], e[6]) + eni2(e[1], e[4], e[6]) + eni2(e[2], e[5], e[6])
        })
        .max()
        .unwrap();
    println!("p2: {}", highest);

    let input = std::fs::read_to_string("everybody_codes_e1_q01_p3.txt").unwrap();
    let highest = input
        .lines()
        .map(|l| {
            let e = l.split_whitespace().map(|e| e.split_once('=').unwrap().1.parse::<u64>().unwrap()).collect::<Vec<_>>();
            eni3(e[0], e[3], e[6]) + eni3(e[1], e[4], e[6]) + eni3(e[2], e[5], e[6])
        })
        .max()
        .unwrap();
    println!("p3: {}", highest);
}

fn eni3(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut pattern = Vec::new();
    let mut sums = Vec::new();
    let mut r = 1;
    let mut start_idx = 0;
    loop {
        r = (r * base) % modulus;
        // find repeating pattern
        if let Some(idx) = pattern.iter().position(|&x| x == r) {
            start_idx = idx;
            break;
        }
        pattern.push(r);
        sums.push(if sums.is_empty() { r } else { sums.last().unwrap() + r });
    }
    let pattern_len = (pattern.len() - start_idx) as u64;
    let mut sum = if start_idx > 0 { sums[start_idx - 1] } else { 0 };
    let pattern_sum = sums.last().unwrap() - sum;

    sum += ((exp - start_idx as u64) / pattern_len) * pattern_sum;
    let remainder_iterations = (exp - start_idx as u64) % pattern_len;
    r = *pattern.last().unwrap();
    for _ in 0..remainder_iterations {
        r = (r * base) % modulus;
        sum += r;
    }

    sum
}

fn eni2(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut exp = exp;
    let mut start = 1;
    if exp > 5 {
        start = square_and_multiply(base, exp - 5, modulus);
        exp = 5;
    }
    eni(start, base, exp, modulus)
}

fn eni(start: u64, base: u64, exp: u64, modulus: u64) -> u64 {
    let mut r = start;
    let mut remainders = VecDeque::new();
    for _ in 0..exp {
        r = (r * base) % modulus;
        remainders.push_front(r);
    }
    remainders.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join("").parse::<u64>().unwrap()
}

fn square_and_multiply(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            // If exp is odd
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus; // Square the base
        exp /= 2; // Divide exponent by 2
    }
    result
}
