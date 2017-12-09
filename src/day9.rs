use std::vec::Vec;
use std::iter::Peekable;
use util;

#[derive(Debug)]
struct Ctx {
    score: u32,
    nest_levels: u32,
    garbage_count: u32,
}

fn parse_garbage<I>(iter: &mut Peekable<I>, ctx: &mut Ctx) where I: Iterator<Item = char> {
    assert_eq!(Some('<'), iter.next(), "expected < to start garbage");
    let mut cancellation = false;
    loop {
        match iter.next() {
            Some(c) => {
                match c {
                    '!' => if !cancellation {
                        cancellation = true;
                        continue; // skip over reset to false
                    },
                    '>' => if !cancellation {
                        break;
                    },
                    other => if !cancellation {
                        ctx.garbage_count += 1;
                    },
                }
                cancellation = false
            }
            None => panic!("unexpected end of stream parsing garbage")
        }
    }
}

fn parse_group<I>(iter: &mut Peekable<I>, ctx: &mut Ctx) where I: Iterator<Item = char> {
    assert_eq!(Some('{'), iter.next(), "expected {{ to start group");
    ctx.nest_levels += 1;

    let mut cancellation = false;
    loop {
        match *iter.peek().expect("unexpected end of stream parsing group") {
            '!' => if !cancellation {
                cancellation = true;
                continue; // skip over reset to false
            },
            '{' => if !cancellation {
                parse_group(iter, ctx);
            },
            '}' => if !cancellation {
                iter.next().unwrap(); // consume
                break;
            }
            '<' => if !cancellation {
                parse_garbage(iter, ctx);
            }
            other => {
                iter.next().unwrap(); // consume
                ()
            },
        }
        cancellation = false;
    }

    ctx.score += ctx.nest_levels;
    ctx.nest_levels -= 1;

}

pub fn run() {
    let s = util::read_all("d9_input.txt").unwrap();
    let mut ctx = Ctx {
        score: 0,
        nest_levels: 0,
        garbage_count: 0
    };

    parse_group(&mut s.chars().peekable(), &mut ctx);
    println!("{:?}", ctx);
}
