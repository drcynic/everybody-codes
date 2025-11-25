fn main() {
    // p1
    let input = std::fs::read_to_string("everybody_codes_e2025_q16_p1.txt").unwrap();
    let nums = input.trim().split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let sum = nums.iter().map(|n| 90 / n).sum::<usize>();
    println!("Part 1: {:?}", sum);

    // p2
    let input = std::fs::read_to_string("everybody_codes_e2025_q16_p2.txt").unwrap();
    let nums = input.trim().split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let prod = formula(nums).iter().product::<usize>();
    println!("Part 2: {:?}", prod);

    // p3
    let input = std::fs::read_to_string("everybody_codes_e2025_q16_p3.txt").unwrap();
    let nums = input.trim().split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let formula = formula(nums);
    // println!("formula: {:?}", &formula);
    let blocks: usize = 202520252025000;
    // let blocks: usize = 100000000000000;
    // let blocks: usize = 10000;
    let mut upper = blocks;
    let mut lower = 1;

    while lower + 1 < upper {
        let pivot = (upper + lower) / 2;
        if formula.iter().map(|n| pivot / n).sum::<usize>() <= blocks {
            lower = pivot; // next section
        } else {
            upper = pivot; // too large, shrink
        }
    }
    println!("Part 3: {lower}");
}

fn formula(nums: Vec<usize>) -> Vec<usize> {
    nums.iter().enumerate().fold(Vec::new(), |mut acc, (i, &n)| {
        let mut sum_so_far = 0;
        for m in &acc {
            if (i + 1) % m == 0 {
                sum_so_far += 1;
            }
        }
        if sum_so_far < n {
            acc.push(i + 1);
        }
        acc
    })
}
