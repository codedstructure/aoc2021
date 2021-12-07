use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_csv_ints(filename: &str) -> Vec<i32> {
    let f = File::open(filename).expect("Could not read file");
    let mut line = String::new();
    BufReader::new(f).read_line(&mut line).unwrap();
    // parse() breaks on line ending, so need to trim that...
    line.retain(|c| !c.is_whitespace());
    line.split(',').map(|x| x.parse::<i32>().unwrap()).collect()
}

#[derive(Debug)]
struct CrabSumSwarm {
    crabsubs: Vec<i32>,
}

impl CrabSumSwarm {
    fn new(filename: &str) -> Self {
        let crabsubs = read_csv_ints(filename);
        Self { crabsubs }
    }

    fn cost(&self, pos: i32) -> i32 {
        let mut total = 0;
        for cs in &self.crabsubs {
            total += (cs - pos).abs();
        }
        total
    }

    fn cost_2(&self, pos: i32) -> i32 {
        let mut total = 0;
        for cs in &self.crabsubs {
            let dist = (cs - pos).abs();
            total += (dist * (dist + 1)) / 2;
        }
        total
    }

    fn min_cost_naive(&self, step2: bool) -> i32 {
        let mut min_cost = 1_000_000_000; // yes, I know...

        for pos in *self.crabsubs.iter().min().unwrap()..*self.crabsubs.iter().max().unwrap() {
            let pos_cost;
            // This is terrible, would be better to pass a cost function in.
            if step2 {
                pos_cost = self.cost_2(pos);
            } else {
                pos_cost = self.cost(pos);
            }
            if pos_cost < min_cost {
                min_cost = pos_cost;
            }
        }
        min_cost
    }
}

pub fn step1() {
    let subs = CrabSumSwarm::new("inputs/day07.txt");

    println!("Min Cost: {}", subs.min_cost_naive(false));
}

pub fn step2() {
    let subs = CrabSumSwarm::new("inputs/day07.txt");

    println!("Min Cost - new calc: {}", subs.min_cost_naive(true));
}
