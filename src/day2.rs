use std::cmp;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let f = File::open("d2_input.txt").unwrap();
    let mut p1_sum = 0;
    let mut p2_sum = 0;

    'nextline:
    for line in BufReader::new(f).lines().filter_map(|l| l.ok()) {
        let mut min = u32::max_value();
        let mut max = u32::min_value();
        let nums: Vec<_> = line.split("\t").map(|c| c.parse::<u32>().unwrap()).collect();

        for &num in &nums {
            if num > max {
                max = num
            }

            if num < min {
                min = num
            }
        }

        p1_sum += max - min;

        for e1 in &nums {
            for e2 in &nums {
                let max = cmp::max(e1, e2);
                let min = cmp::min(e1, e2);
                if e1 != e2 && max % min == 0 {
                    p2_sum += max / min;
                    continue 'nextline;
                }
            }
        }
    }

    println!("part 1: {}", p1_sum);
    println!("part 2: {}", p2_sum);
}
