//use std::collections::HashMap;
use std::{collections::HashSet, iter::*};

//use itertools::{Either, Itertools};

fn main() {
    let part1 = calc("everybody_codes_e2024_q03_p1.txt", false);
    println!("Part 1: {}", part1);
    let part2 = calc("everybody_codes_e2024_q03_p2.txt", false);
    println!("Part 2: {}", part2);
    let part3 = calc("everybody_codes_e2024_q03_p3.txt", true);
    println!("Part 3: {}", part3);
}

fn calc(input_file: &str, use_diag: bool) -> usize {
    let input = std::fs::read_to_string(input_file).unwrap();
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
        .collect();
    //print_grid(&grid);
    let ones = grid.iter().flatten().filter(|&&x| x == 1).count();
    let num = count_edges(&grid, 2, use_diag);
    ones + num
}

fn count_edges(grid: &[Vec<i32>], ref_val: i32, use_diag: bool) -> usize {
    let mut grid = grid.to_vec();
    let mut outsides = HashSet::new();
    for y in 0..grid.len() - 1 {
        for x in 0..grid[y].len() - 1 {
            let cv = grid[y][x];
            if cv < ref_val - 1 {
                continue;
            }

            let mut dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            if use_diag {
                dirs.extend([(1, 1), (-1, 1), (-1, -1), (1, -1)]);
            }
            let valid = dirs.iter().all(|d| {
                let (dx, dy) = d;
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                nx >= 0
                    && nx < grid[y].len() as i32
                    && ny >= 0
                    && ny < grid.len() as i32
                    && (ref_val - grid[ny as usize][nx as usize]).abs() <= 1
            });
            if valid {
                outsides.insert((x, y));
            }
        }
    }

    if !outsides.is_empty() {
        for (x, y) in &outsides {
            grid[*y][*x] = ref_val;
        }
        outsides.len() + count_edges(&grid, ref_val + 1, use_diag)
    } else {
        0
    }
}

fn print_grid(grid: &[Vec<i32>]) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
    println!();
}
