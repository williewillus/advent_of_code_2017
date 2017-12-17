const INPUT: usize = 304;

fn part_1() {
    let mut v = Vec::with_capacity(2018);
    v.push(0);
    let mut pos = 0;

    for i in 1..2018 {
        pos = ((pos + INPUT) % v.len()) + 1;
        v.insert(pos, i);
    }

    let needle = v.iter().position(|i| *i == 2017).unwrap();
    println!("part 1: idx {} ~ {:?}", needle, &v[needle..needle+2]);
}

fn part_2() {
    let mut pos = 0;
    let mut len = 1;
    let mut after_zero = 0;

    for i in 1..50000000 {
        pos = ((pos + INPUT) % len) + 1;
        if pos == 1 {
            after_zero = i;
        }
        len += 1;
    }

    println!("part 2: {}", after_zero);
}

pub fn run() {
    part_1();
    part_2();
}