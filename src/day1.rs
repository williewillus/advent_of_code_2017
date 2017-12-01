use util;

pub fn run() {
    let input: Vec<_> = util::read_all("d1_input.txt").unwrap()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &[u32]) {
    let mut sum = 0;
    for i in 0..input.len() {
        if input [i] == input [(i + 1) % input.len()] {
            sum += input[i];
        }
    }

    println!("part 1: {}", sum)
}

fn part_2(input: &[u32]) {
    let mut sum = 0;
    for i in 0..input.len() {
        if input [i] == input [(i + input.len() / 2) % input.len()] {
            sum += input[i];
        }
    }

    println!("part 2: {}", sum)
}