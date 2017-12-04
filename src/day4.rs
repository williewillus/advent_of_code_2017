use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let f = File::open("d4_input.txt").unwrap();

    let mut p1_valid = 0;
    let mut p2_valid = 0;

    let mut s = HashSet::new();
    let mut s2 = HashSet::new();

    for line in BufReader::new(f).lines().filter_map(|l| l.ok()) {
        let words = line.split(" ").collect::<Vec<_>>();

        for w in &words {
            s.insert(w.to_string()); // cloning to shut up borrowcheck, either that or making a new set each iteration

            let mut chars_sorted = w.chars().collect::<Vec<_>>();
            chars_sorted.sort();
            s2.insert(chars_sorted);
        }

        if s.len() == words.len() {
            p1_valid += 1;
        }

        if s2.len() == words.len() {
            p2_valid += 1;
        }

        s.clear();
        s2.clear();
    }

    println!("part 1: {}", p1_valid);
    println!("part 2: {}", p2_valid);
}