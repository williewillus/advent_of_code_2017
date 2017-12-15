use std::collections::HashSet;
use std::collections::VecDeque;
use day10;

// single hex value -> [bool; 4] representing binary
fn explode(num: u32) -> [bool; 4] {
    let mut ret = [false; 4];
    ret[0] = (num & 1<<3) != 0;
    ret[1] = (num & 1<<2) != 0;
    ret[2] = (num & 1<<1) != 0;
    ret[3] = (num & 1) != 0;
    ret
}

fn to_bools(hash: String) -> Vec<bool> {
    let mut ret = Vec::with_capacity(128);
    hash.chars()
        .map(|c| c.to_digit(16).unwrap()) // hex char -> real value
        .map(explode)
        .for_each(|bs| ret.extend_from_slice(&bs));
    ret
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn neighbors(&self) -> Vec<Coords> {
        let mut ret = Vec::new();

        if self.x > 0 {
            ret.push(Coords { x: self.x - 1, y: self.y });
        }

        if self.x < 127 {
            ret.push(Coords { x: self.x + 1, y: self.y });
        }

        if self.y > 0 {
            ret.push(Coords { x: self.x, y: self.y - 1 });
        }

        if self.y < 127 {
            ret.push(Coords { x: self.x, y: self.y + 1 });
        }

        ret
    }
}

fn flood(grid: &Vec<Vec<bool>>, start: Coords) -> HashSet<Coords> {
    // short circuit for empty case
    if !grid[start.y][start.x] {
        return HashSet::new();
    }

    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(start);

    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        seen.insert(v);

        for nb in v.neighbors() {
            if grid[nb.y][nb.x] && !seen.contains(&nb) {
                q.push_back(nb);
            }
        }
    }

    seen
}

pub fn run() {
    let mut grid = Vec::with_capacity(128);
    for i in 0..128 {
        let hash = day10::knot_hash(&format!("jzgqcdpd-{}", i));
        grid.push(to_bools(hash));
    }

    println!("part 1: {}", grid.iter().map(|row| row.iter().filter(|b| **b).count()).sum::<usize>());

    let mut to_traverse = HashSet::new();
    for i in 0..128 {
        for j in 0..128 {
            to_traverse.insert(Coords { x: i, y: j });
        }
    }

    let mut ccs = 0;
    while !to_traverse.is_empty() {
        let v = *to_traverse.iter().next().unwrap();
        let traversed = flood(&grid, v);

        if traversed.len() > 0 {
            ccs += 1;
        }

        to_traverse.remove(&v);
        for coord in traversed {
            to_traverse.remove(&coord);
        }
    }

    println!("part 2: {}", ccs);
}
