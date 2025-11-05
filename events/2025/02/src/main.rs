fn main() {
    let add = |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2);
    let mul = |(x1, y1), (x2, y2)| (x1 * x2 - y1 * y2, x1 * y2 + y1 * x2);
    let div = |(x1, y1), (x2, y2)| (x1 / x2, y1 / y2);

    let a = (25, 9);
    let mut r = (0, 0);
    for _ in 0..3 {
        r = mul(r, r);
        r = div(r, (10, 10));
        r = add(r, a);
    }
    println!("Part1: {:?}", r);

    let s: (i64, i64) = (35300, -64910);
    let sum = calc_sum(s, 100);
    println!("Part2: {}", sum);

    let s: (i64, i64) = (35300, -64910);
    let sum = calc_sum(s, 1000);
    println!("Part3: {}", sum);
}

fn calc_sum(s: (i64, i64), grid_size: i64) -> i64 {
    let add = |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2);
    let mul = |(x1, y1), (x2, y2)| (x1 * x2 - y1 * y2, x1 * y2 + y1 * x2);
    let div = |(x1, y1), (x2, y2)| (x1 / x2, y1 / y2);

    let opp_corner = add(s, (1000, 1000));
    let step_x = (opp_corner.0 - s.0) / grid_size;
    let step_y = (opp_corner.1 - s.1) / grid_size;
    let mut sum = 0;
    for y in (s.1..=opp_corner.1).step_by(step_y as usize) {
        for x in (s.0..=opp_corner.0).step_by(step_x as usize) {
            let a = (x, y);
            let mut r = (0, 0);
            let mut eng = true;
            for _ in 0..100 {
                r = mul(r, r);
                r = div(r, (100000, 100000));
                r = add(r, a);
                if r.0.abs() > 1000000 || r.1.abs() > 1000000 {
                    eng = false;
                    break;
                }
            }
            if eng {
                sum += 1;
            }
        }
    }
    sum
}
