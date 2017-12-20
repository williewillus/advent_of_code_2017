use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use regex::Regex;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Particle {
    pos: [i32; 3],
    vel: [i32; 3],
    accel: [i32; 3],
}

impl Particle {
    fn step(&mut self) {
        self.vel[0] += self.accel[0];
        self.vel[1] += self.accel[1];
        self.vel[2] += self.accel[2];

        self.pos[0] += self.vel[0];
        self.pos[1] += self.vel[1];
        self.pos[2] += self.vel[2];
    }
}

fn mag(v: &[i32; 3]) -> f32 {
    let s = v.iter().map(|&i| i * i).sum::<i32>();
    (s as f32).sqrt()
}

fn manhattan(v: &[i32; 3]) -> i32 {
    v[0].abs() + v[1].abs() + v[2].abs()
}

fn comparator(a: &&Particle, b: &&Particle) -> Ordering {
    let accel_cmp = mag(&a.accel).partial_cmp(&mag(&b.accel)).unwrap();
    if accel_cmp != Ordering::Equal {
        accel_cmp
    } else {
        let vel_cmp = mag(&a.vel).partial_cmp(&mag(&b.vel)).unwrap();
        if vel_cmp != Ordering::Equal {
            vel_cmp
        } else {
            manhattan(&a.pos).cmp(&manhattan(&b.pos))
        }
    }
}

fn part_2(mut particles: Vec<Particle>) {
    // pos => HashSet<Particles> ending up in that pos this iteration
    // basically a group_by on pos, which rust doesn't have :(
    let mut groups = HashMap::new();

    // todo actually determine when to break when there are no groups?
    for _ in 0..10000 {
        for p in &mut particles {
            p.step();
            groups.entry(p.pos).or_insert(HashSet::new()).insert(*p);
        }

        for group in groups.values() {
            if group.len() > 1 {
                for p in group {
                    let pos = particles.iter().position(|x| *x == *p).unwrap();
                    particles.remove(pos);
                }
            }
        }

        groups.clear();
    }

    println!("part 2: probably {} (guessed from 10000 iterations)", particles.len());
}

pub fn run() {
    let mut particles = Vec::new();

    let re = Regex::new(r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();
    for line in BufReader::new(File::open("d20_input.txt").unwrap()).lines().filter_map(|l| l.ok()) {
        if let Some(mats) = re.captures(&line) {
            let px = mats[1].parse::<i32>().unwrap();
            let py = mats[2].parse::<i32>().unwrap();
            let pz = mats[3].parse::<i32>().unwrap();

            let vx = mats[4].parse::<i32>().unwrap();
            let vy = mats[5].parse::<i32>().unwrap();
            let vz = mats[6].parse::<i32>().unwrap();

            let ax = mats[7].parse::<i32>().unwrap();
            let ay = mats[8].parse::<i32>().unwrap();
            let az = mats[9].parse::<i32>().unwrap();

            particles.push(Particle {
                pos: [px, py, pz],
                vel: [vx, vy, vz],
                accel: [ax, ay, az],
            });
        }
    }

    // given 0 acceleration, those particles with smallest velocity vector magnitude will stay the closest
    // given constant nonzero acceleration, the velocity will become skewed to extremes in the long run
    // given the same acceleration and velocity, then simply the closest starting particle wins
    // therefore, take the particle with smallest acceleration magnitude, then smallest velocity magnitude, then smallest starting manhattan
    let min = *particles.iter().min_by(comparator).unwrap();
    println!("part 1: Particle {}: {:?}", particles.iter().position(|p| *p == min).unwrap(), min);

    part_2(particles);
}