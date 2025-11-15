use memoize::memoize;
use std::collections::{BTreeSet, HashSet, hash_set::IntoIter};

fn main() {
    //p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q10_p1.txt").unwrap();
    let (mut sheep_pos, _, dragon, size) = parse(&input);
    let mut dragon_pos: HashSet<(i64, i64)> = HashSet::from_iter([dragon]);
    let sheep_start = sheep_pos.len();
    for _ in 1..=4 {
        dragon_pos = dragon_pos.into_iter().flat_map(|p| move_dragon(p, size.0)).collect();
        dragon_pos.iter().for_each(|p| {
            sheep_pos.remove(&p);
        });
    }
    println!("p1: {:?}", sheep_start - sheep_pos.len());

    //p2
    let input = std::fs::read_to_string("everybody_codes_e2025_q10_p2.txt").unwrap();
    let (mut sheep_pos, hideouts, dragon, size) = parse(&input);
    let mut dragon_pos: BTreeSet<(i64, i64)> = BTreeSet::from_iter([dragon]);
    let mut eaten = 0;
    for _ in 1..=20 {
        dragon_pos = dragon_pos.into_iter().flat_map(|p| move_dragon(p, size.0)).collect();
        dragon_pos.iter().for_each(|p| {
            if sheep_pos.contains(&p) && !hideouts.contains(&p) {
                sheep_pos.remove(&p);
                eaten += 1;
            }
        });
        sheep_pos = sheep_pos
            .into_iter()
            .map(|p| (p.0, p.1 + 1))
            .filter(|p| p.1 < size.0) //escaped
            .filter(|p| {
                if !dragon_pos.contains(&p) || hideouts.contains(&p) {
                    true
                } else {
                    eaten += 1;
                    false
                }
            })
            .collect();
    }
    println!("p2: {:?}", eaten);

    // p3
    let input = std::fs::read_to_string("everybody_codes_e2025_q10_p3.txt").unwrap();
    let (sheep_pos, hideouts, dragon, size) = parse(&input);
    let n = dfs(sheep_pos, dragon, &hideouts, size, true);
    println!("p3: {}", n);
}

fn parse(input: &str) -> (BTreeSet<(i64, i64)>, BTreeSet<(i64, i64)>, (i64, i64), (i64, i64)) {
    let mut sheep_pos = BTreeSet::new();
    let mut hideouts = BTreeSet::new();
    let mut dragon = None;
    let mut size = (0, 0);
    input.lines().enumerate().for_each(|(y, l)| {
        size.1 = (y + 1) as i64;
        l.as_bytes().iter().enumerate().for_each(|(x, &b)| {
            if b == b'S' {
                sheep_pos.insert((x as i64, y as i64));
            }
            if b == b'#' {
                hideouts.insert((x as i64, y as i64));
            }
            if b == b'D' {
                dragon = Some((x as i64, y as i64));
            }
            size.0 = (x + 1) as i64;
        });
    });

    (sheep_pos, hideouts, dragon.unwrap(), size)
}

fn move_dragon(p: (i64, i64), size: i64) -> IntoIter<(i64, i64)> {
    let mut new_pos = HashSet::new();
    for off in [(-2, -1), (-1, -2), (1, -2), (2, -1), (2, 1), (1, 2), (-1, 2), (-2, 1)] {
        if p.0 + off.0 < size && p.0 + off.0 >= 0 && p.1 + off.1 < size && p.1 + off.1 >= 0 {
            new_pos.insert((p.0 + off.0, p.1 + off.1));
        }
    }
    new_pos.into_iter()
}

#[memoize(Ignore: hideouts, Ignore: size)]
fn dfs(
    sheep_pos: BTreeSet<(i64, i64)>,
    dragon_pos: (i64, i64),
    hideouts: &BTreeSet<(i64, i64)>,
    size: (i64, i64),
    sheep_turn: bool,
) -> usize {
    if sheep_pos.is_empty() {
        return 1; // all eaten
    }

    let mut sum = 0;
    let mut dragon_turn = true;
    if sheep_turn {
        for s in &mut sheep_pos.iter() {
            let p_new = (s.0, s.1 + 1);
            if dragon_pos != p_new || hideouts.contains(&p_new) {
                dragon_turn = false;
                if p_new.1 >= size.1 {
                    continue; // not possible to eat all anymore!
                }
                let mut sp_new = sheep_pos.clone();
                sp_new.remove(s);
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
