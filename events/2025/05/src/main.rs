use std::{cmp::Ordering, iter::*};

use itertools::Itertools;

fn main() {
    let filename = "everybody_codes_e2025_q05_p1.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let (_, r) = input.trim().split_once(":").unwrap();
    let v = r.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
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
    let r1 = fishbone.iter().map(|e| e.1.to_string()).collect::<Vec<String>>().join("");
    println!("Part1: {}", r1);

    let filename = "everybody_codes_e2025_q05_p2.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let fishbones = input
        .lines()
        .map(|line| {
            let (_, r) = line.trim().split_once(":").unwrap();
            let v = r.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
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
        })
        .collect::<Vec<Vec<(Option<i64>, i64, Option<i64>)>>>();
    let weakest = fishbones
        .iter()
        .map(|f| f.iter().map(|e| e.1.to_string()).collect::<Vec<String>>().join("").parse::<i64>().unwrap())
        .min()
        .unwrap();
    let best = fishbones
        .iter()
        .map(|f| f.iter().map(|e| e.1.to_string()).collect::<Vec<String>>().join("").parse::<i64>().unwrap())
        .max()
        .unwrap();
    let dist = best - weakest;
    println!("Part2: {}", dist);

    let filename = "everybody_codes_e2025_q05_p3.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let swords = input
        .lines()
        .map(|line| {
            let (id, r) = line.trim().split_once(":").unwrap();
            let id = id.parse::<i64>().unwrap();
            let v = r.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
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

            let qual = fishbone.iter().map(|e| e.1.to_string()).collect::<Vec<String>>().join("").parse::<i64>().unwrap();
            let levels = fishbone
                .iter()
                .map(|e| {
                    // println!("fishbone: {:?}", e);
                    let mut s = format!("{}{}{}", e.0.unwrap_or(0), e.1, e.2.unwrap_or(0));
                    if e.2.is_none() {
                        s = s[..s.len() - 1].to_string();
                    }
                    let n = s.parse::<i64>().unwrap();
                    // println!("n: {}", n);
                    n
                })
                .collect::<Vec<i64>>();
            // println!("levels: {:?}", levels);

            (id, qual, levels)
        })
        .sorted_by(|a, b| {
            if a.1 != b.1 {
                return a.1.cmp(&b.1);
            }

            let l = &a.2;
            let r = &b.2;
            let mut idx = 0;
            while idx < l.len() && idx < r.len() {
                if l[idx] < r[idx] {
                    return Ordering::Less;
                } else if l[idx] > r[idx] {
                    return Ordering::Greater;
                }
                idx += 1;
            }
            if idx == l.len() && idx == r.len() {
                if a.0 > b.0 {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            } else if idx == l.len() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .rev()
        .collect::<Vec<_>>();

    // println!("sowrds: {:?}", swords);
    for sword in &swords {
        println!("sword: {:?}", sword);
    }
    let r3 = swords.iter().enumerate().map(|(idx, (id, _, _))| (idx as i64 + 1) * id).sum::<i64>();
    println!("Part3: {}", r3);
}
