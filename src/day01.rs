use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

pub fn read_int_list(filename: &str) -> Result<Vec<i32>, Error> {
    let f = File::open(filename)?;
    Ok(BufReader::new(f)
        .lines()
        .map(|l| l.expect("Err"))
        .map(|l| l.parse::<i32>().unwrap())
        .collect())
}

pub fn step1() {
    let mut count = 0;
    let mut current = None;
    for value in read_int_list("inputs/day01.txt").unwrap() {
        if let Some(old_val) = current {
            if old_val < value {
                count += 1;
            }
        }
        current = Some(value);
    }
    println!("Increment count: {}", count);
}

pub fn step2() {
    let mut count = 0;
    let mut previous = 0;
    let mut current;
    let mut window: Vec<i32> = vec![];
    for value in read_int_list("inputs/day01.txt").unwrap() {
        if window.len() < 3 {
            window.push(value);
            continue;
        } else {
            window.remove(0); // fine for a 3-element list
            window.push(value);
        }
        assert!(window.len() == 3);
        current = window.iter().sum();
        if current > previous {
            count += 1;
        }
        previous = current;
    }
    count -= 1; // because we shouldn't count the first 0->anything transition
    println!("Windowed Increment count: {}", count);
}
