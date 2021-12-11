use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug)]
struct OctoMap {
    energy: [[u8; 10]; 10],

    flash_count: i32,
}

impl OctoMap {
    fn new(filename: &str) -> Self {
        let mut energy: [[u8; 10]; 10] = Default::default();
        for (idx, line) in read_list(filename).iter().enumerate() {
            energy[idx] = line
                .chars()
                .map(|x| x.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
        }
        Self {
            energy,
            flash_count: 0,
        }
    }

    fn increment(&mut self) {
        for row in 0..10 {
            for col in 0..10 {
                self.energy[row][col] += 1;
            }
        }
    }

    fn increment_neighbours(&mut self, row: usize, col: usize) {
        if row >= 1 {
            self.energy[row - 1][col] += 1;
            if col >= 1 {
                self.energy[row - 1][col - 1] += 1;
            }
            if col < 9 {
                self.energy[row - 1][col + 1] += 1;
            }
        }
        if row < 9 {
            self.energy[row + 1][col] += 1;
            if col >= 1 {
                self.energy[row + 1][col - 1] += 1;
            }
            if col < 9 {
                self.energy[row + 1][col + 1] += 1;
            }
        }
        if col >= 1 {
            self.energy[row][col - 1] += 1;
        }
        if col < 9 {
            self.energy[row][col + 1] += 1;
        }
    }

    fn flashes(&mut self) -> bool {
        let mut flashed = HashSet::new();
        loop {
            let mut flash_occurred = false;

            for row in 0..10 {
                for col in 0..10 {
                    if flashed.contains(&(row, col)) {
                        continue;
                    }

                    if self.energy[row][col] > 9 {
                        flash_occurred = true;
                        flashed.insert((row, col));
                        self.increment_neighbours(row, col);
                        self.flash_count += 1;
                    }
                }
            }

            if !flash_occurred {
                break;
            }
        }

        // was it a synchronized flash of all octopuses?
        flashed.len() == 100
    }

    fn dissipate(&mut self) {
        for row in 0..10 {
            for col in 0..10 {
                if self.energy[row][col] > 9 {
                    self.energy[row][col] = 0;
                }
            }
        }
    }

    fn step(&mut self) -> bool {
        self.increment();
        let result = self.flashes();
        self.dissipate();

        result
    }
}

pub fn step1() {
    let mut om = OctoMap::new("inputs/day11.txt");

    for _ in 0..100 {
        om.step();
    }

    println!("{}", om.flash_count);
}

pub fn step2() {
    let mut om = OctoMap::new("inputs/day11.txt");
    let mut counter = 1; // pesky off-by-one errors...
    loop {
        if om.step() {
            break;
        }
        counter += 1;
    }

    println!("{}", counter);
}
