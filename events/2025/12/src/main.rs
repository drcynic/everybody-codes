use std::collections::HashSet;

fn main() {
    //p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q12_p1.txt").unwrap();
    let grid = read_grid(input);
    let (width, height) = (grid[0].len(), grid.len());
    let start = [(0, 0)];
    let p1 = bfs(&grid, &mut HashSet::from(start), &start, width, height);
    println!("p1: {}", p1);

    // p2
    let input = std::fs::read_to_string("everybody_codes_e2025_q12_p2.txt").unwrap();
    let grid = read_grid(input);
    let (width, height) = (grid[0].len(), grid.len());
    let start = [(0, 0), (width - 1, height - 1)];
    let p2 = bfs(&grid, &mut HashSet::from(start), &start, width, height);
    println!("p2: {}", p2);

    // p3
    let input = std::fs::read_to_string("everybody_codes_e2025_q12_p3.txt").unwrap();
    let grid = read_grid(input);
    let (width, height) = (grid[0].len(), grid.len());
    let (r1_pos, r1_visited) = get_max(&grid, width, height, &HashSet::new());
    let (r2_pos, r2_visited) = get_max(&grid, width, height, &r1_visited);
    let (r3_pos, _) = get_max(&grid, width, height, &r2_visited);
    let starters = [r1_pos, r2_pos, r3_pos];
    let count = bfs(&grid, &mut HashSet::from(starters), &starters, width, height);
    println!("p3: {}", count);
}

fn get_max(
    grid: &Vec<Vec<u8>>,
    width: usize,
    height: usize,
    initial_visited: &HashSet<(usize, usize)>,
) -> ((usize, usize), HashSet<(usize, usize)>) {
    let mut max_pos = (0, 0);
    let mut max_count = 0;
    let mut max_visited = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            let mut visited = initial_visited.clone();
            visited.insert((x, y));
            let count = bfs(grid, &mut visited, &[(x, y)], width, height);
            if count > max_count {
                max_count = count;
                max_pos = (x, y);
                max_visited = visited;
            }
        }
    }
    (max_pos, max_visited)
}

fn read_grid(input: String) -> Vec<Vec<u8>> {
    let grid = input.trim().lines().fold(Vec::new(), |mut acc_y, l| {
        let row = l.as_bytes().iter().fold(Vec::new(), |mut acc_x, b| {
            acc_x.push(b - b'0');
            acc_x
        });
        acc_y.push(row);
        acc_y
    });
    grid
}

fn bfs(grid: &Vec<Vec<u8>>, visited: &mut HashSet<(usize, usize)>, start: &[(usize, usize)], width: usize, height: usize) -> i32 {
    let mut stack = Vec::from(start);
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
