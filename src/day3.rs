use std::collections::HashMap;
use std::vec::Vec;

const INPUT: u32 = 347991;

fn ring_to_sidelen(ring: u32) -> u32 {
    (2*ring + 1)
}

fn ring_to_ringmax(ring: u32) -> u32 {
    (2*ring + 1) * (2*ring + 1)
}

fn find_ring(val: u32) -> u32 {
    let mut ring = 0;
    let mut root = 1u32;

    while root * root  < val {
        root += 2;
        ring += 1;
    }

    return ring;
}

fn is_corner(val: u32) -> bool {
    let ring = find_ring(val);
    if ring == 0 {
        return true;
    }
    let prev_ring_max = ring_to_ringmax(ring - 1);
    let m = 2u32.pow(ring);
    (val - prev_ring_max) % m == 0
}

fn part_1() {
    let ring = find_ring(INPUT);
    let prev_ringmax = ring_to_ringmax(ring - 1);
    let ringmax = ring_to_ringmax(ring);

    // just simulate to find dist of the other axis, meh
    // basically, note that the offset of each side is `ring` at the corners, descending to 0 in the center of the side
    // the ring's lowest value starts right above the corner, so `off` starts at ring-1
    let mut descend = true;
    let mut off = ring-1;
    for i in prev_ringmax+1..ringmax+1 {
        if i == INPUT {
            break;
        }

        if descend {
            if off == 0 {
                descend = false;
                off += 1;
            } else {
                off -= 1;
            }
        } else {
            if off == ring {
                descend = true;
                off -= 1;
            } else {
                off += 1;
            }
        }
    }

    println!("part 1: {}", ring + off);
}

const UP: (i32, i32) = (0, 1);
const DOWN: (i32, i32) = (0, -1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);

fn turn_ccw(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        UP => LEFT,
        LEFT => DOWN,
        DOWN => RIGHT,
        RIGHT => UP,
        _ => panic!("unknown direction vector {:?}", dir)
    }
}

fn offset(p: (i32, i32), dir: (i32, i32)) -> (i32, i32) {
    return (p.0 + dir.0, p.1 + dir.1)
}

fn part_2() {
    let mut grid: HashMap<(i32, i32), u32> = HashMap::new();
    grid.insert((0, 0), 1);
    let mut cur_pos = (1, 0);
    let mut cur_direction = UP;
    let mut cur_steps_until_turn = 1u32;
    let mut cur_side = 0;

    let mut steps_until_turn = 1;

    loop {
        // fill in spot at cur_pos
        let val = grid.get(&offset(cur_pos, UP)).unwrap_or(&0)
            + grid.get(&offset(cur_pos, DOWN)).unwrap_or(&0)
            + grid.get(&offset(cur_pos, LEFT)).unwrap_or(&0)
            + grid.get(&offset(cur_pos, RIGHT)).unwrap_or(&0)
            + grid.get(&offset(offset(cur_pos, UP), LEFT)).unwrap_or(&0)
            + grid.get(&offset(offset(cur_pos, UP), RIGHT)).unwrap_or(&0)
            + grid.get(&offset(offset(cur_pos, DOWN), LEFT)).unwrap_or(&0)
            + grid.get(&offset(offset(cur_pos, DOWN), RIGHT)).unwrap_or(&0);
        grid.insert(cur_pos, val);
        println!("filling {:?} with {}", cur_pos, val);
        if val > INPUT {
            println!("part 2: {}", val);
            break;
        }

        // move to next pos
        println!("cursteps {}", cur_steps_until_turn);
        if cur_steps_until_turn == 0 {
            // turn

            if cur_side == 3 {
                // finished a ring, go on to the next one outside
                cur_pos = offset(cur_pos, RIGHT);
                cur_direction = UP;
                cur_side = 0;
                steps_until_turn += 2;
                cur_steps_until_turn = steps_until_turn;
                println!("finished a ring, moving to {:?}", cur_pos);
            } else {
                // start the next side
                cur_side += 1;
                cur_direction = turn_ccw(cur_direction);
                cur_steps_until_turn = steps_until_turn;
                println!("turned to face {:?}", cur_direction);
                cur_pos = offset(cur_pos, cur_direction);
            }
        } else {
            // move along a side
            cur_steps_until_turn -= 1;
            cur_pos = offset(cur_pos, cur_direction);
            println!("moved to {:?}", cur_pos);
        }
    }

}

pub fn run() {
    part_1();
    part_2();
}
