use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug, Clone)]
struct Image {
    pixels: Vec<Vec<u8>>,
    algorithm: Vec<u8>,
    background: u8,
    line_width: usize,
}

impl Image {
    fn new(filename: &str) -> Self {
        let mut pixels = vec![];
        let mut algorithm = vec![];
        let background = 0u8;
        let mut line_width = 0;

        for line in read_list(filename) {
            if line.is_empty() {
                continue;
            }
            if algorithm.is_empty() {
                algorithm = line
                    .chars()
                    .map(|x| if x == '#' { 1u8 } else { 0u8 })
                    .collect();
                continue;
            }

            line_width = line.len(); // don't care about repeated setting
            pixels.push(
                line.chars()
                    .map(|x| if x == '#' { 1u8 } else { 0u8 })
                    .collect(),
            );
        }

        Self {
            pixels,
            algorithm,
            background,
            line_width,
        }
    }

    fn enhance(&self) -> Self {
        let mut pixels = vec![];
        let algorithm = self.algorithm.clone();
        let line_width = self.line_width + 2;

        let mut background = self.background;

        // evaluate new image including a 1 pixel border around current extent
        for r in -1..=self.pixels.len() as i32 + 1 {
            let mut new_row = vec![];
            for c in -1..self.line_width as i32 + 1 {
                let index = self.surround_value(r, c);
                let algo_result = self.algorithm[index];
                new_row.push(algo_result);
            }
            pixels.push(new_row);
        }

        // Check what happens at an arbitrary point a long way away from our
        // image, represented by the `background` value.
        if self.background == 0u8 && algorithm[0] == 1 {
            // algorithm 0 is used when self & surrounding are all dark.
            // the infinite plane is 'dark', but won't be next time
            background = 1u8;
        }
        if self.background == 1u8 && self.algorithm[511] == 0 {
            // algorithm 511 is used when self & surrounding are all lit
            // the infinite plane is 'lit', but won't be next time
            background = 0u8;
        }

        Self {
            pixels,
            algorithm,
            background,
            line_width,
        }
    }

    fn surround_value(&self, row: i32, col: i32) -> usize {
        let mut v: usize = 0;
        for r in row - 1..=row + 1 {
            // Rust question: not sure about this, I want to create the
            // empty vec on-demand, but that's subject to a 'temporary
            // freed while still in use' error. The clone fixes that, but
            // feels very hacky, since most of the time it is unnecessary.
            let line = self
                .pixels
                .get(r as usize)
                .unwrap_or(&vec![self.background; self.line_width])
                .clone();
            for c in col - 1..=col + 1 {
                let p = line.get(c as usize).unwrap_or(&self.background);
                v = (v << 1) | *p as usize;
            }
        }
        v
    }

    fn count_lit(&self) -> usize {
        let mut count = 0;
        for r in &self.pixels {
            for p in r {
                if *p == 1 {
                    count += 1;
                }
            }
        }
        count
    }

    #[allow(dead_code)]
    fn display(&self) {
        for line in &self.pixels {
            for p in line {
                print!("{}", if *p == 1 { '#' } else { '.' });
            }
            println!();
        }
    }
}

pub fn step1() {
    let mut im = Image::new("inputs/day20.txt");

    im = im.enhance();
    im = im.enhance();
    println!("Lit pixels {}", im.count_lit());
}

pub fn step2() {
    let mut im = Image::new("inputs/day20.txt");

    for _ in 0..50 {
        im = im.enhance();
    }
    println!("Lit pixels {}", im.count_lit());
}
