use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let mut input = vec![14, 0, 15, 12, 11, 11, 3, 5, 1, 6, 8, 4, 9, 1, 8, 4];

    let mut seen = HashMap::new();
    seen.insert(input.clone(), 0);

    let mut iters = 0;
    loop {
        let hi = find_highest(&input);
        redistribute(&mut input, hi);
        iters += 1;
        if seen.contains_key(&input) {
            println!("part 1: {}", iters);
            println!("part 2: {}", iters - seen[&input]);
            break;
        } else {
            seen.insert(input.clone(), iters);
        }
    }
}

fn redistribute(nums: &mut [u32], start_from: usize) {
    let mut redist = nums[start_from];
    nums[start_from] = 0;
    let mut idx = (start_from + 1) % nums.len();
    while redist > 0 {
        nums[idx] += 1;
        redist -= 1;
        idx = (idx + 1) % nums.len();
    }
}

fn find_highest(nums: &[u32]) -> usize {
    let mut max = u32::min_value();
    let mut max_idx = 0;

    for (idx, &i) in nums.iter().enumerate() {
        if i > max {
            max = i;
            max_idx = idx;
        }
    }

    max_idx
}