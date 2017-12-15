use day10;

fn count_ones(hash: String) -> u32 {
    hash.chars()
        .map(|c| c.to_digit(16).unwrap().count_ones())
        .sum()
}

pub fn run() {
    let s: u32 = (0..128).map(|i| {
        let hash_input = format!("jzgqcdpd-{}", i);
        day10::knot_hash(&hash_input)
    }).map(count_ones).sum();
    println!("part 1: {}", s);
}