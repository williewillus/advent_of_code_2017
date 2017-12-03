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

pub fn run() {
    part_1();
}
