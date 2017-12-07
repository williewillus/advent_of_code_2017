use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Node {
    name: String,
    weight: i32,
    children: Vec<String>,
}

fn get_weight(all: &HashMap<String, Node>, root: &str) -> i32 {
    println!("looking for weight of {}", root);
    let r = &all[root];
    let mut weight = r.weight;
    if !r.children.is_empty() {
        let children_weights = r.children.iter().map(|c| get_weight(all, c)).collect::<Vec<_>>();
        let first_weight = children_weights[0];
        for (idx, wt) in children_weights.iter().enumerate() {
            if *wt != first_weight {
                let bad_child = &r.children[idx];
                println!("{}'s child {} subweight {} differs from its siblings {}", r.name, bad_child, wt, first_weight);

                let diff = first_weight - wt;
                println!("{} was {} and should weigh {} instead", bad_child, all[bad_child].weight, all[bad_child].weight + diff);
            }
            weight += *wt;
        }

    }

    weight
}

fn check_balance(all: &HashMap<String, Node>, root: &Node) -> bool {
    false
}

pub fn run() {
    let f = File::open("d7_input.txt").unwrap();
    let re = Regex::new(r"(\w+) \((\d+)\)(.*)").unwrap();

    let mut all = HashMap::new();
    let mut parents = HashMap::new();

    for line in BufReader::new(f).lines().filter_map(|l| l.ok()) {
        for m in re.captures_iter(&line) {
            let name = &m[1];
            let weight = m[2].parse::<i32>().unwrap();
            let children_spec = if m.len() >= 4 && m[3].len() >= 3 {
                println!("match {}", &m[3]);
                &m[3][4..] // take off space and arrow
            } else {
                ""
            };

            let mut children = Vec::new();
            for child in children_spec.split(", ") {
                if !child.is_empty() {
                    children.push(child.to_string());
                    parents.insert(child.to_string(), name.to_string());
                }
            }

            all.insert(name.to_string(), Node {
                name: name.to_string(),
                weight,
                children
            });

            println!("{} {} {}", name, weight, children_spec);
        }
    }

    let mut root = Node { name: "".to_owned(), weight: 0, children: Vec::new() };
    for (k, v) in &all {
        if !parents.contains_key(k) {
            root = v.clone();
            println!("part 1: {:?}", root);
        }
    }

    // println!("{}", get_weight(&all, &root.name));

}