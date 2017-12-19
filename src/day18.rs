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
    Snd(char),
    Set(char, Operand),
    Add(char, Operand),
    Mul(char, Operand),
    Mod(char, Operand),
    Rcv(char),
    Jgz(Operand, Operand),
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
        "snd" => Insn::Snd(ch),
        "set" => Insn::Set(ch, parse_opnd(&mats[3])),
        "add" => Insn::Add(ch, parse_opnd(&mats[3])),
        "mul" => Insn::Mul(ch, parse_opnd(&mats[3])),
        "mod" => Insn::Mod(ch, parse_opnd(&mats[3])),
        "rcv" => Insn::Rcv(ch),
        "jgz" => Insn::Jgz(parse_opnd(&mats[2]), parse_opnd(&mats[3])),
        _ => panic!("unknown insn {}", &mats[1]),
    }
}

struct State<'a> {
    regs: HashMap<char, i64>,
    pc: i64,
    insns: &'a Vec<Insn>,

    // part 1
    last_played: i64,

    // part 2
    send_count: u32,
    blocked: bool,
}

impl<'a> State<'a> {
    fn new(insns: &Vec<Insn>) -> State {
        State {
            regs: HashMap::new(),
            pc: 0,
            insns: insns,
            last_played: 0,
            send_count: 0,
            blocked: false,
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

    // return true if we sent this iteration
    fn step(&mut self, snd: &mut VecDeque<i64>, rcv: &mut VecDeque<i64>) -> bool {
        !self.terminated() && self.simulate(snd, rcv, false)
    }

    // if part 1, return true if we recovered something and should stop
    // if part 2, return true if we sent something
    // todo clean up? I don't want two copies of the interpreter though
    fn simulate(&mut self, snd: &mut VecDeque<i64>, rcv: &mut VecDeque<i64>, part_1: bool) -> bool {
        let insn = self.insns[self.pc as usize];

        self.pc += 1; // pre-inc pc, fix it later if we actually shouldn't have done this

        match insn {
            Insn::Snd(r) => {
                if part_1 {
                    self.last_played = *self.regs.get(&r).unwrap_or(&0)
                } else {
                    self.send_count += 1;
                    snd.push_back(*self.regs.get(&r).unwrap_or(&0));
                    return true;
                }
            },
            Insn::Set(r, o) => {
                let v = self.get_val(o);
                self.regs.insert(r, v);
            },
            Insn::Add(r, o) => {
                *self.regs.entry(r).or_insert(0) += self.get_val(o);
            },
            Insn::Mul(r, o) => {
                *self.regs.entry(r).or_insert(0) *= self.get_val(o);
            },
            Insn::Mod(r, o) => {
                *self.regs.entry(r).or_insert(0) %= self.get_val(o);
            },
            Insn::Rcv(ch) => {
                if part_1 && *self.regs.get(&ch).unwrap_or(&0) > 0 {
                    println!("part 1: recovered {}", self.last_played);
                    return true;
                } else {
                    if rcv.is_empty() {
                        self.blocked = true;
                        self.pc -= 1; // don't increment pc so this insn will be re-run next iteration
                    } else {
                        self.blocked = false;
                        self.regs.insert(ch, rcv.pop_front().unwrap());
                    }
                }
            },
            Insn::Jgz(test, o) => {
                let v = self.get_val(test);
                if v > 0 {
                    self.pc -= 1; // undo earlier inc
                    self.pc += self.get_val(o);
                }
            }
        };

        false
    }
}

fn part_1(mut state: State) {
    let mut dummy_1 = VecDeque::new();
    let mut dummy_2 = VecDeque::new();

    while !state.terminated() {
        if state.simulate(&mut dummy_1, &mut dummy_2, true) {
            break;
        }
    }
}

fn part_2(insns: &Vec<Insn>) {
    let mut prog_0 = State::new(&insns);
    let mut prog_1 = State::new(&insns);
    prog_1.regs.insert('p', 1);

    let mut p0_rcv_queue = VecDeque::new();
    let mut p1_rcv_queue = VecDeque::new();

    while !(prog_0.blocked && prog_1.blocked)
        && !(prog_0.terminated() && prog_1.terminated()) {
        // run p0 until it can't
        while !prog_0.blocked && !prog_0.terminated() {
            if prog_0.step(&mut p1_rcv_queue, &mut p0_rcv_queue) {
                // p0 sent so unblock p1
                prog_1.blocked = false;
            }
        }

        // run p1 until it can't
        while !prog_1.blocked && !prog_1.terminated() {
            if prog_1.step(&mut p0_rcv_queue, &mut p1_rcv_queue) {
                // p1 sent so unblock p0
                prog_0.blocked = false;
            }
        }
    }

    println!("part 2: {}", prog_1.send_count);
}

pub fn run() {
    let re = Regex::new(r"(snd|set|add|mul|mod|rcv|jgz) ([a-z]|-?\d+)(?: ([a-z]|-?\d+))?").unwrap();
    let insns = BufReader::new(File::open("d18_input.txt").unwrap()).lines().filter_map(|l| l.ok())
        .map(|l| parse_insn(&l, &re))
        .collect::<Vec<_>>();

    part_1(State::new(&insns));
    part_2(&insns);

}