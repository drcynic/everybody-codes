use std::iter::*;

fn main() {
    let p1 = sum_to_min(&read_nail("everybody_codes_e2024_q04_p1.txt"));
    println!("Part 1: {}", p1);

    let p2 = sum_to_min(&read_nail("everybody_codes_e2024_q04_p2.txt"));
    println!("Part 2: {}", p2);

    let p3 = sum_to_avg(&mut read_nail("everybody_codes_e2024_q04_p3.txt"));
    println!("Part 3: {}", p3);
}

fn sum_to_avg(nails: &mut [i64]) -> i64 {
    nails.sort();
    let pivot = nails[nails.len() / 2];
    let sum = nails.iter().fold(0, |a, n| a + (n - pivot).abs());
    sum
}

fn sum_to_min(nails: &[i64]) -> i64 {
    let min = nails.iter().min().unwrap();
    let sum = nails.iter().fold(0, |a, n| a + (n - min));
    sum
}

fn read_nail(filename: &str) -> Vec<i64> {
    let input = std::fs::read_to_string(filename).unwrap();
    input.trim().lines().map(|line| line.parse::<i64>().unwrap()).collect()
}
