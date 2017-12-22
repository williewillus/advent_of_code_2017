use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use itertools::Itertools;
use util::Direction;
use util::Position;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Status {
    CLEAN,
    WEAKENED,
    INFECTED,
    FLAGGED
}

fn part_1(mut statuses: HashMap<Position, Status>, start: Position) {
    let mut dir = Direction::UP;
    let mut cur_pos = start;
    let mut infect_count = 0;

    for _ in 0..10000 {
        let was_infected = *statuses.get(&cur_pos).unwrap_or(&Status::CLEAN) == Status::INFECTED;
        if was_infected {
            dir = dir.cw();
            statuses.remove(&cur_pos);
        } else {
            dir = dir.ccw();
            statuses.insert(cur_pos, Status::INFECTED);
            infect_count += 1;
        }

        cur_pos = cur_pos.offset(&dir);
    }

    println!("part 1: {}", infect_count);
}

fn part_2(mut statuses: HashMap<Position, Status>, start: Position) {
    let mut dir = Direction::UP;
    let mut cur_pos = start;
    let mut infect_count = 0;

    for _ in 0..10000000 {
        let status = *statuses.get(&cur_pos).unwrap_or(&Status::CLEAN);

        match status {
            Status::CLEAN => {
                dir = dir.ccw();
                statuses.insert(cur_pos, Status::WEAKENED);
            },
            Status::WEAKENED => {
                statuses.insert(cur_pos, Status::INFECTED);
                infect_count += 1;
            },
            Status::INFECTED => {
                dir = dir.cw();
                statuses.insert(cur_pos, Status::FLAGGED);
            },
            Status::FLAGGED => {
                dir = dir.opposite();
                statuses.remove(&cur_pos);
            },
        }

        cur_pos = cur_pos.offset(&dir);
    }

    println!("part 2: {}", infect_count);
}

pub fn run() {
    let lines = BufReader::new(File::open("d22_input.txt").unwrap()).lines().filter_map(|l| l.ok()).collect::<Vec<_>>();
    let given_height = lines.len();
    let given_width = lines[0].len();
    let mut statuses = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                // we receive higher y's first when going down the file
                statuses.insert(Position(x as i32, (given_height - y - 1) as i32), Status::INFECTED);
            }
        }
    }

    let start = Position((given_width / 2) as i32, (given_height / 2) as i32);
    part_1(statuses.clone(), start);
    part_2(statuses, start);
}