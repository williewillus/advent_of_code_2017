use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let f = File::open("d5_input.txt").unwrap();

    let mut nums = BufReader::new(f)
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    compute(&mut nums.clone(), false); // need clean copy since it destructively mutates
    compute(&mut nums, true);
}

fn compute(nums: &mut [i32], part_2: bool) {
    let mut pc = 0i32;
    let mut steps = 0;

    while pc >= 0 && (pc as usize) < nums.len() {
        let insn = nums[pc as usize];
        if part_2 && insn >= 3 {
            nums[pc as usize] -= 1;
        } else {
            nums[pc as usize] += 1;
        }
        pc += insn;
        steps += 1;
    }

    if part_2 {
        println!("part 2: {}", steps);
    } else {
        println!("part 1: {}", steps);
    }

}