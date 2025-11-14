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
    let p1 = pos.iter().map(|p| p.as_bytes().iter().zip(child.as_bytes()).filter(|(p, c)| p == c).count()).product::<usize>();
    println!("p1: {:?}", p1);

    let input = std::fs::read_to_string("everybody_codes_e2025_q09_p2.txt").unwrap();
    let pos = input.lines().map(|s| s.split_once(":").unwrap().1).collect::<Vec<_>>();
    let parents_by_child = find_childs(&pos);
    let p2 = parents_by_child
        .iter()
        .map(|(child, (p1, p2))| {
            let l = pos[*p1].as_bytes().iter().zip(pos[*child].as_bytes()).filter(|(p, c)| p == c).count();
            let r = pos[*p2].as_bytes().iter().zip(pos[*child].as_bytes()).filter(|(p, c)| p == c).count();
            l * r
        })
        .sum::<usize>();
    println!("p2: {:?}", p2);

    // p3
    let input = std::fs::read_to_string("everybody_codes_e2025_q09_p3.txt").unwrap();
    let pos = input.lines().map(|s| s.split_once(":").unwrap().1).collect::<Vec<_>>();
    let parents_by_child = find_childs(&pos);
    let mut families: Vec<HashSet<usize>> = Vec::new();
    for (child, (p1, p2)) in &parents_by_child {
        let con_fam = families
            .iter()
            .enumerate()
            .filter(|(_, f)| f.contains(child) || f.contains(p1) || f.contains(p2))
            .map(|(fi, _)| fi)
            .collect::<Vec<_>>();
        if !con_fam.is_empty() {
            let mut combined = con_fam.iter().fold(HashSet::new(), |mut acc, fi| {
                acc.extend(&families[*fi]);
                acc
            });
            combined.extend([child, p1, p2]);
            con_fam.iter().sorted().rev().for_each(|ri| {
                families.remove(*ri);
            });
            families.push(combined);
        } else {
            families.push(HashSet::from([*child, *p1, *p2]));
        }
    }
    let max_idx = families.iter().map(|f| f.len()).enumerate().max_by(|(_, a), (_, b)| a.cmp(b)).map(|(i, _)| i).unwrap();
    let p3 = families[max_idx].iter().map(|&id| id + 1).sum::<usize>();
    println!("p3: {:?}", p3);
}

fn find_childs(pos: &Vec<&str>) -> HashMap<usize, (usize, usize)> {
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
                    parents_by_child.insert(i, (j, k));
                }
            }
        }
    }
    parents_by_child
}
