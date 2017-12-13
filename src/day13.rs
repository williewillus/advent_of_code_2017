use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn severity(ranges: &HashMap<u32, u32>, preticks: u32) -> (bool, u32) {
    let mut severity = 0;
    let mut caught = false;

    for (&k, &v) in ranges {
        // e.g. range 4: 0 1 2 3 2 1 | 0 1 2 ...
        if (k + preticks) % (2 * (v-1)) == 0 {
            severity += k * v;
            caught = true;
        }
    }

    (caught, severity)
}

pub fn run() {
    let f = File::open("d13_input.txt").unwrap();
    let mut firewall_ranges = HashMap::new();

    for line in BufReader::new(f).lines().filter_map(|l| l.ok()) {
        let splits = line.split(": ").collect::<Vec<_>>();
        let id = splits[0].parse::<u32>().unwrap();
        let range = splits[1].parse::<u32>().unwrap();
        firewall_ranges.insert(id, range);
    }

    println!("part 1: {}", severity(&firewall_ranges, 0).1);

    for preticks in 1.. {
        if !severity(&firewall_ranges, preticks).0 {
            println!("part 2: {}", preticks);
            break;
        }
    }

}