use std::fs::File;
use std::io::Read;
use std::io::BufReader;

pub fn read_all(path: &str) -> Option<String> {
    let f = File::open(path);

    if f.is_ok() {
        let mut input = String::new();
        let mut rdr = BufReader::new(f.unwrap());

        if rdr.read_to_string(&mut input).is_ok() {
            return Some(input);
        }
    }

    return None;
}

#[derive(Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {
    pub fn ccw(&self) -> Direction {
        match self {
            &Direction::UP => Direction::LEFT,
            &Direction::LEFT => Direction::DOWN,
            &Direction::DOWN => Direction::RIGHT,
            &Direction::RIGHT => Direction::UP,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Position(pub i32, pub i32);
impl Position {
    pub fn offset(&self, dir: &Direction) -> Position {
        match dir {
            &Direction::UP => Position(self.0, self.1 + 1),
            &Direction::DOWN => Position(self.0, self.1 - 1),
            &Direction::LEFT => Position(self.0 - 1, self.1),
            &Direction::RIGHT => Position(self.0 + 1, self.1),
        }
    }
}
