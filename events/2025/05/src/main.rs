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
            (id, qual, levels)
        })
        .sorted_by(cmp_swords)
        .rev()
        .enumerate()
        .map(|(idx, (id, _, _))| (idx as i64 + 1) * id)
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

fn cmp_swords((l_id, l_qual, l_levels): &(i64, i64, Vec<i64>), (r_id, r_qual, r_levels): &(i64, i64, Vec<i64>)) -> Ordering {
    if l_qual != r_qual {
        return l_qual.cmp(r_qual); // sort by quality
    }

    // sort by levels
    let mut idx = 0;
    while idx < l_levels.len() && idx < r_levels.len() {
        if l_levels[idx] < r_levels[idx] {
            return Ordering::Less;
        } else if l_levels[idx] > r_levels[idx] {
            return Ordering::Greater;
        }
        idx += 1;
    }
    if idx == l_levels.len() && idx == r_levels.len() {
        // sort by id
        if l_id > r_id {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    } else if idx == l_levels.len() {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
