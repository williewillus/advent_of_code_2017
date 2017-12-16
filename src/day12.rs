use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn bfs_from(neighbors: &HashMap<u32, Vec<u32>>, root: u32) -> HashSet<u32> {
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(root);

    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        seen.insert(v);

        for nb in &neighbors[&v] {
            if !seen.contains(nb) {
                q.push_back(*nb);
            }
        }
    }

    seen
}

fn count_connected_components(neighbors: &HashMap<u32, Vec<u32>>) -> usize {
    let mut all_nodes = neighbors.keys().cloned().collect::<HashSet<_>>();
    let mut ccs = 0;

    while !all_nodes.is_empty() {
        let next_root = *all_nodes.iter().next().unwrap();

        let traversed = bfs_from(neighbors, next_root);
        if traversed.len() > 0 {
            ccs += 1;
        }

        all_nodes.remove(&next_root);
        for n in traversed {
            all_nodes.remove(&n);
        }
    }

    ccs
}

pub fn run() {
    let f = File::open("d12_input.txt").unwrap();
    let mut neighbors = HashMap::new();

    for line in BufReader::new(f).lines().filter_map(|l| l.ok()) {
        let splits = line.split(" <-> ").collect::<Vec<_>>();
        let from = splits[0].parse::<u32>().unwrap();

        let vec = neighbors.entry(from).or_insert(Vec::new());
        for to in splits[1].split(", ") {
            vec.push(to.parse::<u32>().unwrap());
        }
    }

    println!("part 1: {}", bfs_from(&neighbors, 0).len());
    println!("part 2: {}", count_connected_components(&neighbors));
}