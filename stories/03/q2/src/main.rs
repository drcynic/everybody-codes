//use std::collections::HashMap;
use std::{collections::HashSet, ops::Add};

fn main() {
    let input = std::fs::read_to_string("everybody_codes_e3_q02_p1.txt").unwrap();
    let mut start = (0, 0);
    let mut vocal_bone = (0, 0);
    input.lines().enumerate().for_each(|(row, l)| {
        if let Some(col) = l.find("#") {
            vocal_bone = (col as i32, row as i32);
        }
        if let Some(col) = l.find("@") {
            start = (col as i32, row as i32);
        }
    });
    let dir = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut current = start;
    let mut visited = HashSet::from([current]);
    let mut num_steps = 0;
    let mut cur_dir_idx = 0;
    while current != vocal_bone {
        // draw_visited(&visited);
        while visited.contains(&(current.0 + dir[cur_dir_idx].0, current.1 + dir[cur_dir_idx].1)) {
            cur_dir_idx = cur_dir_idx.add(1).rem_euclid(dir.len());
        }
        current = (current.0 + dir[cur_dir_idx].0, current.1 + dir[cur_dir_idx].1);
        visited.insert(current);
        cur_dir_idx = cur_dir_idx.add(1).rem_euclid(dir.len());
        num_steps += 1;
    }
    println!("p1: {num_steps}");
}

#[allow(dead_code)]
fn draw_visited(visited: &HashSet<(i32, i32)>) {
    let mut grid = vec![vec!['.'; 30]; 30];
    for (x, y) in visited {
        grid[*y as usize][*x as usize] = '+';
    }
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}
