use std::collections::HashSet;

use itertools::Itertools;
use memoize::memoize;

fn main() {
    //p1
    // let input = std::fs::read_to_string("everybody_codes_e2025_q12_p1a.txt").unwrap();
    // let grid = input.trim().lines().fold(Vec::new(), |mut acc_y, l| {
    //     let row = l.as_bytes().iter().fold(Vec::new(), |mut acc_x, b| {
    //         acc_x.push(b - b'0');
    //         acc_x
    //     });
    //     acc_y.push(row);
    //     acc_y
    // });
    // // println!("{:?}", &grid);

    // let width = grid[0].len();
    // let height = grid.len();
    // println!("width: {}, height: {}", width, height);
    // let mut stack = vec![(0, 0)];
    // let mut visited = HashSet::new();
    // visited.insert((0, 0));
    // let mut count = 1;
    // while !stack.is_empty() {
    //     let pos = stack.pop().unwrap();
    //     println!("pos: {:?}", pos);
    //     for dir in [(-1i64, 0), (0, -1i64), (1i64, 0), (0, 1i64)] {
    //         let new_pos = ((pos.0 as i64 + dir.0) as usize, (pos.1 as i64 + dir.1) as usize);
    //         println!("new_pos: {:?}", new_pos);
    //         if !visited.contains(&new_pos) && new_pos.0 < width && new_pos.1 < height && grid[new_pos.1][new_pos.0] <= grid[pos.1][pos.0] {
    //             // println!(
    //             //     "pos: {:?}, b: {}, new_pos: {:?}, b: {}",
    //             //     pos,
    //             //     grid[pos.1][pos.0], new_pos, grid[new_pos.1][new_pos.0]
    //             // );
    //             stack.push(new_pos);
    //             visited.insert(new_pos);
    //             count += 1;
    //         }
    //     }
    // }
    // let p1 = count;
    // println!("p1: {}", p1);

    // p2
    // let input = std::fs::read_to_string("everybody_codes_e2025_q12_p2.txt").unwrap();
    // let grid = input.trim().lines().fold(Vec::new(), |mut acc_y, l| {
    //     let row = l.as_bytes().iter().fold(Vec::new(), |mut acc_x, b| {
    //         acc_x.push(b - b'0');
    //         acc_x
    //     });
    //     acc_y.push(row);
    //     acc_y
    // });
    // // println!("{:?}", &grid);

    // let width = grid[0].len();
    // let height = grid.len();
    // println!("width: {}, height: {}", width, height);
    // let mut stack = vec![(0, 0), (width - 1, height - 1)];
    // let mut visited = HashSet::new();
    // visited.insert((0, 0));
    // visited.insert((width - 1, height - 1));
    // let mut count = 2;
    // while !stack.is_empty() {
    //     let pos = stack.pop().unwrap();
    //     // println!("pos: {:?}", pos);
    //     for dir in [(-1i64, 0), (0, -1i64), (1i64, 0), (0, 1i64)] {
    //         let new_pos = ((pos.0 as i64 + dir.0) as usize, (pos.1 as i64 + dir.1) as usize);
    //         // println!("new_pos: {:?}", new_pos);
    //         if !visited.contains(&new_pos) && new_pos.0 < width && new_pos.1 < height && grid[new_pos.1][new_pos.0] <= grid[pos.1][pos.0] {
    //             // println!(
    //             //     "pos: {:?}, b: {}, new_pos: {:?}, b: {}",
    //             //     pos,
    //             //     grid[pos.1][pos.0], new_pos, grid[new_pos.1][new_pos.0]
    //             // );
    //             stack.push(new_pos);
    //             visited.insert(new_pos);
    //             count += 1;
    //         }
    //     }
    // }
    // println!("p2: {}", count);

    let input = std::fs::read_to_string("everybody_codes_e2025_q12_p3.txt").unwrap();
    let grid = input.trim().lines().fold(Vec::new(), |mut acc_y, l| {
        let row = l.as_bytes().iter().fold(Vec::new(), |mut acc_x, b| {
            acc_x.push(b - b'0');
            acc_x
        });
        acc_y.push(row);
        acc_y
    });
    // println!("{:?}", &grid);

    let width = grid[0].len();
    let height = grid.len();
    println!("width: {}, height: {}", width, height);
    let mut max_pos = (0, 0);
    let mut max_count = 0;
    let mut max_visited = HashSet::new();
    for y in 0..height {
        println!("y: {}", y);
        for x in 0..width {
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            visited.insert((x, y));
            let count = bfs(&grid, &mut visited, &[(x, y)], width, height);
            // println!("count: {}", count);
            if count > max_count {
                max_count = count;
                max_pos = (x, y);
                max_visited = visited;
            }
        }
    }
    let r1_visited = max_visited.clone();
    let r1_pos = max_pos;
    let r1_count = max_count;
    let mut max_pos = (0, 0);
    let mut max_count = 0;
    let mut max_visited = HashSet::new();
    for y in 0..height {
        println!("y: {}", y);
        for x in 0..width {
            let mut visited: HashSet<(usize, usize)> = r1_visited.clone();
            visited.insert((x, y));
            let count = bfs(&grid, &mut visited, &[(x, y)], width, height);
            // println!("count: {}", count);
            if count > max_count {
                max_count = count;
                max_pos = (x, y);
                max_visited = visited;
            }
        }
    }

    let r2_visited: HashSet<(usize, usize)> = max_visited.clone().into_iter().merge(r1_visited).collect();
    let r2_pos = max_pos;
    let r2_count = max_count;

    let mut max_pos = (0, 0);
    let mut max_count = 0;
    let mut max_visited = HashSet::new();
    for y in 0..height {
        println!("y: {}", y);
        for x in 0..width {
            let mut visited: HashSet<(usize, usize)> = r2_visited.clone();
            visited.insert((x, y));
            let count = bfs(&grid, &mut visited, &[(x, y)], width, height);
            // println!("count: {}", count);
            if count > max_count {
                max_count = count;
                max_pos = (x, y);
                max_visited = visited;
            }
        }
    }

    let r3_pos = max_pos;
    let r3_count = max_count;
    let r3_visited = max_visited;
    println!("r1 pos: {:?}", r1_pos);
    println!("r2 pos: {:?}", r2_pos);
    println!("r3 pos: {:?}", r3_pos);

    let starters = [r1_pos, r2_pos, r3_pos];
    println!("starters: {:?}", starters);
    let count = bfs(
        &grid,
        &mut HashSet::from_iter(starters.iter().cloned().collect::<HashSet<_>>()),
        &starters,
        width,
        height,
    );
    println!("p3: {}", count);
}

// #[memoize[Ignore:grid]]
fn bfs(grid: &Vec<Vec<u8>>, visited: &mut HashSet<(usize, usize)>, start: &[(usize, usize)], width: usize, height: usize) -> i32 {
    let mut stack = Vec::from(start);
    // let mut visited: HashSet<(usize, usize)> = start.iter().cloned().collect();
    let mut count = start.len() as i32;
    while !stack.is_empty() {
        let pos = stack.pop().unwrap();
        for dir in [(-1i64, 0), (0, -1i64), (1i64, 0), (0, 1i64)] {
            let new_pos = ((pos.0 as i64 + dir.0) as usize, (pos.1 as i64 + dir.1) as usize);
            if !visited.contains(&new_pos) && new_pos.0 < width && new_pos.1 < height && grid[new_pos.1][new_pos.0] <= grid[pos.1][pos.0] {
                stack.push(new_pos);
                visited.insert(new_pos);
                count += 1;
            }
        }
    }
    count
}
