fn main() {
    //p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q11_p1.txt").unwrap();
    let mut flocks = input.trim().lines().map(|l| l.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut round = 0;
    while move_up(&mut flocks) {
        round += 1;
    }
    round += 1;
    while round <= 10 && move_down(&mut flocks) {
        round += 1;
    }
    let p1 = flocks.iter().enumerate().map(|(i, e)| (i + 1) * e).sum::<usize>();
    println!("p1: {}", p1);

    // p2
    let input = std::fs::read_to_string("everybody_codes_e2025_q11_p2.txt").unwrap();
    let mut flocks = input.trim().lines().map(|l| l.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut round = 0;
    while move_up(&mut flocks) {
        round += 1;
    }
    round += 1;
    round += count_move_down(&flocks);
    println!("p2: {}", round);

    // p3 - input is only ascending, find target and sum differences of flocks above target to target
    let input = std::fs::read_to_string("everybody_codes_e2025_q11_p3.txt").unwrap();
    let flocks = input.trim().lines().map(|l| l.parse::<usize>().unwrap()).collect::<Vec<_>>();
    println!("p3: {}", count_move_down(&flocks));
}

fn count_move_down(flocks: &[usize]) -> usize {
    let target = flocks.iter().sum::<usize>() / flocks.len();
    flocks.iter().filter(|e| *e > &target).map(|e| e - target).sum::<usize>()
}

fn move_down(flocks: &mut Vec<usize>) -> bool {
    let mut moved = false;
    for idx in 0..flocks.len() - 1 {
        if flocks[idx + 1] > 0 && flocks[idx + 1] > flocks[idx] {
            flocks[idx] += 1;
            flocks[idx + 1] -= 1;
            moved = true;
        }
    }
    moved
}

fn move_up(flocks: &mut Vec<usize>) -> bool {
    let mut moved = false;
    for idx in 0..flocks.len() - 1 {
        if flocks[idx] > 0 && flocks[idx + 1] < flocks[idx] {
            flocks[idx] -= 1;
            flocks[idx + 1] += 1;
            moved = true;
        }
    }
    moved
}
