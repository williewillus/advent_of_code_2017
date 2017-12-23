use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
enum Operand {
    Reg(char),
    Imm(i64),
}

#[derive(Copy, Clone, Debug)]
enum Insn {
    Set(char, Operand),
    Sub(char, Operand),
    Mul(char, Operand),
    Jnz(Operand, Operand),
}

fn parse_opnd(opnd: &str) -> Operand {
    if let Ok(imm) = opnd.parse::<i64>() {
        Operand::Imm(imm)
    } else {
        Operand::Reg(opnd.chars().next().unwrap())
    }
}

fn parse_insn(line: &str, re: &Regex) -> Insn {
    let mats = re.captures(&line).unwrap();
    let ch = mats[2].chars().next().unwrap();

    match &mats[1] {
        "set" => Insn::Set(ch, parse_opnd(&mats[3])),
        "sub" => Insn::Sub(ch, parse_opnd(&mats[3])),
        "mul" => Insn::Mul(ch, parse_opnd(&mats[3])),
        "jnz" => Insn::Jnz(parse_opnd(&mats[2]), parse_opnd(&mats[3])),
        _ => panic!("unknown insn {}", &mats[1]),
    }
}

struct State<'a> {
    regs: HashMap<char, i64>,
    pc: i64,
    insns: &'a Vec<Insn>,
    mul_count: u32,
}

impl<'a> State<'a> {
    fn new(insns: &Vec<Insn>) -> State {
        State {
            regs: HashMap::new(),
            pc: 0,
            insns: insns,
            mul_count: 0,
        }
    }

    fn get_val(&self, opnd: Operand) -> i64 {
        match opnd {
            Operand::Reg(r) => *self.regs.get(&r).unwrap_or(&0),
            Operand::Imm(i) => i,
        }
    }

    fn terminated(&self) -> bool {
        self.pc < 0 || (self.pc as usize) >= self.insns.len()
    }

    fn simulate(&mut self) -> bool {
        let insn = self.insns[self.pc as usize];

        self.pc += 1; // pre-inc pc, fix it later if we actually shouldn't have done this

        match insn {
            Insn::Set(r, o) => {
                let v = self.get_val(o);
                self.regs.insert(r, v);
            },
            Insn::Sub(r, o) => {
                *self.regs.entry(r).or_insert(0) -= self.get_val(o);
            },
            Insn::Mul(r, o) => {
                *self.regs.entry(r).or_insert(0) *= self.get_val(o);
                self.mul_count += 1;
            },
            Insn::Jnz(test, o) => {
                let v = self.get_val(test);
                if v != 0 {
                    self.pc -= 1; // undo earlier inc
                    self.pc += self.get_val(o);
                }
            },
        };

        false
    }
}

pub fn run() {
    let re = Regex::new(r"(snd|set|sub|mul|mod|rcv|jgz|jnz) ([a-z]|-?\d+)(?: ([a-z]|-?\d+))?").unwrap();
    let insns = BufReader::new(File::open("d23_input.txt").unwrap()).lines().filter_map(|l| l.ok())
        .map(|l| parse_insn(&l, &re))
        .collect::<Vec<_>>();

    let mut s = State::new(&insns);
    while !s.terminated() {
        s.simulate();
    }
    println!("part 1: {}", s.mul_count);
    println!("part 2: please run day23_p2.c");
}