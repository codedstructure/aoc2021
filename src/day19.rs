use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Delta(i32, i32, i32);

impl From<Vec<i32>> for Delta {
    fn from(item: Vec<i32>) -> Self {
        assert!(item.len() == 3);
        Delta(item[0], item[1], item[2])
    }
}

impl Delta {
    fn translate(&self, other: &Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }

    fn delta(&self, other: &Self) -> Self {
        Self(other.0 - self.0, other.1 - self.1, other.2 - self.2)
    }

    fn rotate(&self, dir: i32) -> Self {
        match dir {
            0 => Delta(self.0, self.1, self.2),
            1 => Delta(self.1, -self.0, self.2),
            2 => Delta(-self.0, -self.1, self.2),
            3 => Delta(-self.1, self.0, self.2),

            4 => Delta(self.0, self.2, -self.1),
            5 => Delta(self.2, -self.0, -self.1),
            6 => Delta(-self.0, -self.2, -self.1),
            7 => Delta(-self.2, self.0, -self.1),

            8 => Delta(self.2, self.1, -self.0),
            9 => Delta(self.1, -self.2, -self.0),
            10 => Delta(-self.2, -self.1, -self.0),
            11 => Delta(-self.1, self.2, -self.0),

            12 => Delta(self.0, -self.1, -self.2),
            13 => Delta(-self.1, -self.0, -self.2),
            14 => Delta(-self.0, self.1, -self.2),
            15 => Delta(self.1, self.0, -self.2),

            16 => Delta(-self.2, self.1, self.0),
            17 => Delta(self.1, self.2, self.0),
            18 => Delta(self.2, -self.1, self.0),
            19 => Delta(-self.1, -self.2, self.0),

            20 => Delta(self.2, self.0, self.1),
            21 => Delta(self.0, -self.2, self.1),
            22 => Delta(-self.2, -self.0, self.1),
            23 => Delta(-self.0, self.2, self.1),
            _ => self.clone(),
        }
    }

    fn mdist(&self, other: &Self) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
struct Scanner {
    beacons: HashSet<Delta>,

    ident: i32,
}

impl Scanner {
    fn new(lines: Vec<String>, ident: i32) -> Self {
        let mut beacons = HashSet::new();
        for beacon in lines {
            let d: Delta = beacon
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .into();

            beacons.insert(d);
        }
        Self { beacons, ident }
    }

    fn rotate(&self, dir: i32) -> Self {
        let mut beacons = HashSet::new();
        for beacon in &self.beacons {
            beacons.insert(beacon.rotate(dir));
        }
        Self {
            beacons,
            ident: self.ident,
        }
    }

    fn translate(&self, delta: &Delta) -> Self {
        let mut beacons = HashSet::new();
        for beacon in &self.beacons {
            beacons.insert(beacon.translate(delta));
        }
        Self {
            beacons,
            ident: self.ident,
        }
    }

    fn count_matches(&self, other: &Scanner) -> i32 {
        let mut count = 0;
        for this_b in &self.beacons {
            for that_b in &other.beacons {
                if this_b == that_b {
                    count += 1;
                }
            }
        }
        count
    }

    fn overlaps(&self, other: &Scanner) -> Option<(Scanner, Delta)> {
        // are there at least 12 beacons in `other` which could overlap?
        for dir in 0..24 {
            let rot_scanner = other.rotate(dir);
            for beacon in &self.beacons {
                for other_beacon in &rot_scanner.beacons {
                    if beacon == other_beacon {
                        continue;
                    }
                    let beacon_delta = other_beacon.delta(beacon);
                    let offset_scanner = rot_scanner.translate(&beacon_delta);

                    if self.count_matches(&offset_scanner) >= 12 {
                        return Some((offset_scanner, beacon_delta));
                    }
                }
            }
        }
        None
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
        let mut s_id = 0;
        for line in read_list(filename) {
            if line.starts_with("---") {
                delta_lines = vec![];
            } else if line.is_empty() {
                scanners.push(Scanner::new(delta_lines.clone(), s_id));
                s_id += 1;
            } else {
                delta_lines.push(line);
            }
        }
        // don't forget the last set of data
        scanners.push(Scanner::new(delta_lines.clone(), s_id));

        Self { scanners }
    }

    fn build_map(&self) -> (HashMap<i32, Scanner>, HashSet<Delta>) {
        let mut sm_fixed = HashMap::new();
        sm_fixed.insert(0, self.scanners[0].clone());
        let mut offsets = HashSet::new();

        while sm_fixed.len() < self.scanners.len() {
            for o in sm_fixed.clone().values() {
                for s in &self.scanners {
                    if sm_fixed.contains_key(&s.ident) {
                        continue;
                    }
                    if let Some((fixed, offset)) = o.overlaps(s) {
                        // println!("Got overlap {} with {} at offset {:?}",
                        //          o.ident, s.ident, &offset);
                        sm_fixed.insert(s.ident, fixed);
                        offsets.insert(offset);
                    }
                }
            }
        }
        (sm_fixed, offsets)
    }
}

pub fn step1() {
    let sm = ScannerMap::new("inputs/day19.txt");

    let (fixed, _) = sm.build_map();

    let mut beacons_fixed = HashSet::new();
    for smf in fixed.values() {
        for bf in &smf.beacons {
            beacons_fixed.insert(bf);
        }
    }
    println!("Beacon count: {}", beacons_fixed.len());
}

pub fn step2() {
    // yes it is silly rebuilding this again for part2 since it's slow,
    // but I'm going for consistency of the 'framework'... :)
    let sm = ScannerMap::new("inputs/day19.txt");

    let (_, offsets) = sm.build_map();

    let mut max_distance = 0;
    for s1 in &offsets {
        for s2 in &offsets {
            if s1 == s2 {
                continue;
            }
            let distance = s1.mdist(s2);
            if distance > max_distance {
                max_distance = distance;
            }
        }
    }
    println!("Max distance: {}", max_distance);
}
