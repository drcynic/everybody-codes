use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

fn main() {
    //p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q09_p1.txt").unwrap();
    let mut pos = input.lines().map(|s| s.split_once(":").unwrap().1).collect::<Vec<_>>();
    let child = pos.pop().unwrap();
    let mut p1 = 1;
    for p in &pos {
        p1 *= p.as_bytes().iter().zip(child.as_bytes()).filter(|(p, c)| p == c).count();
    }
    println!("p1: {:?}", p1);

    let input = std::fs::read_to_string("everybody_codes_e2025_q09_p2.txt").unwrap();
    let pos = input.lines().map(|s| s.split_once(":").unwrap().1).collect::<Vec<_>>();
    let mut parents_by_child = HashMap::new();
    for i in 0..pos.len() {
        for j in 0..pos.len() {
            for k in 0..pos.len() {
                if i == j || j == k || i == k {
                    continue;
                }
                let child = pos[i];
                let p1 = pos[j];
                let p2 = pos[k];
                let is_child = p1.as_bytes().iter().zip(p2.as_bytes()).zip(child.as_bytes()).all(|((p1, p2), c)| p1 == c || p2 == c);
                if is_child {
                    parents_by_child.insert(child, (p1, p2));
                }
            }
        }
    }
    let p2 = parents_by_child
        .iter()
        .map(|(child, (p1, p2))| {
            let l = p1.as_bytes().iter().zip(child.as_bytes()).filter(|(p, c)| p == c).count();
            let r = p2.as_bytes().iter().zip(child.as_bytes()).filter(|(p, c)| p == c).count();
            l * r
        })
        .sum::<usize>();
    println!("p2: {:?}", p2);

    // p3
    let input = std::fs::read_to_string("everybody_codes_e2025_q09_p3.txt").unwrap();
    let pos = input.lines().map(|s| s.split_once(":").unwrap().1).collect::<Vec<_>>();
    let mut families: Vec<HashSet<usize>> = Vec::new();
    for i in 0..pos.len() {
        for j in 0..pos.len() {
            for k in 0..pos.len() {
                if i == j || j == k || i == k {
                    continue;
                }
                let child = pos[i];
                let p1 = pos[j];
                let p2 = pos[k];
                let is_child = p1.as_bytes().iter().zip(p2.as_bytes()).zip(child.as_bytes()).all(|((p1, p2), c)| p1 == c || p2 == c);
                if is_child {
                    let con_fam = families
                        .iter()
                        .enumerate()
                        .filter(|(_, f)| f.contains(&i) || f.contains(&j) || f.contains(&k))
                        .map(|(fi, _)| fi)
                        .collect::<Vec<_>>();
                    if !con_fam.is_empty() {
                        let mut combined = con_fam.iter().fold(HashSet::new(), |mut acc, fi| {
                            acc.extend(&families[*fi]);
                            acc
                        });
                        combined.extend([i, j, k]);
                        con_fam.iter().sorted().rev().for_each(|ri| {
                            families.remove(*ri);
                        });
                        families.push(combined);
                    } else {
                        families.push(HashSet::from([i, j, k]));
                    }
                }
            }
        }
    }
    let max_idx = families.iter().map(|f| f.len()).enumerate().max_by(|(_, a), (_, b)| a.cmp(b)).map(|(i, _)| i).unwrap();
    let p3 = families[max_idx].iter().map(|&id| id + 1).sum::<usize>();
    println!("p3: {:?}", p3);
}
