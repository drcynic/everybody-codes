use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    //p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q07_p1a.txt").unwrap();
    let (names, rules) = names_and_rules(&input);
    let filtered = filter_names(&names, &rules);
    println!("Part1: {}", filtered.last().unwrap());

    //p2
    let input = std::fs::read_to_string("everybody_codes_e2025_q07_p2a.txt").unwrap();
    let (names, rules) = names_and_rules(&input);
    let filtered = names
        .iter()
        .enumerate()
        .filter(|(_, n)| n.as_bytes().iter().tuple_windows().all(|(c, n)| rules.get(c).unwrap().contains(n)))
        .map(|(idx, _)| idx + 1)
        .sum::<usize>();
    println!("Part2: {:?}", filtered);

    let input = std::fs::read_to_string("everybody_codes_e2025_q07_p3a.txt").unwrap();
    let (names, rules) = names_and_rules(&input);
    let prefixes = filter_names(&names, &rules);
    let mut unique_names = HashSet::new();
    for p in &prefixes {
        gen_names(p, &rules, &mut unique_names);
    }
    println!("Part3: {:?}", unique_names.len());
}

fn filter_names(names: &Vec<String>, rules: &HashMap<u8, HashSet<u8>>) -> Vec<String> {
    let filtered = names
        .iter()
        .filter(|n| n.as_bytes().iter().tuple_windows().all(|(c, n)| rules.get(c).unwrap().contains(n)))
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    filtered
}

fn names_and_rules(input: &str) -> (Vec<String>, HashMap<u8, HashSet<u8>>) {
    let (names, rules) = input.split_once("\n\n").unwrap();
    let names = names.trim().split(",").map(|s| s.to_string()).collect::<Vec<_>>();
    let rules = rules.trim().lines().fold(HashMap::new(), |mut acc, l| {
        let (c, f) = l.split_once(" > ").unwrap();
        let f = f.split(",").map(|s| *s.trim().as_bytes().first().unwrap()).collect::<Vec<_>>();
        let f: HashSet<u8> = HashSet::from_iter(f);
        acc.insert(*c.as_bytes().first().unwrap(), f);
        acc
    });
    (names, rules)
}

fn gen_names(prefix: &str, rules: &HashMap<u8, HashSet<u8>>, unique_names: &mut HashSet<String>) {
    let mut stack = vec![prefix.to_string()];

    while let Some(name) = stack.pop() {
        let last_char = name.as_bytes().last().unwrap();
        if let Some(possibilities) = rules.get(last_char) {
            for possibility in possibilities {
                let new_name = format!("{}{}", name, *possibility as char);
                if new_name.len() <= 11 {
                    stack.push(new_name.clone());
                    if new_name.len() >= 7 {
                        unique_names.insert(new_name);
                    }
                }
            }
        }
    }
}
