use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use util;

// todo do it in place / without allocating
fn reverse(data: &mut [i32], start: usize, len: usize) {
    let data_len = data.len();
    let mut tmp = vec![0; len];
    for offset in 0..len {
        tmp[offset] = data[(start + offset) % data_len];
    }

    for (offset, val) in tmp.iter().rev().enumerate() {
        data[(start + offset) % data_len] = *val;
    }
}

fn hash_iter(data: &mut [i32], lens: &[usize], pos: &mut usize, skip_size: &mut usize) {
    for &len in lens {
        reverse(data, *pos, len);
        *pos = (*pos + len + *skip_size) % data.len();
        *skip_size += 1;
    }
}

pub fn run() {
    let mut p1_data = (0..256).collect::<Vec<_>>();
    hash_iter(&mut p1_data, &[94,84,0,79,2,27,81,1,123,93,218,23,103,255,254,243], &mut 0, &mut 0);
    println!("part 1: {}", p1_data[0] * p1_data[1]);

    let mut p2_lens = util::read_all("d10_input.txt").unwrap().bytes().map(|b| b as usize).collect::<Vec<_>>();
    p2_lens.extend([17, 31, 73, 47, 23].iter());

    let mut p2_data = (0..256).collect::<Vec<_>>();
    let mut pos = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        hash_iter(&mut p2_data, &p2_lens, &mut pos, &mut skip_size);
    }

    let res = p2_data.chunks(16)
        .map(|chnk| {
            let mut result = 0;
            for val in chnk {
                result ^= val;
            }
            result
        })
        .map(|i| format!("{:02x}", i))
        .collect::<Vec<_>>()
        .join("");

    assert_eq!(32, res.len());
    println!("part 2: {}", res);
}
