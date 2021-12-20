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

        // Rust: I really want to use range -1 .. len + 2, but -1 isn't a
        // valid usize, and slices operate on that rather than e.g. i32
        for r in 0..self.pixels.len() + 2 {
            let mut new_row = vec![];
            for c in 0..self.line_width + 2 {
                let index = self.surround_value(r, c);
                let algo_result = self.algorithm[index];
                new_row.push(algo_result);
            }
            pixels.push(new_row);
        }

        if self.background == 0u8 && algorithm[0] == 1 {
            // the inifinite plain is 'dark', but won't be next time
            background = 1u8;
        }
        if self.background == 1u8 && self.algorithm[511] == 0 {
            // the inifinite plain is 'lit', but won't be next time
            background = 0u8;
        }

        Self {
            pixels,
            algorithm,
            background,
            line_width,
        }
    }

    fn surround_value(&self, row: usize, col: usize) -> usize {
        // Note: row/col both have +1 offsets, which is why we do the -2
        // rather than just -1 on ri / ci below...
        // Yes this is hacky and ties it to `enhance()`, but avoids having
        // to do the usize/i32 hack in two places. I think.
        let mut v: usize = 0;
        for r in row..row + 3 {
            // Rust: I want to iterate x-1..=x+1 and use that in a Vec::get(),
            // both of which only operate on usize. Convert to i32 for fixing
            // up, then back to usize in the get. Maybe there's a better way?
            let ri: i32 = (r as i32) - 2;
            // Rust question: not sure about this, I want to create the
            // empty vec on-demand, but that's subject to a 'temporary
            // freed while still in use' error. The clone fixes that, but
            // feels very hacky, since most of the time it is unnecessary.
            let line = self
                .pixels
                .get(ri as usize)
                .unwrap_or(&vec![self.background; self.line_width])
                .clone();
            for c in col..col + 3 {
                let ci = (c as i32) - 2;
                let p = line.get(ci as usize).unwrap_or(&self.background);
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
