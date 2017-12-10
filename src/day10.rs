use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use util;

// todo do it in place
fn reverse(data: &mut [i32], start: usize, len: usize) {
    let data_len = data.len();
    let mut tmp = vec![0; len];
    for offset in 0..len {
        tmp[offset] = data[(start + offset) % data_len];
    }
    tmp.reverse();
    for offset in 0..len {
        data[(start + offset) % data_len] = tmp[offset];
    }
}

fn doit(mut data: Vec<i32>, lens: &[usize], pos: &mut usize, skip_size: &mut usize) -> Vec<i32> {

    for &len in lens.iter() {
        reverse(&mut data, *pos, len);
        *pos = (*pos + len + *skip_size) % data.len();
        *skip_size += 1;
    }

    data
}

fn xor_all(data: &[i32]) -> i32 {
    let mut result = data[0];
    for val in data.iter().skip(1) {
        result ^= val;
    }
    result
}

pub fn run() {
    let p1 = doit((0..256).collect::<Vec<_>>(), &[94,84,0,79,2,27,81,1,123,93,218,23,103,255,254,243], &mut 0, &mut 0);
    println!("part 1: {}", p1[0] * p1[1]);

    let mut p2_lens = Vec::new();
    for b in util::read_all("d10_input.txt").unwrap().bytes() {
        p2_lens.push(b as usize);
    }
    p2_lens.extend([17, 31, 73, 47, 23].iter());

    let mut p2_data = (0..256).collect::<Vec<_>>();
    let mut pos = 0;
    let mut skip_size = 0;
    for i in 0..64 {
        p2_data = doit(p2_data, &p2_lens, &mut pos, &mut skip_size);
    }

    let mut p2_dense = Vec::new();
    let mut s = String::new();
    for chunk in p2_data.chunks(16) {
        let blargh = xor_all(chunk);
        p2_dense.push(blargh);
        s.push_str(&format!("{:02x}", blargh));
    }

    assert_eq!(16, p2_dense.len());
    assert_eq!(32, s.len());
    println!("{}", s);
}
