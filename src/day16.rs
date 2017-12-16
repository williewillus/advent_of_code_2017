use std::collections::VecDeque;
use regex::Regex;
use util;

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn parse_input() -> Vec<Move> {
    let mut ret = Vec::new();
    let s_re = Regex::new(r"s(\d+)").unwrap();
    let x_re = Regex::new(r"x(\d+)/(\d+)").unwrap();
    let p_re = Regex::new(r"p(\w+)/(\w+)").unwrap();

    for insn in util::read_all("d16_input.txt").unwrap().split(",") {
        if let Some(caps) = s_re.captures(&insn) {
            let num = caps[1].parse::<usize>().unwrap();
            ret.push(Move::Spin(num));
        } else if let Some(caps) = x_re.captures(&insn) {
            let a = caps[1].parse::<usize>().unwrap();
            let b = caps[2].parse::<usize>().unwrap();
            ret.push(Move::Exchange(a, b));
        } else if let Some(caps) = p_re.captures(&insn) {
            let a = caps[1].chars().next().unwrap();
            let b = caps[2].chars().next().unwrap();
            ret.push(Move::Partner(a, b));
        }
    }

    ret
}

fn dance_iter(moves: &[Move], progs: &mut VecDeque<char>) {
    for m in moves {
        match m {
            &Move::Spin(spins) => {
                for _ in 0..spins {
                    let c = progs.pop_back().unwrap();
                    progs.push_front(c);
                }
            },
            &Move::Exchange(a, b) => progs.swap(a, b),
            &Move::Partner(a, b) => {
                let i = progs.iter().position(|c| *c == a).unwrap();
                let j = progs.iter().position(|c| *c == b).unwrap();
                progs.swap(i, j);
            },
        }
    }
}

fn part_1(moves: &[Move]) {
    let mut progs = VecDeque::new();
    for i in 0..16 {
        progs.push_back((b'a' + i) as char);
    }
    dance_iter(moves, &mut progs);
    println!("part 1: {}", progs.iter().collect::<String>());
}

pub fn run() {
    let moves = parse_input();
    part_1(&moves);

    let mut seen = Vec::new();
    let mut progs = VecDeque::new();
    for i in 0..16 {
        progs.push_back((b'a' + i) as char);
    }

    for i in 0..1000000000 {
        let s = progs.iter().collect::<String>();
        if seen.contains(&s) {
            // found cycle, i = cycle length since we started at 0
            // wrap back around and get the result
            println!("part 2: {} (seen size {})", seen[1000000000 % i], seen.len());
            break;
        }
        seen.push(s);
        dance_iter(&moves, &mut progs);
    }
}