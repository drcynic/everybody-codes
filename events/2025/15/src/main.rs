use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: i32,
    pos: (i32, i32),
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
    fn new(cost: i32, pos: (i32, i32)) -> Self {
        Self { cost, pos }
    }
}

fn main() {
    let input = std::fs::read_to_string("everybody_codes_e2025_q15_p1a.txt").unwrap();
    println!("Part 1: {}", solve(&input));

    let input = std::fs::read_to_string("everybody_codes_e2025_q15_p2.txt").unwrap();
    println!("Part 2: {}", solve(&input));

    let input = std::fs::read_to_string("everybody_codes_e2025_q15_p3.txt").unwrap();
    println!("Part 3: {}", solve(&input));
}

fn solve(input: &str) -> i32 {
    let mut x_pos = vec![];
    let mut y_pos = vec![];
    let mut walls: Vec<((i32, i32), (i32, i32))> = Vec::new();

    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut dir_idx = 0;
    let mut position = (0, 0);
    for move_str in input.trim().split(",").collect::<Vec<_>>() {
        let off = if &move_str[..1] == "L" { -1i64 } else { 1 };
        dir_idx = (dir_idx + 4 + off) % 4;
        let length: i32 = move_str[1..].parse().unwrap();

        let start = position;
        position.0 += dirs[dir_idx as usize].0 * length;
        position.1 += dirs[dir_idx as usize].1 * length;

        let x1 = start.0.min(position.0);
        let x2 = start.0.max(position.0);
        let y1 = start.1.min(position.1);
        let y2 = start.1.max(position.1);

        walls.push(((x1, y1), (x2, y2)));

        // Add boundary coordinates
        x_pos.push(x1 - 1);
        x_pos.push(x2 + 1);
        y_pos.push(y1 - 1);
        y_pos.push(y2 + 1);
    }
    let end = position;

    // Adjust first and last walls to exclude start and end points
    walls[0] = exclude_point_from_wall(walls[0], (0, 0));
    let last_idx = walls.len() - 1;
    walls[last_idx] = exclude_point_from_wall(walls[last_idx], end);

    x_pos.push(end.0);
    y_pos.push(end.1);
    x_pos.sort();
    y_pos.sort();

    // Dijkstra with stepping only to wall x/y +-1 positions
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(State::new(0, (0, 0)));

    while let Some(State { cost, pos }) = heap.pop() {
        if pos == end {
            return cost;
        }

        if !visited.insert(pos) {
            continue;
        }

        // Try moving to next points in all 4 directions
        let left = x_pos.iter().filter(|&x| *x < pos.0).last().map(|&x| (x, pos.1));
        let right = x_pos.iter().filter(|&x| *x > pos.0).nth(0).map(|&x| (x, pos.1));
        let up = y_pos.iter().filter(|&y| *y < pos.1).last().map(|&y| (pos.0, y));
        let down = y_pos.iter().filter(|&y| *y > pos.1).nth(0).map(|&y| (pos.0, y));
        for next in [left, right, up, down].into_iter().flatten() {
            if visited.contains(&next) {
                continue;
            }

            if is_valid(&walls, pos, next) {
                let move_cost = (pos.0 - next.0).abs() + (pos.1 - next.1).abs(); // manhattan dist
                heap.push(State::new(cost + move_cost, next));
            }
        }
    }

    -1
}

fn exclude_point_from_wall(wall: ((i32, i32), (i32, i32)), point: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let ((x1, y1), (x2, y2)) = wall;

    match (x1, y1, x2, y2) {
        (x, y, _, _) if (x, y) == point && x2 > x => ((x + 1, y), (x2, y2)), // horizontal from point
        (x, y, _, _) if (x, y) == point && y2 > y => ((x, y + 1), (x2, y2)), // vertical from point
        (_, _, x, y) if (x, y) == point && x1 < x => ((x1, y1), (x - 1, y)), // horizontal to point
        (_, _, x, y) if (x, y) == point && y1 < y => ((x1, y1), (x, y - 1)), // vertical to point
        _ => wall,
    }
}

fn is_valid(walls: &[((i32, i32), (i32, i32))], from: (i32, i32), to: (i32, i32)) -> bool {
    let move_x_min = from.0.min(to.0);
    let move_x_max = from.0.max(to.0);
    let move_y_min = from.1.min(to.1);
    let move_y_max = from.1.max(to.1);

    // Check if path intersects any wall
    walls.iter().all(|&((wx1, wy1), (wx2, wy2))| {
        let wall_min_x = wx1.min(wx2);
        let wall_max_x = wx1.max(wx2);
        let wall_min_y = wy1.min(wy2);
        let wall_max_y = wy1.max(wy2);
        move_x_max < wall_min_x || move_x_min > wall_max_x || move_y_max < wall_min_y || move_y_min > wall_max_y
    })
}
