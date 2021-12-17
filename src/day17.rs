use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug, Default)]
struct Probe {
    xpos: i32,
    ypos: i32,

    xvel: i32,
    yvel: i32,

    targetx: (i32, i32),
    targety: (i32, i32),
    max_height: i32,
}

impl Probe {
    fn new(filename: &str) -> Self {
        let desc = &read_list(filename)[0];

        let mut parts = desc.split(' ');
        parts.next(); // 'target'
        parts.next(); // 'area:'
        let tx = parts.next().unwrap(); // xx..xx,
        let ty = parts.next().unwrap(); // yy..yy
        Self {
            xpos: 0,
            ypos: 0,
            xvel: 0,
            yvel: 0,
            targetx: Probe::tsplit(tx),
            targety: Probe::tsplit(ty),
            max_height: 0,
        }
    }

    fn tsplit(t: &str) -> (i32, i32) {
        // Yes, this really should be regex, but that's not in std...
        let (_, mut range) = t.split_once('=').unwrap();
        range = range.trim_end_matches(',');

        let mut rp = range.split("..");
        let a: i32 = rp.next().unwrap().parse().unwrap();
        let b: i32 = rp.next().unwrap().parse().unwrap();
        // we want smallest *absolute* value provided first
        if a.abs() < b.abs() {
            (a, b)
        } else {
            (b, a)
        }
    }

    fn inrange(&self) -> bool {
        // targety values are negative, first is always abs-smaller.
        self.xpos >= self.targetx.0
            && self.xpos <= self.targetx.1
            && self.ypos <= self.targety.0
            && self.ypos >= self.targety.1
    }

    fn beyond(&self) -> bool {
        self.ypos <= self.targety.1 || self.xpos >= self.targetx.1
    }

    fn step(&mut self) {
        self.xpos += self.xvel;
        self.ypos += self.yvel;
        self.xvel -= self.xvel.signum(); // drag
        self.yvel -= 1; // gravity

        if self.ypos >= self.max_height {
            self.max_height = self.ypos;
        }
    }

    fn reset(&mut self, xv: i32, yv: i32) {
        self.xpos = 0;
        self.ypos = 0;
        self.xvel = xv;
        self.yvel = yv;
        self.max_height = 0;
    }

    fn iterate(&mut self, xv: i32, yv: i32) -> bool {
        self.reset(xv, yv);
        loop {
            self.step();
            if self.inrange() {
                break true;
            }
            if self.beyond() {
                break false;
            }
        }
    }

    fn search(&mut self) -> i32 {
        let mut max_height = 0;
        // Cover search space - 1000 as max yvel is a hack.
        for xv in 0..self.targetx.1 + 1 {
            for yv in self.targety.1..1000 {
                if self.iterate(xv, yv) && self.max_height > max_height {
                    max_height = self.max_height;
                }
            }
        }
        max_height
    }

    fn count_good(&mut self) -> i32 {
        let mut count = 0;
        // Cover search space - 1000 as max yvel is a hack.
        for xv in 0..self.targetx.1 + 1 {
            for yv in self.targety.1..1000 {
                if self.iterate(xv, yv) {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn step1() {
    let mut probe = Probe::new("inputs/day17.txt");

    // 5151
    println!("Max height: {}", probe.search());
}

pub fn step2() {
    let mut probe = Probe::new("inputs/day17.txt");

    // 968
    println!("Count: {}", probe.count_good());
}
