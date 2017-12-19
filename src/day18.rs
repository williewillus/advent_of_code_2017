use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread;
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
    Jgz(char, Operand),
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
        "jgz" => Insn::Jgz(ch, parse_opnd(&mats[3])),
        _ => panic!("unknown insn {}", &mats[1]),
    }
}

struct State {
    regs: HashMap<char, i64>,
    pc: i64,
    insns: Vec<Insn>,
    snd_queue: Sender<i64>,
    rcv_queue: Receiver<i64>,
    send_count: u32,
}

impl State {
    fn get_val(&self, opnd: Operand) -> i64 {
        match opnd {
            Operand::Reg(r) => *self.regs.get(&r).unwrap_or(&0),
            Operand::Imm(i) => i,
        }
    }

    fn simulate(&mut self, insn: Insn) -> Option<u32> {
        // I have no idea why I'm getting mismatch arm type errors so here we go () for everyone
        // todo figure out how to force the arms to not return anything
        match insn {
            Insn::Snd(r) => {
                self.send_count += 1;
                self.snd_queue.send(*self.regs.get(&r).unwrap_or(&0)).unwrap();
                ()
            },
            Insn::Set(r, o) => {
                let v = self.get_val(o);
                self.regs.insert(r, v);
                ()
            },
            Insn::Add(r, o) => {
                *self.regs.entry(r).or_insert(0) += self.get_val(o);
                ()
            },
            Insn::Mul(r, o) => {
                *self.regs.entry(r).or_insert(0) *= self.get_val(o);
                ()
            },
            Insn::Mod(r, o) => {
                *self.regs.entry(r).or_insert(0) %= self.get_val(o);
                ()
            },
            Insn::Rcv(ch) => {
                if let Ok(v) = self.rcv_queue.recv_timeout(Duration::new(5, 0)) {
                    self.regs.insert(ch, v);
                } else {
                    println!("recv timeout");
                    return Some(self.send_count);
                }
                ()
            },
            Insn::Jgz(ch, o) => {
                if *self.regs.get(&ch).unwrap_or(&0) > 0 {
                    self.pc += self.get_val(o);
                } else {
                    self.pc += 1;
                }
                ()
            }
        };

        match insn {
            Insn::Jgz(_, _) => (),
            _ => self.pc += 1,
        }

        None
    }
}

pub fn run() {
    let re = Regex::new(r"(snd|set|add|mul|mod|rcv|jgz) ([a-z]|-?\d+)(?: ([a-z]|-?\d+))?").unwrap();
    let insns = BufReader::new(File::open("d18_input.txt").unwrap()).lines().filter_map(|l| l.ok())
        .map(|l| parse_insn(&l, &re))
        .collect::<Vec<_>>();

    let (tx0, rx1) = channel();
    let (tx1, rx0) = channel();

    let mut prog_0 = State {
        regs: HashMap::new(),
        pc: 0,
        insns: insns.clone(),
        snd_queue: tx0,
        rcv_queue: rx0,
        send_count: 0,
    };

    let mut prog_1 = State {
        regs: {
            let mut r = HashMap::new();
            r.insert('p', 1);
            r
        },
        pc: 0,
        insns: insns.clone(),
        snd_queue: tx1,
        rcv_queue: rx1,
        send_count: 0,
    };

    let p0 = thread::spawn(move || {
        while prog_0.pc >= 0 && (prog_0.pc as usize) < prog_0.insns.len() {
            let insn = prog_0.insns[prog_0.pc as usize];
            if let Some(ex) = prog_0.simulate(insn) {
                return ex;
            }
            println!("p0 sends {}", prog_0.send_count);
        }
        return 0;
    });

    let p1 = thread::spawn(move || {
        while prog_1.pc >= 0 && (prog_1.pc as usize) < prog_1.insns.len() {
            let insn = prog_1.insns[prog_1.pc as usize];
            if let Some(ex) = prog_1.simulate(insn) {
                return ex;
            }
            println!("p1 sends {}", prog_1.send_count);
        }
        return 0;
    });

    println!("{:?}", p0.join());
    println!("{:?}", p1.join());
}