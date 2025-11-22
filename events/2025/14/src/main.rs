use std::collections::HashSet;

use is_odd::IsOdd;
use itertools::Itertools;
use memoize::memoize;

fn main() {
    //p1/p2
    let input = std::fs::read_to_string("everybody_codes_e2025_q14_p1.txt").unwrap();
    let rounds = 10;
    let p1 = solve1_and_2(input, rounds);
    println!("Part 2: {}", p1);

    let input = std::fs::read_to_string("everybody_codes_e2025_q14_p2.txt").unwrap();
    let rounds = 2025;
    let p2 = solve1_and_2(input, rounds);
    println!("Part 2: {}", p2);

    let input = std::fs::read_to_string("everybody_codes_e2025_q14_p3.txt").unwrap();
    let (width, height) = (34, 34);
    let (pat_width, pat_height) = (8, 8);
    let (pat_x, pat_y) = (width / 2 - 4, height / 2 - 4);
    let pat_tiles = input.trim().lines().enumerate().fold(HashSet::new(), |mut acc, (y, l)| {
        l.as_bytes().iter().enumerate().for_each(|(x, b)| {
            if *b == b'#' {
                acc.insert((pat_x + x as i64, pat_y + y as i64));
            }
        });
        acc
    });

    let mut tiles = HashSet::new();
    let mut last_same = 0;
    for round in 1..=5000 {
        let mut new_tiles = HashSet::new();
        for y in 0..height {
            for x in 0..width {
                let odd = [(-1, -1), (-1, 1), (1, 1), (1, -1)]
                    .iter()
                    .filter(|(dx, dy)| {
                        let diag = (x as i64 + dx, y as i64 + dy);
                        tiles.contains(&diag)
                    })
                    .count()
                    .is_odd();
                let active = tiles.contains(&(x as i64, y as i64));
                if (active && odd) || (!active && !odd) {
                    new_tiles.insert((x as i64, y as i64));
                }
            }
        }
        let mut same = true;
        for y in pat_y..(pat_y + pat_height) {
            let r = (pat_x..(pat_x + pat_width))
                .all(|x| pat_tiles.contains(&(x as i64, y as i64)) == new_tiles.contains(&(x as i64, y as i64)));
            if !r {
                same = false;
                break;
            }
        }
        if same {
            let diff = round - last_same;
            println!("same in round: {}, diff: {}, active tiles: {}", round, diff, new_tiles.len());
            last_same = round;
        }
        tiles = new_tiles;
    }
    // to lazy to code the pattern check, simply print out and do calc by hand
    // let pat_r = 892u64 + 3203;
    // let bla = 1000000000u64 - 125;
    // let rem = bla % pat_r;
    // let bla = bla / pat_r;
    // println!("rem: {}", rem);
    // let act = 588u64 + 552u64;
    // println!("bla: {}", bla * act + 552);
    let pat_r = 1518u64 + 97 + 150 + 1066 + 9 + 457 + 798;
    let rel_rounds = 1000000000u64 - 319u64;
    let _rem = rel_rounds % pat_r;
    let full_repeats = rel_rounds / pat_r;
    let act = 644u64 + 496 + 660 + 604 + 516 + 504 + 556;
    println!("Part3: {}", full_repeats * act + 556);
}

fn solve1_and_2(input: String, rounds: i32) -> usize {
    let mut tiles = input.trim().lines().enumerate().fold(HashSet::new(), |mut acc, (y, l)| {
        l.as_bytes().iter().enumerate().for_each(|(x, b)| {
            if *b == b'#' {
                acc.insert((x as i64, y as i64));
            }
        });
        acc
    });
    let (width, height) = (
        input.trim().lines().nth(0).unwrap().len() as i64,
        input.trim().lines().count() as i64,
    );
    // print_tiles(&tiles, width, height);
    let mut sum_active = 0;
    for _ in 1..=rounds {
        let mut new_tiles = HashSet::new();
        for y in 0..height {
            for x in 0..width {
                let odd = [(-1, -1), (-1, 1), (1, 1), (1, -1)]
                    .iter()
                    .filter(|(dx, dy)| {
                        let diag = (x as i64 + dx, y as i64 + dy);
                        tiles.contains(&diag)
                    })
                    .count()
                    .is_odd();
                let active = tiles.contains(&(x as i64, y as i64));
                // println!("x: {}, y: {}, active: {}, odd: {}", x, y, active, odd);
                if (active && odd) || (!active && !odd) {
                    new_tiles.insert((x as i64, y as i64));
                }
            }
        }
        sum_active += new_tiles.len();
        tiles = new_tiles;
    }
    sum_active
}

#[allow(dead_code)]
fn print_tiles(tiles: &HashSet<(i64, i64)>, width: i64, height: i64) {
    for y in 0..height {
        for x in 0..width {
            print!("{}", if tiles.contains(&(x as i64, y as i64)) { '#' } else { '.' });
        }
        println!();
    }
}
