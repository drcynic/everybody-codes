use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
};

fn main() {
    // p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q17_p1.txt").unwrap();
    let nums = input.trim().lines().fold(Vec::new(), |mut acc_y, l| {
        let acc_x = l.trim().as_bytes().iter().fold(Vec::new(), |mut acc_x, &b| {
            acc_x.push((b - b'0') as isize);
            acc_x
        });
        acc_y.push(acc_x);
        acc_y
    });
    let (cx, cy) = (nums[0].len() as isize / 2, nums.len() as isize / 2);
    let nums = input.trim().lines().enumerate().fold(-((b'@' - b'0') as isize), |acc, (y, l)| {
        let sum_x = l.trim().as_bytes().iter().enumerate().fold(0, |mut acc_x, (x, &b)| {
            let (dx, dy) = (x as isize - cx, y as isize - cy);
            if dx * dx + dy * dy <= 10 * 10 {
                acc_x += (b - b'0') as isize;
            }
            acc_x
        });
        acc + sum_x
    });
    println!("Part 1: {:?}", nums);

    // // p2
    let input = std::fs::read_to_string("everybody_codes_e2025_q17_p2.txt").unwrap();
    let nums = input.trim().lines().fold(Vec::new(), |mut acc_y, l| {
        let acc_x = l.trim().as_bytes().iter().fold(Vec::new(), |mut acc_x, &b| {
            if b == b'@' {
                acc_x.push(0);
            } else {
                acc_x.push((b - b'0') as isize);
            }
            acc_x
        });
        acc_y.push(acc_x);
        acc_y
    });
    let (cx, cy) = (nums[0].len() as isize / 2, nums.len() as isize / 2);

    let mut max_sum = 0;
    let mut max_step = 0;
    let mut last = 0;
    for step in 1..=cx {
        let mut sum = 0;
        for y in 0..nums.len() {
            for x in 0..nums[y].len() {
                let (dx, dy) = (x as isize - cx, y as isize - cy);
                if dx * dx + dy * dy <= step * step {
                    sum += nums[y][x];
                }
            }
        }
        let tmp = last;
        last = sum;
        sum -= tmp;
        // println!("step: {:?}, sum: {sum}", step);
        if sum > max_sum {
            max_sum = sum;
            max_step = step;
        }
    }
    println!("Part 2: {:?}", max_sum * max_step);

    let input = std::fs::read_to_string("everybody_codes_e2025_q17_p3.txt").unwrap();
    let mut sx = 0;
    let mut sy = 0;
    let mut cx = 0;
    let mut cy = 0;
    let base_map = input.trim().lines().enumerate().fold(BTreeMap::new(), |mut acc, (y, l)| {
        l.trim().as_bytes().iter().enumerate().for_each(|(x, &b)| {
            if b == b'@' {
                cx = x as isize;
                cy = y as isize;
            } else if b == b'S' {
                sx = x as isize;
                sy = y as isize;
                acc.insert((x as isize, y as isize), 0);
            } else {
                acc.insert((x as isize, y as isize), (b - b'0') as isize);
            }
        });
        acc
    });
    // println!("base_map: {:?}", base_map);
    println!("sx: {sx}, sy: {sy}");
    println!("cx: {cx}, cy: {cy}");

    let mut best_round = 0;
    let mut best_dist = 100000;
    for r in 1..cx {
        // let r = 4;
        let max_dist = (r + 1) * 30;
        println!("max_dist: {max_dist}");
        let map: BTreeMap<(isize, isize), isize> = base_map
            .iter()
            .filter(|(p, _)| {
                let (dx, dy) = (p.0 as isize - cx, p.1 as isize - cy);
                dx * dx + dy * dy > r * r
            })
            .map(|(&k, &v)| (k, v))
            .collect();
        // find opposite point from start and vulcano
        let (tx, ty) = (cx, cy + r + 1);
        println!("tx: {tx}, ty: {ty}");
        // draw_grid(&map, 2 * cx + 1, 2 * cy + 1);

        // dijkstra
        let time_left = dijkstra(tx - 1, ty, &map, sx, sy);
        let time_right = dijkstra(tx + 1, ty, &map, sx, sy);
        println!("time_left: {time_left}, move_right: {time_right}");
        println!("time_left + time_right: {}", time_left + time_right);

        let overall_dist = time_left
            + time_right
            + map.get(&(tx, ty)).unwrap_or(&0)
            + map.get(&(tx - 1, ty)).unwrap_or(&0)
            + map.get(&(tx + 1, ty)).unwrap_or(&0);
        println!("overall_time: {overall_dist}");
        if overall_dist < max_dist && overall_dist < best_dist {
            best_round = r;
            best_dist = overall_dist;
        }
    }
    println!("best_round: {best_round}, best_dist: {best_dist}");
    println!("p3: {}", best_round * best_dist)
}

fn draw_grid(map: &BTreeMap<(isize, isize), isize>, sx: isize, sy: isize) {
    let mut grid = vec![vec!['.'; sx as usize]; sy as usize];
    for (&(x, y), &v) in map {
        grid[y as usize][x as usize] = v.to_string().chars().next().unwrap();
    }
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn dijkstra(start_x: isize, start_y: isize, map: &BTreeMap<(isize, isize), isize>, end_x: isize, end_y: isize) -> isize {
    let mut dist = HashMap::new();
    let mut pq = BinaryHeap::new();
    dist.insert((start_x, start_y), 0);
    pq.push(Reverse((0, (start_x, start_y))));
    while let Some(Reverse((d, (x, y)))) = pq.pop() {
        if x == end_x && y == end_y {
            return d;
        }
        if d > dist[&(x, y)] {
            continue;
        }
        for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (nx, ny) = (x + dx, y + dy);
            if let Some(&w) = map.get(&(nx, ny)) {
                let nd = d + w;
                if nd < *dist.get(&(nx, ny)).unwrap_or(&100000000) {
                    dist.insert((nx, ny), nd);
                    pq.push(Reverse((nd, (nx, ny))));
                }
            }
        }
    }

    100000000
}
