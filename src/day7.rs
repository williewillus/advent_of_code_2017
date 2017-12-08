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

            if !children_spec.is_empty() {
                let mut v = Vec::new();

                for child in children_spec.split(", ") {
                    all_children.insert(child.to_string());
                    v.push(child.to_string());
                }

                children.insert(name.to_string(), v);
            }


            weights.insert(name.to_string(), weight);
        }
    }

    let mut root = "";
    for node in weights.keys() {
        if !all_children.contains(node) {
            println!("{} has no parent", node);
            root = node;
            break;
        }
    }

    let mut prev = "".to_string();
    let mut cur = root.to_string();
    while let Some(unbal) = get_unbalanced_child(&cur, &children, &weights) {
        prev = cur;
        cur = unbal;
        println!("prev {} cur {}", prev, cur);
    }

    // prev is unbalanced, its child cur differs
    // cur's children all have the same weight
    println!("final cur {}", cur);

    println!("{}", subtree_weight(root, &children, &weights));
    println!("{:?}", get_unbalanced_child("nnoaqvv", &children, &weights));
}

fn subtree_weight(root: &str, children: &HashMap<String, Vec<String>>, weights: &HashMap<String, i32>) -> i32 {
    let mut ret = weights[root];
    for child in children.get(root).unwrap_or(&Vec::new()) {
        println!("{}", child);
        ret += weights[child];
    }
    ret
}

fn get_unbalanced_child(root: &str, children: &HashMap<String, Vec<String>>, weights: &HashMap<String, i32>) -> Option<String> {
    // weights -> how many times we've seen them
    let mut seen_weights = HashMap::new();
    for child in children.get(root).unwrap_or(&Vec::new()) {
        *seen_weights.entry(subtree_weight(child, children, weights)).or_insert(0) += 1;
    }

    for (weight, freq) in seen_weights {
        if freq == 1 {
            let child = children[root].iter().find(|c| weight == subtree_weight(c, &children, &weights)).unwrap();
            return Some(child.clone());
        }
    }

    None
}