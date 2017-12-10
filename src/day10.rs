use util;

fn circular_get(data: &[u32], idx: usize) -> u32 {
    data[idx % data.len()]
}

fn circular_set(data: &mut [u32], idx: usize, val: u32) {
    let data_len = data.len();
    data[idx % data_len] = val;
}

fn reverse(data: &mut [u32], start: usize, len: usize) {
    for offset in 0..len/2 {
        let start_idx = start + offset;
        let end_idx = start + len - offset - 1;

        let start = circular_get(data, start_idx);
        let end = circular_get(data, end_idx);

        circular_set(data, start_idx, end);
        circular_set(data, end_idx, start);
    }
}

fn hash_iter(data: &mut [u32], lens: &[usize], pos: &mut usize, skip_size: &mut usize) {
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
    p2_lens.extend_from_slice(&[17, 31, 73, 47, 23]);

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
