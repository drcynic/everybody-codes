//use std::collections::HashMap;
use std::{
    collections::{HashMap, VecDeque},
    iter::*,
};

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("everybody_codes_e3_q01_p1.txt").unwrap();
    let p1 = input
        .lines()
        .map(|l| {
            let (v, rgb) = l.split_once(':').unwrap();
            let rgb = rgb
                .trim()
                .split_whitespace()
                .map(|s| i32::from_str_radix(&s.chars().map(|c| if c.is_lowercase() { '0' } else { '1' }).collect::<String>(), 2).unwrap())
                .collect::<Vec<i32>>();
            if rgb[1] > rgb[0] && rgb[1] > rgb[2] { v.parse::<i32>().unwrap() } else { 0 }
        })
        .sum::<i32>();
    println!("p1: {p1}");

    let input = std::fs::read_to_string("everybody_codes_e3_q01_p2.txt").unwrap();
    let values = input
        .lines()
        .map(|l| {
            let (v, rgbs) = l.split_once(':').unwrap();
            let rgbs = rgbs
                .trim()
                .split_whitespace()
                .map(|s| i32::from_str_radix(&s.chars().map(|c| if c.is_lowercase() { '0' } else { '1' }).collect::<String>(), 2).unwrap())
                .collect::<Vec<i32>>();
            (v.parse::<i32>().unwrap(), rgbs[..3].iter().sum::<i32>(), rgbs[3])
        })
        .collect::<Vec<(i32, i32, i32)>>();
    let max_shine = values.iter().map(|(_, _, s)| *s).max().unwrap();
    let max_vals = values.iter().filter(|(_, _, s)| *s == max_shine).collect::<Vec<_>>();
    let sorted_max_vals = max_vals.iter().sorted_by(|l, r| l.1.cmp(&r.1)).collect::<Vec<_>>();
    println!("p2: {}", sorted_max_vals[0].0);

    let input = std::fs::read_to_string("everybody_codes_e3_q01_p3.txt").unwrap();
    // let mut group_counts = [0i32; 6]; // rm, rs, gm, gs, bm, bs
    let values = input.lines().fold([(0i32, 0i32); 6], |mut group_counts, l| {
        let (v, rgbs) = l.split_once(':').unwrap();
        let rgbs = rgbs
            .trim()
            .split_whitespace()
            .map(|s| i32::from_str_radix(&s.chars().map(|c| if c.is_lowercase() { '0' } else { '1' }).collect::<String>(), 2).unwrap())
            .collect::<Vec<i32>>();
        let r = rgbs[0];
        let g = rgbs[1];
        let b = rgbs[2];
        let s = rgbs[3];
        let v = v.parse::<i32>().unwrap();
        if (s > 30 && s < 33) || (r == g && r >= b) || (r == b && r >= g) || (g == b && g >= r) {
            return group_counts;
        }
        let shiny_idx_add = if s <= 30 { 0 } else { 1 };
        let idx = if r > g && r > b { 0 } else { if g > b { 2 } else { 4 } };
        group_counts[idx + shiny_idx_add].0 += 1;
        group_counts[idx + shiny_idx_add].1 += v;
        group_counts
    });
    let max = values.iter().map(|(c, _)| *c).max().unwrap();
    let p3 = values.iter().filter(|(c, _)| *c == max).nth(0).unwrap();
    println!("p3: {:?}", p3);
}
