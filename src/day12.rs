use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// Use BTreeSet since standard HashSet does not itself implement Hash,
// so we can't use a HashSet of HashSet in count_connected_components
fn bfs_from(neighbors: &HashMap<u32, Vec<u32>>, root: u32) -> BTreeSet<u32> {
    let mut seen = BTreeSet::new();
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
    let mut ccs = HashSet::new();
    for root in neighbors.keys() {
        ccs.insert(bfs_from(neighbors, *root));
    }
    ccs.len()
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