use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("everybody_codes_e2024_q06_p1a.txt").unwrap();
    let paths_by_length = parse_input(input);
    let r1 = paths_by_length
        .values()
        .find(|v| v.len() == 1)
        .unwrap()
        .first()
        .unwrap()
        .iter()
        .join("");
    println!("Part1: {:?}", r1);

    let input = std::fs::read_to_string("everybody_codes_e2024_q06_p2.txt").unwrap();
    let paths_by_length = parse_input(input);
    let r2 = first_letter_path(paths_by_length);
    println!("Part2: {:?}", r2);

    let input = std::fs::read_to_string("everybody_codes_e2024_q06_p3.txt").unwrap();
    let paths_by_length = parse_input(input);
    let r3 = first_letter_path(paths_by_length);
    println!("Part3: {:?}", r3);
}

fn first_letter_path(paths_by_length: HashMap<usize, Vec<VecDeque<String>>>) -> String {
    paths_by_length
        .values()
        .find(|v| v.len() == 1)
        .unwrap()
        .first()
        .unwrap()
        .iter()
        .map(|s| s[..1].to_string())
        .join("")
}

fn parse_input(input: String) -> HashMap<usize, Vec<VecDeque<String>>> {
    let (leaf_parents, parent_by_child) =
        input
            .trim()
            .lines()
            .fold((Vec::new(), HashMap::new()), |(mut leaf_parents, mut parent_by_child), l| {
                let (parent_id, r) = l.split_once(':').unwrap();
                if parent_id != "ANT" && parent_id != "BUG" {
                    r.split(",").filter(|s| *s != "ANT" && *s != "BUG").for_each(|c| {
                        if c == "@" {
                            leaf_parents.push(parent_id.to_string());
                        } else {
                            parent_by_child.insert(c.to_string(), parent_id.to_string());
                        }
                    });
                }
                (leaf_parents, parent_by_child)
            });

    leaf_parents.iter().fold(HashMap::new(), |mut acc, n| {
        let mut path: VecDeque<String> = [n.clone(), "@".to_string()].try_into().unwrap();
        let mut cur_id = n.clone();
        while let Some(parent_id) = parent_by_child.get(&cur_id) {
            path.push_front(parent_id.clone());
            cur_id = parent_id.to_string();
        }
        acc.entry(path.len()).or_insert(Vec::new()).push(path);
        acc
    })
}
