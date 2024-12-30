//use std::collections::HashMap;
use std::{collections::HashSet, iter::*};

//use itertools::{Either, Itertools};

fn main() {
    let input = std::fs::read_to_string("everybody_codes_e2024_q02_p1.txt").unwrap();
    let (word_input, text) = input.split_at(input.find('\n').unwrap());
    let words: Vec<&str> = word_input.split_once(':').unwrap().1.split(",").collect();
    let p1 = words.iter().map(|w| text.matches(w).count()).sum::<usize>();
    println!("Part1: {}", p1);

    let (words, text) = parse("everybody_codes_e2024_q02_p2.txt");
    let p2 = text
        .lines()
        .map(|line| {
            let mut found_indices: HashSet<usize> = HashSet::new();
            words.iter().for_each(|w| {
                let mut start_idx = 0;
                while let Some(idx) = line[start_idx..].find(w) {
                    (start_idx + idx..start_idx + idx + w.len()).for_each(|i| {
                        found_indices.insert(i);
                    });
                    start_idx += idx + 1;
                }
            });
            found_indices.len()
        })
        .sum::<usize>();
    println!("Part2: {}", p2);

    // part3
    let (words, text) = parse("everybody_codes_e2024_q02_p3.txt");
    let text_grid = text
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut found_indices: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..text_grid.len() {
        for x in 0..text_grid[y].len() {
            for word in &words {
                if text_grid[y][x] != word.chars().next().unwrap() {
                    continue;
                }
                for d in [(1, 0), (0, 1)].iter() {
                    let mut found = true;
                    for i in 1..word.len() {
                        let (dx, dy) = (d.0 * i as i32, d.1 * i as i32);
                        let cx = (x as i32 + dx) % text_grid[y].len() as i32;
                        let cy = y as i32 + dy;
                        if cx < 0 || cy < 0 || cy >= text_grid.len() as i32 {
                            found = false;
                            break;
                        }

                        if text_grid[cy as usize][cx as usize] != word.chars().nth(i).unwrap() {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        for i in 0..word.len() {
                            let (dx, dy) = (d.0 * i as i32, d.1 * i as i32);
                            let cx = (x as i32 + dx) % text_grid[y].len() as i32;
                            found_indices.insert((cx as usize, (y as i32 + dy) as usize));
                        }
                    }
                }
            }
        }
    }
    println!("Part3: {}", found_indices.len());
}

fn parse(input: &str) -> (Vec<String>, String) {
    let input = std::fs::read_to_string(input).unwrap();
    let (word_input, text) = input.split_once("\n\n").unwrap();
    let words: Vec<String> = word_input[6..]
        .split(",")
        .flat_map(|s| [s.to_owned(), s.chars().rev().collect::<String>()].into_iter())
        .collect::<Vec<String>>();
    (words, text.to_owned())
}
