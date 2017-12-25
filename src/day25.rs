use std::collections::HashMap;

struct State {
    tape: HashMap<i64, bool>,
    pos: i64,
    state: char,
}

impl State {
    fn cur(&self) -> bool {
        *self.tape.get(&self.pos).unwrap_or(&false)
    }

    fn set_cur(&mut self, v: bool) {
        self.tape.insert(self.pos, v);
    }

    fn checksum(&self) -> usize {
        self.tape.values().filter(|v| **v).count()
    }

    fn simulate(&mut self) {
        let c = !self.cur();
        match self.state {
            'A' => {
                if c {
                    self.set_cur(true);
                    self.pos += 1;
                    self.state = 'B';
                } else {
                    self.set_cur(false);
                    self.pos -= 1;
                    self.state = 'C';
                }
            },
            'B' => {
                if c {
                    self.set_cur(true);
                    self.pos -= 1;
                    self.state = 'A';
                } else {
                    self.set_cur(true);
                    self.pos -= 1;
                    self.state = 'D';
                }
            },
            'C' => {
                if c {
                    self.set_cur(true);
                    self.pos += 1;
                    self.state = 'D';
                } else {
                    self.set_cur(false);
                    self.pos += 1;
                    self.state = 'C';
                }
            },
            'D' => {
                if c {
                    self.set_cur(false);
                    self.pos -= 1;
                    self.state = 'B';
                } else {
                    self.set_cur(false);
                    self.pos += 1;
                    self.state = 'E';
                }
            },
            'E' => {
                if c {
                    self.set_cur(true);
                    self.pos += 1;
                    self.state = 'C';
                } else {
                    self.set_cur(true);
                    self.pos -= 1;
                    self.state = 'F';
                }
            },
            'F' => {
                if c {
                    self.set_cur(true);
                    self.pos -= 1;
                    self.state = 'E';
                } else {
                    self.set_cur(true);
                    self.pos += 1;
                    self.state = 'A';
                }
            },
            _ => panic!("unknown state {}", self.state),
        }
    }
}


pub fn run() {
    let mut state = State {
        tape: HashMap::new(),
        pos: 0,
        state: 'A',
    };

    for _ in 0..12172063 {
        state.simulate();
    }

    println!("{}", state.checksum());
}