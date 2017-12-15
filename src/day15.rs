struct Generator {
    factor: u64,
    next: u64,
}

impl Generator {
    fn new(factor: u64, start: u64) -> Generator {
        Generator {
            factor,
            next: (start * factor).wrapping_rem(2147483647),
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = Some(self.next);
        self.next = (self.next * self.factor).wrapping_rem(2147483647);
        ret
    }
}

const A_FACTOR: u64 = 16807;
const B_FACTOR: u64 = 48271;

// todo that probably won't happen bc lazy: parse from input file instead
const A_START: u64 = 512;
const B_START: u64 = 191;

fn compute(p1: bool) {
    let a = Generator::new(A_FACTOR, A_START).filter(|v| p1 || (*v & 0b11) == 0);
    let b = Generator::new(B_FACTOR, B_START).filter(|v| p1 || (*v & 0b111) == 0);

    println!("part {}: {}",
             if p1 { "1" } else { "2" },
             a.zip(b).take(if p1 { 40000000 } else { 5000000 }).filter(|&(l, r)| (l & 0xFFFF) == (r & 0xFFFF)).count());
}

pub fn run() {
    compute(true);
    compute(false);
}
