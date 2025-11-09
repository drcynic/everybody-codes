use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("everybody_codes_e2025_q03_p1a.txt").unwrap();
    let r1 = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).unique().sum::<i64>();
    println!("Part1: {}", r1);

    let input = std::fs::read_to_string("everybody_codes_e2025_q03_p2a.txt").unwrap();
    let r2 = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).sorted_unstable().dedup().take(20).sum::<i64>();
    println!("Part2: {}", r2);

    let input = std::fs::read_to_string("everybody_codes_e2025_q03_p3a.txt").unwrap();
    let r3 = input.trim().split(',').sorted_unstable().dedup_with_count().map(|(c, _)| c).max().unwrap();
    println!("Part3: {}", r3);
}
