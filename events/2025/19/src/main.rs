use std::collections::{BTreeMap, HashMap};

use memoize::memoize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Passage {
    x: isize,
    y: isize,
    l: isize,
}

fn main() {
    // p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q19_p1.txt").unwrap();
    let (passages, end, start) = parse(input);
    let flaps = bfs(passages, end, start);
    println!("Part 1: {:?}", flaps);

    // p2
    let input = std::fs::read_to_string("everybody_codes_e2025_q19_p2.txt").unwrap();
    let (passages, end, start) = parse(input);
    let flaps = bfs(passages, end, start);
    println!("Part 2: {:?}", flaps);

    // p3
    // did not get bfs working with jump positions in time, switched to dfs with memoization instead
    let input = std::fs::read_to_string("everybody_codes_e2025_q19_p3.txt").unwrap();
    let passages = passages_by_x(input);
    let flaps = dfs(&passages, (0, 0), 0, 0);
    println!("Part 3: {:?}", flaps);
}

fn bfs(passages: Vec<Passage>, end: (isize, isize), start: (isize, isize)) -> i32 {
    // dijkstra
    let mut distances = HashMap::new();
    distances.insert(start, 0);
    let mut queue = vec![start];
    let mut flaps = 10000;
    while let Some(current) = queue.pop() {
        if current.0 == end.0 {
            flaps = distances[&current];
            break;
        }
        let current_distance = distances[&current];
        for neighbor in [(current.0 + 1, current.1 + 1), (current.0 + 1, current.1 - 1)] {
            if !distances.contains_key(&neighbor) && can_move(neighbor, &passages) {
                let cost = if neighbor.1 > current.1 { 1 } else { 0 };
                distances.insert(neighbor, current_distance + cost);
                queue.push(neighbor);
            }
        }
    }
    flaps
}

fn parse(input: String) -> (Vec<Passage>, (isize, isize), (isize, isize)) {
    let passages = input
        .lines()
        .map(|l| {
            let s = l.split(",").map(|v| v.parse::<isize>().unwrap()).collect::<Vec<_>>();
            Passage { x: s[0], y: s[1], l: s[2] }
        })
        .collect::<Vec<_>>();
    let last_pas = passages.last().unwrap();
    let end = (last_pas.x, last_pas.y);
    let start = (0, 0);
    (passages, end, start)
}

fn passages_by_x(input: String) -> Vec<Vec<(usize, usize)>> {
    let (passages, _, _) = parse(input);
    let mut map = BTreeMap::new();
    for pas in &passages {
        map.entry(pas.x as usize).or_insert_with(Vec::new).push((pas.x as usize, pas.y as usize));
    }
    let passages: Vec<_> = map.into_values().collect();
    passages
}

fn can_move(next: (isize, isize), passages: &[Passage]) -> bool {
    if next.1 < 0 {
        return false;
    }

    passages.iter().filter(|p| next.0 == p.x).count() == 0
        || passages.iter().filter(|p| next.0 == p.x).any(|p| next.1 >= p.y && next.1 < p.y + p.l)
}

#[memoize(Ignore: passages)]
fn dfs(passages: &[Vec<(usize, usize)>], pos: (usize, usize), flaps: usize, idx: usize) -> usize {
    if idx == passages.len() {
        return flaps;
    }

    passages[idx]
        .iter()
        .map(|&(x1, y1)| {
            let dx = x1 - pos.0;
            let dy = y1.abs_diff(pos.1);
            let straight = y1.saturating_sub(pos.1) + dx.saturating_sub(dy).div_ceil(2);
            let next_pos = (pos.0 + dx, pos.1 + 2 * straight - dx);
            dfs(passages, next_pos, flaps + straight, idx + 1)
        })
        .min()
        .unwrap()
}
