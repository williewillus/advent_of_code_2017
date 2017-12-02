mod day1;
mod day2;
mod util;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args[1].as_str() {
        "1" => day1::run(),
        "2" => day2::run(),
        _ => panic!("no such day {}", args[1])
    }
}
