use util;

pub fn run() {
    let mut max= 0.0f64;
    let mut x = 0.0f64;
    let mut y = 0.0f64;
    let s = util::read_all("d11_input.txt").unwrap();

    for dir in s.trim().split(",") {
        match dir {
            "n" => y += 1.0,
            "s" => y -= 1.0,
            "e" => x += 1.0,
            "w" => x -= 1.0,
            "ne" => {
                x += 1.0;
                y += 0.5;
            },
            "se" => {
                x += 1.0;
                y -= 0.5;
            },
            "nw" => {
                x -= 1.0;
                y += 0.5;
            },
            "sw" => {
                x -= 1.0;
                y -= 0.5;
            }
            _ => panic!("unknown dir {}", dir)
        }

        max = max.max(x.abs()).max(y.abs());
    }

    println!("just eyeball it: x: {} y: {} max coord in any direction: {}", x, y, max);
}