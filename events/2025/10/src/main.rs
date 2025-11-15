use memoize::memoize;
use std::collections::{BTreeSet, HashSet};

fn main() {
    //p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q10_p1.txt").unwrap();
    let sheep_pos = input.lines().enumerate().fold(HashSet::new(), |mut acc, (y, l)| {
        l.as_bytes().iter().enumerate().for_each(|(x, &b)| {
            if b == b'S' {
                acc.insert((x as i64, y as i64));
            }
        });
        acc
    });
    println!("sheep_pos: {:?}", sheep_pos.len());
    let d_offsets = [(-2, -1), (-1, -2), (1, -2), (2, -1), (2, 1), (1, 2), (-1, 2), (-2, 1)];
    let move_dragon = |new_pos: &mut HashSet<(i64, i64)>, p: (i64, i64), size: i64| {
        new_pos.insert(p);
        for off in d_offsets {
            if p.0 + off.0 < size && p.0 + off.0 >= 0 && p.1 + off.1 < size && p.1 + off.1 >= 0 {
                new_pos.insert((p.0 + off.0, p.1 + off.1));
            }
        }
    };
    let mut dragon_pos = HashSet::new();
    let size = 21;
    dragon_pos.insert((size / 2, size / 2));
    for _ in 1..=4 {
        let mut new_pos = HashSet::new();
        dragon_pos.into_iter().for_each(|p| move_dragon(&mut new_pos, p, size));
        dragon_pos = new_pos;
        // println!("dragon_pos: {:?}", dragon_pos);
    }
    let p1 = dragon_pos.iter().filter(|p| sheep_pos.contains(p)).count();
    println!("p1: {:?}", p1);

    //p2
    let input = std::fs::read_to_string("everybody_codes_e2025_q10_p2.txt").unwrap();
    let mut hideouts = HashSet::new();
    let mut sheep_pos = input.lines().enumerate().fold(HashSet::new(), |mut acc, (y, l)| {
        l.as_bytes().iter().enumerate().for_each(|(x, &b)| {
            if b == b'S' {
                acc.insert((x as i64, y as i64));
            }
            if b == b'#' {
                hideouts.insert((x as i64, y as i64));
            }
        });
        acc
    });
    let move_dragon = |new_pos: &mut HashSet<(i64, i64)>, p: (i64, i64), size: i64| {
        for off in [(-2, -1), (-1, -2), (1, -2), (2, -1), (2, 1), (1, 2), (-1, 2), (-2, 1)] {
            if p.0 + off.0 < size && p.0 + off.0 >= 0 && p.1 + off.1 < size && p.1 + off.1 >= 0 {
                new_pos.insert((p.0 + off.0, p.1 + off.1));
            }
        }
    };
    let size = 101;
    let mut dragon_pos = HashSet::new();
    dragon_pos.insert((size / 2, size / 2));
    let mut eaten = 0;
    for _ in 1..=20 {
        let mut new_pos = HashSet::new();
        dragon_pos.into_iter().for_each(|p| move_dragon(&mut new_pos, p, size));
        dragon_pos = new_pos;
        let to_rm = dragon_pos.iter().filter(|p| sheep_pos.contains(p) && !hideouts.contains(p)).collect::<Vec<_>>();
        eaten += to_rm.len();
        to_rm.into_iter().for_each(|p| {
            sheep_pos.remove(&p);
        });
        let mut new_pos = HashSet::new();
        sheep_pos.iter().for_each(|p| {
            let p_new = (p.0, p.1 + 1);
            if p_new.1 < size {
                if !dragon_pos.contains(&p_new) || hideouts.contains(&p_new) {
                    new_pos.insert(p_new);
                } else {
                    eaten += 1;
                }
            }
        });
        sheep_pos = new_pos;
    }
    println!("p2: {:?}", eaten);

    // p3
    let input = std::fs::read_to_string("everybody_codes_e2025_q10_p3.txt").unwrap();
    let mut hideouts = HashSet::new();
    let mut dragon_pos = (0, 0);
    let sheep_pos = input.lines().enumerate().fold(BTreeSet::new(), |mut acc, (y, l)| {
        l.as_bytes().iter().enumerate().for_each(|(x, &b)| {
            if b == b'S' {
                acc.insert((x as i64, y as i64));
            }
            if b == b'#' {
                hideouts.insert((x as i64, y as i64));
            }
            if b == b'D' {
                dragon_pos = (x as i64, y as i64);
            }
        });
        acc
    });

    let size = (7, 6);
    let n = dfs(sheep_pos, dragon_pos, &hideouts, size, true);
    println!("p3: {}", n);
}

#[memoize(Ignore: hideouts, Ignore: size)]
fn dfs(
    sheep_pos: BTreeSet<(i64, i64)>,
    dragon_pos: (i64, i64),
    hideouts: &HashSet<(i64, i64)>,
    size: (i64, i64),
    sheep_turn: bool,
) -> usize {
    if sheep_pos.is_empty() {
        return 1; // all sheep escaped, but not all eaten
    }

    let mut sum = 0;
    let mut dragon_turn = true;
    if sheep_turn {
        for s in &mut sheep_pos.iter() {
            let p_new = (s.0, s.1 + 1);
            if dragon_pos != p_new || hideouts.contains(&p_new) {
                dragon_turn = false;
                let mut sp_new = sheep_pos.clone();
                sp_new.remove(s);
                if p_new.1 >= size.1 {
                    continue; // not possible to eat all anymore!
                }
                sp_new.insert(p_new);
                sum += dfs(sp_new, dragon_pos, hideouts, size, false);
            }
        }
    }
    if dragon_turn {
        let p = dragon_pos;
        for off in [(-2, -1), (-1, -2), (1, -2), (2, -1), (2, 1), (1, 2), (-1, 2), (-2, 1)] {
            let p_new = (p.0 + off.0, p.1 + off.1);
            if p_new.0 < size.0 && p_new.0 >= 0 && p_new.1 < size.1 && p_new.1 >= 0 {
                if sheep_pos.contains(&p_new) && !hideouts.contains(&p_new) {
                    let mut sp_new = sheep_pos.clone();
                    sp_new.remove(&p_new);
                    sum += dfs(sp_new, p_new, hideouts, size, true);
                } else {
                    sum += dfs(sheep_pos.clone(), p_new, hideouts, size, true);
                }
            }
        }
    }
    sum
}
