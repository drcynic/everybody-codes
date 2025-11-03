use std::{iter::*, ops::Add};

fn main() {
    //p1
    let (names, cmds) = read("everybody_codes_e2025_q01_p1.txt");
    let max_idx = (names.len() - 1) as i32;
    let idx = cmds.iter().fold(0, |idx, cmd| idx.add(cmd).clamp(0, max_idx));
    println!("Part1: {}", names[idx as usize]);

    let (names, cmds) = read("everybody_codes_e2025_q01_p2.txt");
    let len = names.len() as i32;
    let idx = cmds.iter().sum::<i32>().rem_euclid(len);
    println!("Part2: {}", names[idx as usize]);

    //p3
    let (mut names, cmds) = read("everybody_codes_e2025_q01_p3.txt");
    let len = names.len() as i32;
    cmds.iter().for_each(|cmd| {
        names.swap(0, cmd.rem_euclid(len) as usize);
    });
    println!("Part3: {}", names[0]);
}

fn read(filename: &str) -> (Vec<String>, Vec<i32>) {
    let input = std::fs::read_to_string(filename).unwrap();
    let (names, cmds) = input.split_once("\n\n").unwrap();
    let names = names.trim().split(',').map(|s| s.to_string()).collect::<Vec<_>>();
    let cmds = cmds.trim().split(',').map(|s| s[1..].parse::<i32>().unwrap() * if &s[0..1] == "R" { 1 } else { -1 }).collect::<Vec<_>>();
    (names, cmds)
}
