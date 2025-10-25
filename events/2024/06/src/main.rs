use std::{
    collections::{HashMap, VecDeque},
    iter::*,
};

fn main() {
    let mut cols_by_idx = read_input("everybody_codes_e2024_q05_p1.txt");
    let mut n = 0;
    for r in 0..10 {
        n = calc_round(&mut cols_by_idx, r);
    }
    println!("part1: {}", n);

    let mut cols_by_idx = read_input("everybody_codes_e2024_q05_p2.txt");
    let mut num_counts: HashMap<usize, usize> = HashMap::new();
    let mut r = 0;
    loop {
        n = calc_round(&mut cols_by_idx, r);
        num_counts.entry(n).and_modify(|count| *count += 1).or_insert(1);

        if num_counts.get(&n).unwrap() == &2024 {
            println!("part2: {}", (r + 1) * n);
            break;
        }
        r += 1;
    }

    let mut cols_by_idx = read_input("everybody_codes_e2024_q05_p3.txt");
    let mut max_num = 0usize;
    for r in 0..1000000 {
        n = calc_round(&mut cols_by_idx, r);
        if n > max_num {
            max_num = n;
        }
    }
    println!("Part3: {max_num}");
}

fn read_input(filename: &str) -> Vec<VecDeque<usize>> {
    let input = std::fs::read_to_string(filename).unwrap();
    let grid: Vec<Vec<usize>> = input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect())
        .collect();
    let mut cols_by_idx = vec![VecDeque::new(); grid[0].len()];
    for row in &grid {
        for (col, &num) in row.iter().enumerate() {
            cols_by_idx[col].push_back(num);
        }
    }
    cols_by_idx
}

fn calc_round(grid: &mut Vec<VecDeque<usize>>, round: usize) -> usize {
    let clapper_col_idx = round % grid.len();
    let clapper = grid[clapper_col_idx].pop_front().unwrap();
    let dance_col_idx = (round + 1) % grid.len();
    let dance_col = &mut grid[dance_col_idx];
    let l = dance_col.len();
    let col_rounds = (clapper - 1) / l;
    let col_pos = (clapper - 1) % l;
    if col_rounds % 2 == 0 {
        dance_col.insert(col_pos, clapper);
    } else {
        dance_col.insert(l - col_pos, clapper);
    }

    // combine as string and then parse back to usize
    grid.iter()
        .fold("".to_string(), |a, col| a + &col[0].to_string())
        .parse::<usize>()
        .unwrap()
}

#[allow(dead_code)]
fn print(grid: &Vec<VecDeque<usize>>) {
    let max_col = grid.iter().map(|col| col.len()).max().unwrap();
    for y in 0..max_col {
        for x in 0..grid.len() {
            if y < grid[x].len() {
                print!("{:>3} ", grid[x][y]);
            } else {
                print!("    ");
            }
        }
        println!();
    }
}
