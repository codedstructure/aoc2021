use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Delta(i32, i32, i32);

impl From<Vec<i32>> for Delta {
    fn from(item: Vec<i32>) -> Self {
        assert!(item.len() == 3);
        Delta(item[0], item[1], item[2])
    }
}

#[derive(Debug, Default)]
struct Scanner {
    beacons: HashSet<Delta>,
}

impl Scanner {
    fn new(lines: Vec<String>) -> Self {
        let mut beacons = HashSet::new();
        for beacon in lines {
            let d: Delta = beacon
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .into();

            beacons.insert(d);
        }
        Self { beacons }
    }
}

#[derive(Debug)]
struct ScannerMap {
    scanners: Vec<Scanner>,
}

impl ScannerMap {
    fn new(filename: &str) -> Self {
        let mut scanners: Vec<Scanner> = vec![];
        let mut delta_lines: Vec<String> = vec![];
        for line in read_list(filename) {
            if line.starts_with("---") {
                delta_lines = vec![];
            } else if line.is_empty() {
                scanners.push(Scanner::new(delta_lines.clone()));
            } else {
                delta_lines.push(line);
            }
        }
        // don't forget the last set of data
        scanners.push(Scanner::new(delta_lines.clone()));

        Self { scanners }
    }
}

pub fn step1() {
    let s = ScannerMap::new("inputs/day19.txt");
    println!("Scanner data: {:?}", s);
    println!("Scanner count: {}", s.scanners.len());
    for b in s.scanners {
        println!(" - beacon count: {}", b.beacons.len());
    }
}

pub fn step2() {}
