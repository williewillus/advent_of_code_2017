mod day1;
mod day2;
mod day3;
mod day4;
mod util;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("please give day");
        return;
    }

    match args[1].as_str() {
        "1" => day1::run(),
        "2" => day2::run(),
        "3" => day3::run(),
        "4" => day4::run(),
        _ => panic!("no such day {}", args[1])
    }
}
