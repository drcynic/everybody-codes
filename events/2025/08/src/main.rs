use std::cmp::{max, min};

use itertools::Itertools;

fn main() {
    //p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q08_p1.txt").unwrap();
    let pos = input.trim().split(",").map(|p| p.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let nails = 32;
    let p1 = pos.iter().tuple_windows().filter(|(l, r)| (*l - *r).abs() == (nails / 2)).count();
    println!("Part1: {}", p1);

    // //2
    let input = std::fs::read_to_string("everybody_codes_e2025_q08_p2.txt").unwrap();
    let pos = input.trim().split(",").map(|p| p.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let pairs = pos.iter().tuple_windows().map(|(l, r)| (min(l, r), max(l, r))).collect::<Vec<_>>();
    let p2 = pairs
        .iter()
        .enumerate()
        .map(|(idx, (l, r))| {
            let mut sum = 0;
            for i in 0..idx {
                let (pl, pr) = pairs[i];
                if (pl < l && pr > l && pr < r) || (pl > l && pl < r && pr > r) {
                    sum += 1;
                }
            }
            sum
        })
        .sum::<i32>();
    println!("pairs: {:?}", p2);

    //3
    let input = std::fs::read_to_string("everybody_codes_e2025_q08_p3.txt").unwrap();
    let pos = input.trim().split(",").map(|p| p.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let pairs = pos.iter().tuple_windows().map(|(l, r)| (min(l, r), max(l, r))).collect::<Vec<_>>();
    let mut p3 = 0;
    let nails = 256;
    for nl in 1..=nails {
        for nr in 1..=nails / 2 {
            if nl == nr {
                continue;
            }
            let l = min(nr, nl);
            let r = max(nr, nl);
            let mut sum = 0;
            for i in 0..pairs.len() {
                let (pl, pr) = pairs[i];
                if (*pl < l && *pr > l && *pr < r) || (*pl > l && *pl < r && *pr > r) || (l == *pl && r == *pr) {
                    sum += 1;
                }
            }
            p3 = max(p3, sum);
        }
    }
    println!("pairs: {:?}", p3);
}
