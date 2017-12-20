use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use util::Direction;

fn valid_move(pos: (usize, usize), dir: Direction, grid: &Vec<Vec<u8>>) -> bool {
    let (x, y) = pos;
    match dir {
        Direction::DOWN => y < grid.len() - 1,
        Direction::UP => y > 0,
        Direction::RIGHT => x < grid[0].len() - 1,
        Direction::LEFT => x > 0,
    }
}

fn is_clear(pos: (usize, usize), dir: Direction, grid: &Vec<Vec<u8>>) -> bool {
    if valid_move(pos, dir, grid) {
        let (nx, ny) = mv(pos, dir);
        grid[ny][nx] != b' '
    } else {
        false
    }
}

fn mv(pos: (usize, usize), dir: Direction) -> (usize, usize) {
    let (x, y) = pos;
    match dir {
        Direction::DOWN => (x, y+1),
        Direction::UP => (x, y-1),
        Direction::RIGHT => (x+1, y),
        Direction::LEFT => (x-1, y),
    }
}

pub fn run() {
    let grid: Vec<Vec<u8>> = BufReader::new(File::open("d19_input.txt").unwrap())
        .lines().filter_map(|l| l.ok())
        .map(|l| l.into_bytes())
        .collect::<Vec<_>>();
    let mut pos = (grid[0].iter().position(|c| *c == b'|').unwrap(), 0usize);
    let mut dir = Direction::DOWN;
    let mut seen = Vec::new();
    let mut steps = 1; // count entering the grid

    'outer:
    loop {
        if is_clear(pos, dir, &grid) {
            steps += 1;
            pos = mv(pos, dir);

            match grid[pos.1][pos.0] {
                letter @ b'A'...b'Z' => {
                    seen.push(letter);
                } ,
                _ => (),
            }
        } else {
            // cannot advance any further
            for &new_dir in [Direction::DOWN, Direction::UP, Direction::RIGHT, Direction::LEFT].iter() {
                // don't go back where we came from
                if new_dir != dir.opposite()
                    && is_clear(pos, new_dir, &grid) {
                    dir = new_dir;
                    continue 'outer;
                }
            }

            // if no valid ways to turn, exit
            break;
        }
    }

    println!("part 1: {}", seen.iter().map(|b| *b as char).collect::<String>());
    println!("part 2: {}", steps);
}
