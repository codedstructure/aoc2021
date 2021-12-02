use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f)
        .lines()
        .map(|l| l.expect("Err"))
        .collect()
}

pub fn step1() {
    let mut xpos = 0;
    let mut depth = 0;
    for instr in read_list("inputs/day02.txt") {
        let mut instr_iter = instr.split_whitespace();
        let movement = instr_iter.next().unwrap();
        let amount: i32 = instr_iter.next().unwrap().parse().unwrap();
        match movement {
            "up" => depth -= amount,
            "down" => depth += amount,
            "forward" => xpos += amount,
            _ => ()
        }
    }
    println!("xpos * depth = {}", xpos * depth);
}

pub fn step2() {
    let mut xpos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for instr in read_list("inputs/day02.txt") {
        let mut instr_iter = instr.split_whitespace();
        let movement = instr_iter.next().unwrap();
        let amount: i32 = instr_iter.next().unwrap().parse().unwrap();
        match movement {
            "up" => aim -= amount,
            "down" => aim += amount,
            "forward" => {
                xpos += amount;
                depth += aim * amount;
            }
            _ => ()
        }
    }
    println!("xpos * depth = {}", xpos * depth);
}
