use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use itertools::Itertools;

fn select_next(components: &[(u32, u32)], used_idxs: &HashSet<usize>, connect: u32, p2: bool) -> (u32, u32) {
    if components.len() == used_idxs.len() {
        return (0, 0); // no more to pick
    }

    let mut new_used = used_idxs.clone();
    let mut max = (0, 0); // leaves will provide base case 0 length and 0 strength

    for (idx, comp) in components.iter().enumerate() {
        // for all components that we can connect to and have not used
        if (comp.0 == connect || comp.1 == connect) && !used_idxs.contains(&idx) {
            // try using it
            new_used.insert(idx);
            let new_connect = if comp.0 == connect { comp.1 } else { comp.0 };
            let (length, strength) = select_next(components, &new_used, new_connect, p2);
            new_used.remove(&idx);

            let result = (if p2 { length + 1 } else { 0 }, strength + comp.0 + comp.1);
            // max on tuple will max the length first, then the strength
            max = max.max(result);
        }
    }

    return max;
}

pub fn run() {
    let mut components = Vec::new();

    for line in BufReader::new(File::open("d24_input.txt").unwrap()).lines().filter_map(|l| l.ok()) {
        let (l, r) = line.split("/").next_tuple().unwrap();
        components.push((l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()));
    }

    println!("part 1: {}", select_next(&components, &HashSet::new(), 0, false).1);
    println!("part 2: {}", select_next(&components, &HashSet::new(), 0, true).1);
}