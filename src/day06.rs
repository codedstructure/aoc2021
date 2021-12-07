use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_csv_ints(filename: &str) -> Vec<usize> {
    let f = File::open(filename).expect("Could not read file");
    let mut line = String::new();
    BufReader::new(f).read_line(&mut line).unwrap();
    // parse() breaks on line ending, so need to trim that...
    line.retain(|c| !c.is_whitespace());
    line.split(',').map(|x| {x.parse::<usize>().unwrap()}).collect()
}

#[derive(Debug)]
struct LanternSim {
    remaining: [usize; 9],
}

impl LanternSim {
    fn new(filename: &str) -> Self {
        let fish = read_csv_ints(filename);
        let mut remaining = [0; 9];
        for f in fish {
            remaining[f] += 1;
        }
        Self {
            remaining
        }
    }

    fn step(&mut self) {
        let mut next_remaining = [0; 9];
        for idx in 0..9 {
            if idx == 0 {
                // spawn fish
                next_remaining[8] += self.remaining[idx];
                next_remaining[6] += self.remaining[idx];
            } else {
                next_remaining[idx - 1] += self.remaining[idx];
            }
        }
        self.remaining = next_remaining;
    }

    fn total_fish(&self) -> usize {
        self.remaining.iter().sum()
    }
}

pub fn step1() {
    let mut sim = LanternSim::new("inputs/day06.txt");

    for _ in 0..80 {
        sim.step();
    }

    println!("Fish: {}", sim.total_fish());
}

pub fn step2() {
    let mut sim = LanternSim::new("inputs/day06.txt");

    for _ in 0..256 {
        sim.step();
    }

    println!("Fish: {}", sim.total_fish());
}
