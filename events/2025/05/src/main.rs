use std::{cmp::Ordering, iter::*};

use itertools::Itertools;

fn main() {
    //p1
    let filename = "everybody_codes_e2025_q05_p1a.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let (_, r) = input.trim().split_once(":").unwrap();
    let fishbone = parse_fishbone(r);
    let r1 = fishbone.iter().map(|e| e.1.to_string()).collect::<Vec<String>>().join("");
    println!("Part1: {}", r1);

    //p2
    let filename = "everybody_codes_e2025_q05_p2a.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let qualities = input
        .lines()
        .map(|line| {
            let (_, r) = line.trim().split_once(":").unwrap();
            quality(&parse_fishbone(r))
        })
        .collect::<Vec<_>>();
    let best = qualities.iter().max().unwrap();
    let weakest = qualities.iter().min().unwrap();
    let dist = best - weakest;
    println!("Part2: {}", dist);

    //p3
    let filename = "everybody_codes_e2025_q05_p3a.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let r3 = input
        .lines()
        .map(|line| {
            let (id, r) = line.trim().split_once(":").unwrap();
            let id = id.parse::<i64>().unwrap();
            let fishbone = parse_fishbone(r);
            let qual = quality(&fishbone);
            let levels = levels(&fishbone);
            (qual, levels, id)
        })
        .sorted()
        .rev()
        // .inspect(|e| println!("{:?}", e))
        .enumerate()
        .map(|(idx, (_, _, id))| (idx as i64 + 1) * id)
        .sum::<i64>();
    println!("Part3: {}", r3);
}

fn parse_fishbone(input: &str) -> Vec<(Option<i64>, i64, Option<i64>)> {
    let v = input.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let fishbone = v.iter().fold(Vec::new(), |mut acc, n| {
        for idx in 0..acc.len() {
            let (l, c, r): (Option<i64>, i64, Option<i64>) = acc[idx];
            if l.is_none() && n < &c {
                acc[idx].0 = Some(*n);
                return acc;
            } else if r.is_none() && n > &c {
                acc[idx].2 = Some(*n);
                return acc;
            }
        }
        acc.push((None, *n, None));
        acc
    });
    fishbone
}

fn quality(fishbone: &Vec<(Option<i64>, i64, Option<i64>)>) -> i64 {
    fishbone.iter().map(|e| e.1.to_string()).collect::<Vec<String>>().join("").parse::<i64>().unwrap()
}

fn levels(fishbone: &Vec<(Option<i64>, i64, Option<i64>)>) -> Vec<i64> {
    fishbone
        .iter()
        .map(|e| format!("{}{}{}", e.0.unwrap_or(0), e.1, if let Some(r) = e.2 { &r.to_string() } else { "" }).parse::<i64>().unwrap())
        .collect::<_>()
}
