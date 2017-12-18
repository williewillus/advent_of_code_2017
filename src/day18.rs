use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use regex::Regex;

enum Operand {
    Reg(char),
    Imm(i64),
}

enum Insn {
    Snd(Operand),
    Set(Operand, Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Mod(Operand, Operand),
    Rcv(Operand),
    Jgz(Operand, Operand),
}

fn get_value(opnd: &str, regs: &HashMap<char, i64>) -> i64 {
    if let Ok(imm) = opnd.parse::<i64>() {
        println!("imm {}", imm);
        imm
    } else {
        *regs.get(&opnd.chars().next().unwrap()).unwrap_or(&0)
    }
}

fn parse_insn(line: &str, re: &Regex, regs: &mut HashMap<char, i64>, last_played: &mut i64, pc: &mut i64) {
    if let Some(mats) = re.captures(&line) {
        println!("{}", &mats[1]);
        match &mats[1] {
            "snd" => *last_played = get_value(&mats[2], regs),
            "set" => {
                *regs.entry(mats[2].chars().next().unwrap()).or_insert(0) = get_value(&mats[3], regs)
            },
            "add" => *regs.entry(mats[2].chars().next().unwrap()).or_insert(0) += get_value(&mats[3], regs),
            "mul" => *regs.entry(mats[2].chars().next().unwrap()).or_insert(0) *= get_value(&mats[3], regs),
            "mod" => *regs.entry(mats[2].chars().next().unwrap()).or_insert(0) %= get_value(&mats[3], regs),
            "rcv" => {
                if get_value(&mats[2], regs) != 0 {
                    panic!("recovered {}", *last_played); // todo stop abusing panic
                }
            },
            "jgz" => {
                if get_value(&mats[2], regs) > 0 {
                    *pc += get_value(&mats[3], regs);
                } else {
                    *pc += 1;
                }
            },
            _ => panic!("unknown insn {}", &mats[1]),
        }
        if &mats[1] != "jgz" {
            *pc += 1;
        }
    }
}

pub fn run() {
    let re = Regex::new(r"(snd|set|add|mul|mod|rcv|jgz) ([a-z]|-?\d+)(?: ([a-z]|-?\d+))?").unwrap();
    let f = File::open("d18_input.txt").unwrap();

    let lines = BufReader::new(f).lines().filter_map(|l| l.ok()).collect::<Vec<_>>();
    let mut regs = HashMap::new();
    let mut pc = 0i64;
    let mut last_played = 0;
    while pc >= 0 && (pc as usize) < lines.len() {
        parse_insn(&lines[pc as usize], &re, &mut regs, &mut last_played, &mut pc);
        // println!("{:?}", regs);
    }
}