use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use memoize::memoize;

fn main() {
    //p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q07_p1a.txt").unwrap();
    let (names, rules) = names_and_rules(&input);
    let filtered = filter_names(&names, &rules);
    println!("Part1: {}", filtered.last().unwrap());

    //p2
    let input = std::fs::read_to_string("everybody_codes_e2025_q07_p2a.txt").unwrap();
    let (names, rules) = names_and_rules(&input);
    let filtered = names.iter().enumerate().filter_map(|(idx, n)| check_name(&rules, n).then(|| idx + 1)).sum::<usize>();
    println!("Part2: {:?}", filtered);

    let input = std::fs::read_to_string("everybody_codes_e2025_q07_p3a.txt").unwrap();
    let (names, rules) = names_and_rules(&input);
    let prefixes = filter_names(&names, &rules);
    let p3 = prefixes
        .iter()
        .filter(|p| prefixes.iter().all(|q| *p == q || !p.starts_with(q)))
        .map(|p| gen_names(*p.as_bytes().last().unwrap(), p.len(), &rules))
        .sum::<usize>();
    println!("Part3: {:?}", p3);
}

fn check_name(rules: &HashMap<u8, HashSet<u8>>, n: &str) -> bool {
    n.as_bytes().iter().tuple_windows().all(|(c, n)| rules.get(c).unwrap().contains(n))
}

fn filter_names(names: &Vec<String>, rules: &HashMap<u8, HashSet<u8>>) -> Vec<String> {
    names.iter().filter_map(|n| check_name(&rules, n).then(|| n.to_string())).collect::<Vec<_>>()
}

fn names_and_rules(input: &str) -> (Vec<String>, HashMap<u8, HashSet<u8>>) {
    let (names, rules) = input.split_once("\n\n").unwrap();
    let names = names.trim().split(",").map(|s| s.to_string()).collect::<Vec<_>>();
    let rules = rules.trim().lines().fold(HashMap::new(), |mut acc, l| {
        let (c, f) = l.split_once(" > ").unwrap();
        let f: HashSet<u8> = f.split(",").map(|s| *s.trim().as_bytes().first().unwrap()).collect();
        acc.insert(*c.as_bytes().first().unwrap(), f);
        acc
    });
    (names, rules)
}

#[memoize(Ignore: rules)]
fn gen_names(letter: u8, size: usize, rules: &HashMap<u8, HashSet<u8>>) -> usize {
    if size == 11 {
        return 1;
    }
    let count = if size < 7 { 0 } else { 1 };
    if let Some(followers) = rules.get(&letter) {
        count + followers.iter().map(|f| gen_names(*f, size + 1, rules)).sum::<usize>()
    } else {
        count
    }
}
