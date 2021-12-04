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
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut one_count = [0; 12];
    let mut zero_count = [0; 12];
    for reading in read_list("inputs/day03.txt") {
        for idx in 0..12 {
            match reading.as_str().chars().nth(idx) {
                Some('1') => one_count[idx] += 1,
                Some('0') => zero_count[idx] += 1,
                _ => ()
            }
        }
    }
    println!("one_count: {:?}", one_count);
    println!("zero_count: {:?}", zero_count);
    for idx in 0..12 {
        let bit = 11 - idx;
        if one_count[idx] > zero_count[idx] {
            gamma |= 1 << bit;
        } else {
            epsilon |= 1 << bit;
        }
    }
    println!("gamma: {:?}", gamma);
    println!("epsilon: {:?}", epsilon);
    println!("epsilon * gamma = {}", gamma * epsilon);
}

fn count_v<T: PartialEq>(i: &Vec<Vec<T>>, bit_pos: usize, value: T) -> usize {
    let mut c = 0;
    for x in i.iter() {
        if x[bit_pos] == value {
            c += 1;
        }
    }
    c
    //i.iter().filter(|&x| *x == value).count()
}

pub fn step2() {
    let mut o2_rating = 0;
    let mut co2_rating = 0;

    // Part 2
    let mut o2_readings: Vec<Vec<_>>  = read_list("inputs/day03.txt").iter()
        .map(|l| l.as_str().chars().collect())
        .collect();

    //let mut co2_readings = o2_readings.clone();
    let mut co2_readings: Vec<Vec<_>>  = read_list("inputs/day03.txt").iter()
        .map(|l| l.as_str().chars().collect())
        .collect();

    let mut test_pos = 0;
    while o2_readings.len() > 1 && test_pos < 12 {
        // annoyance: why aren't vectors their own iterators?
        // e.g. iter/collect would be nice if inferred...
        //o2_readings = o2_readings.iter().filter(|r| criteria(r[test_pos]));
        //
        // annoyance: why can't I pull this closure out of the loop as a var?
        let mut target = '1';
        if count_v(&o2_readings, test_pos, '0') > count_v(&o2_readings, test_pos, '1') {
            target = '0';
        }
        o2_readings.retain(|x| x[test_pos] == target);
        test_pos += 1;
    }

    test_pos = 0;
    while co2_readings.len() > 1 && test_pos < 12 {
        // annoyance: why aren't vectors their own iterators?
        // e.g. iter/collect would be nice if inferred...
        //o2_readings = o2_readings.iter().filter(|r| criteria(r[test_pos]));
        //
        // annoyance: why can't I pull this closure out of the loop as a var?
        let mut target = '0';
        if count_v(&co2_readings, test_pos, '0') > count_v(&co2_readings, test_pos, '1') {
            target = '1';
        }
        co2_readings.retain(|x| x[test_pos] == target);
        test_pos += 1;
        println!("Remain: {}", co2_readings.len());
    }

    for idx in 0..12 {
        let bit = 11 - idx;
        if o2_readings[0][idx] == '1' {
            o2_rating |= 1 << bit;
        }
        if co2_readings[0][idx] == '1' {
            co2_rating |= 1 << bit;
        }
    }
    println!("{:?}", o2_readings);
    println!("{:?}", co2_readings);
    println!("{}. {}. {}", o2_rating, co2_rating, o2_rating * co2_rating);
}
