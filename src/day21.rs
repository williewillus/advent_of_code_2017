use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use pathfinding::Matrix;
use itertools::Itertools;
use itertools::iterate;

fn to_matrix(side: &str) -> Matrix<bool> {
    Matrix::square_from_vec(
        side.bytes()
            .filter(|b| *b != b'/')
            .map(|b| b == b'#')
            .collect())
}

fn to_rules(line: &str) -> Vec<(Matrix<bool>, Matrix<bool>)> { // todo can I use a fixed-size array? it messes with flat_map below
    let (left, right) = line.split(" => ").next_tuple().unwrap();

    let pat = to_matrix(left);
    let result = to_matrix(right);

    vec![
        (pat.flipped_lr(), result.clone()),
        (pat.flipped_lr().rotated_cw(1), result.clone()),
        (pat.flipped_lr().rotated_cw(2), result.clone()),
        (pat.flipped_lr().rotated_cw(3), result.clone()),

        (pat.flipped_ud(), result.clone()),
        (pat.flipped_ud().rotated_cw(1), result.clone()),
        (pat.flipped_ud().rotated_cw(2), result.clone()),
        (pat.flipped_ud().rotated_cw(3), result.clone()),

        (pat.rotated_cw(3), result.clone()),
        (pat.rotated_cw(2), result.clone()),
        (pat.rotated_cw(1), result.clone()),
        (pat, result),
    ]
}

fn enhance(old: &Matrix<bool>, rules: &HashMap<Matrix<bool>, Matrix<bool>>) -> Matrix<bool> {
    if old.rows % 2 == 0 {
        let old_chunks = old.rows / 2;
        let new_grid_size = old_chunks * 3;
        let mut new_grid = Matrix::new_square(new_grid_size, false);

        for chunk_y in 0..old_chunks {
            for chunk_x in 0..old_chunks {
                let old_chunk = old.slice(chunk_x*2..chunk_x*2 + 2, chunk_y*2..chunk_y*2 + 2);
                let res = &rules[&old_chunk];
                new_grid.set_slice(&(chunk_x*3, chunk_y*3), &res);
            }
        }

        new_grid
    } else {
        assert_eq!(0, old.rows % 3);

        let old_chunks = old.rows / 3;
        let new_grid_size = old_chunks * 4;
        let mut new_grid = Matrix::new_square(new_grid_size, false);

        for chunk_y in 0..old_chunks {
            for chunk_x in 0..old_chunks {
                let old_chunk = old.slice(chunk_x*3..chunk_x*3 + 3, chunk_y*3..chunk_y*3 + 3);
                let res = &rules[&old_chunk];
                new_grid.set_slice(&(chunk_x*4, chunk_y*4), &res);
            }
        }

        new_grid
    }
}

pub fn run() {
    let rules = BufReader::new(File::open("d21_input.txt").unwrap()).lines().filter_map(|l| l.ok())
        .flat_map(|l| to_rules(&l).into_iter())
        .collect::<HashMap<_, _>>();

    let init = Matrix::square_from_vec(vec![false, true, false, false, false, true, true, true, true]);
    // todo use itertools.iterate() when this actually works
    let mut iter = iterate(init, |s| enhance(s, &rules));
    println!("part 1: {}", iter.nth(5).unwrap().as_ref().iter().filter(|b| **b).count());
    // another 12 to get the 18th iteration
    println!("part 2: {}", iter.nth(12).unwrap().as_ref().iter().filter(|b| **b).count());
}
