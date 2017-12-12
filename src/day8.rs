use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let f = File::open("d8_input.txt").unwrap();
    let mut mem = HashMap::new();
    let mut max = i32::min_value();

    for line in BufReader::new(f).lines().filter_map(|l| l.ok()) {
        let splits = line.split_whitespace().collect::<Vec<_>>();

        let amount = {
            let res = splits[2].parse::<i32>().unwrap();
            match splits[1] {
                "dec" => -1 * res,
                "inc" => res,
                _ => panic!("unknown op {}", &splits[1])
            }
        };

        let pred_var = *mem.entry(splits[4].to_string()).or_insert(0);
        let pred_val = splits[6].parse::<i32>().unwrap();
        let pred_success = match splits[5] {
            "==" => pred_var == pred_val,
            ">=" => pred_var >= pred_val,
            ">" => pred_var > pred_val,
            "<" => pred_var < pred_val,
            "<=" => pred_var <= pred_val,
            "!=" => pred_var != pred_val,
            _ => panic!("unknown predicate {}", &splits[5])
        };

        if pred_success {
            *mem.entry(splits[0].to_string()).or_insert(0) += amount;
            max = max.max(mem[splits[0]]); // wew
        }
    }

    println!("part 1: {}", mem.iter().max_by(|&(_, v1), &(_, v2)| v1.cmp(v2)).unwrap().1);
    println!("part 2: {}", max);
}