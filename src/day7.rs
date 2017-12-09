use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let f = File::open("d7_input.txt").unwrap();
    let re = Regex::new(r"(\w+) \((\d+)\)(.*)").unwrap();

    let mut all_children = HashSet::new();
    let mut weights = HashMap::new();
    let mut children = HashMap::new();

    for line in BufReader::new(f).lines().filter_map(|l| l.ok()) {
        for m in re.captures_iter(&line) {
            let name = &m[1];
            let weight = m[2].parse::<i32>().unwrap();
            let children_spec = if let Some(idx) = m[3].find("->") {
                &m[3][idx+3..] // take off space and arrow
            } else {
                ""
            };

            let mut v = Vec::new();

            for child in children_spec.split(", ") {
                if !child.is_empty() {
                    all_children.insert(child.to_string());
                    v.push(child.to_string());
                }
            }

            children.insert(name.to_string(), v);
            weights.insert(name.to_string(), weight);
        }
    }

    let mut root = "";
    for node in weights.keys() {
        if !all_children.contains(node) {
            println!("part 1: {} has no parent", node);
            root = node;
            break;
        }
    }

    // Recurse down into the unbalanced child, until there is no longer a unbalanced child
    let mut prev = "".to_string();
    let mut cur = root.to_string();
    while let Some(unbal) = get_unbalanced_child(&cur, &children, &weights) {
        prev = cur;
        cur = unbal;
        println!("prev {} cur {}", prev, cur);
    }

    // find what cur's subweight should be by looking at one of its siblings
    let cur_weight = subtree_weight(&cur, &children, &weights);
    let sibling: &str = &children[&prev].iter().find(|c| **c != cur).unwrap();
    let sib_weight = subtree_weight(sibling, &children, &weights);


    let diff = sib_weight - cur_weight;
    println!("part 2: {}'s subtree weight is {}, should be {} (diff {})", cur, cur_weight, sib_weight, diff);
    println!("part 2: {}'s own weight should be changed to {}", cur, weights[&cur] + diff);
}

fn subtree_weight(root: &str, children: &HashMap<String, Vec<String>>, weights: &HashMap<String, i32>) -> i32 {
    let mut ret = weights[root];
    for child in &children[root] {
        ret += weights[child];
    }
    ret
}

fn get_unbalanced_child(root: &str, children: &HashMap<String, Vec<String>>, weights: &HashMap<String, i32>) -> Option<String> {
    // weights -> how many times we've seen them
    let mut seen_weights = HashMap::new();
    for child in &children[root] {
        let subweight = subtree_weight(child, children, weights);
        println!("{} subweight {}", child, subweight);
        let old_freq = *seen_weights.get(&subweight).unwrap_or(&0);
        seen_weights.insert(subweight, old_freq + 1);
    }

    // if there's a subtree weight we've only seen once at this level, it's the unbalanced child
    // ASSUMPTION: there is only one entry with freq 1, all other weights are identical
    for (weight, freq) in &seen_weights {
        if *freq == 1 {
            let child = children[root].iter().find(|c| *weight == subtree_weight(c, &children, &weights)).unwrap();
            println!("{:?}", seen_weights);
            return Some(child.clone());
        }
    }

    None
}