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


