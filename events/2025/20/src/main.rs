use std::{
    cmp,
    collections::{BinaryHeap, HashSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    pos: (usize, usize),
    rot: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse order for min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn new(cost: usize, pos: (usize, usize), rot: usize) -> Self {
        Self { cost, pos, rot }
    }
}

fn main() {
    // p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q20_p1.txt").unwrap();
    let (_, _, _, trampolines) = parse(input);
    let p1 = trampolines
        .iter()
        .map(|(tx, ty)| {
            let n = if trampolines.contains(&(tx + 1, *ty)) { 1 } else { 0 };
            let u1 = if *ty > 0 && ty % 2 == 0 && tx % 2 == 0 && trampolines.contains(&(*tx, ty.saturating_sub(1))) { 1 } else { 0 };
            let u2 = if *ty > 0 && ty % 2 == 1 && tx % 2 == 1 && trampolines.contains(&(*tx, ty.saturating_sub(1))) { 1 } else { 0 };
            n + u1 + u2
        })
        .sum::<usize>();
    println!("p1: {}", p1);

    let input = std::fs::read_to_string("everybody_codes_e2025_q20_p2.txt").unwrap();
    let (start, end, width, trampolines) = parse(input);
    let p2 = bfs(start, end, width, 0, trampolines);
    println!("p2: {}", p2);

    // p3
    let input = std::fs::read_to_string("everybody_codes_e2025_q20_p3.txt").unwrap();
    let (start, end, width, trampolines) = parse(input);
    let p3 = bfs(start, end, width, 1, trampolines);
    println!("p3: {}", p3);
}

fn bfs(start: (usize, usize), end: (usize, usize), width: usize, rot_add: usize, trampolines_orig: HashSet<(usize, usize)>) -> usize {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(State::new(0, start, rot_add));

    while let Some(State { cost, pos, rot }) = heap.pop() {
        if pos == end {
            return cost;
        }

        if !visited.insert((pos, rot % 3)) {
            continue;
        }

        let mut trampolines = trampolines_orig.clone();
        for _ in 0..(rot % 3) {
            trampolines = rotate(width, &trampolines);
        }

        // Try moving to next points in all 4 directions and same position!
        // same
        if trampolines.contains(&(pos.0, pos.1)) {
            heap.push(State::new(cost + 1, (pos.0, pos.1), rot + rot_add));
        }
        // r
        if trampolines.contains(&(pos.0 + 1, pos.1)) {
            heap.push(State::new(cost + 1, (pos.0 + 1, pos.1), rot + rot_add));
        }
        // l
        if trampolines.contains(&(pos.0.saturating_sub(1), pos.1)) {
            heap.push(State::new(cost + 1, (pos.0.saturating_sub(1), pos.1), rot + rot_add));
        }
        // up
        if pos.1 % 2 == 0 && pos.0 % 2 == 0 && trampolines.contains(&(pos.0, pos.1.saturating_sub(1)))
            || pos.1 % 2 == 1 && pos.0 % 2 == 1 && trampolines.contains(&(pos.0, pos.1.saturating_sub(1)))
        {
            heap.push(State::new(cost + 1, (pos.0, pos.1.saturating_sub(1)), rot + rot_add));
        }
        // down
        if pos.1 % 2 == 0 && pos.0 % 2 == 1 && trampolines.contains(&(pos.0, pos.1 + 1))
            || pos.1 % 2 == 1 && pos.0 % 2 == 0 && trampolines.contains(&(pos.0, pos.1 + 1))
        {
            heap.push(State::new(cost + 1, (pos.0, pos.1 + 1), rot + rot_add));
        }
    }

    unreachable!()
}

#[allow(dead_code)]
fn print_trampolines(width: usize, trampolines: &HashSet<(usize, usize)>) {
    for y in 0..=(width / 2) {
        for x in 0..width {
            if trampolines.contains(&(x, y)) {
                print!("T");
            } else if x >= y && x < width - y {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn rotate(width: usize, trampolines: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let new_trampolines = trampolines
        .iter()
        .map(|&(tx, ty)| {
            let off_from_first = (tx - ty) / 2;
            let sub = if ty % 2 == 0 { tx % 2 } else { (tx + 1) % 2 };
            let nx = width - 1 - off_from_first - 2 * ty - sub;
            let ny = off_from_first;
            (nx, ny)
        })
        .collect::<HashSet<_>>();
    new_trampolines
}

fn parse(input: String) -> ((usize, usize), (usize, usize), usize, HashSet<(usize, usize)>) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut width = 0;
    let mut height = 0;
    let trampolines_orig = input.lines().enumerate().fold(HashSet::new(), |mut acc, (y, l)| {
        l.trim().as_bytes().iter().enumerate().for_each(|(x, b)| {
            if *b == b'T' {
                acc.insert((x, y));
            } else if *b == b'S' {
                start = (x, y);
                acc.insert((x, y));
            } else if *b == b'E' {
                end = (x, y);
                acc.insert((x, y));
            }
            width = cmp::max(width, x + 1);
        });
        height = cmp::max(height, y + 1);
        acc
    });
    (start, end, width, trampolines_orig)
}
