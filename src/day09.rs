use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug)]
struct HeightMap {
    height: Vec<Vec<u8>>,

    line_width: usize,
}

impl HeightMap {
    fn new(filename: &str) -> Self {
        let mut height = vec![];
        let mut line_width = 0;
        for line in read_list(filename) {
            line_width = line.len(); // don't care about repeated setting
            height.push(
                line.chars()
                    .map(|x| x.to_string().parse::<u8>().unwrap())
                    .collect(),
            );
        }
        Self { height, line_width }
    }

    fn neighbours(&self, row: usize, col: usize) -> Vec<u8> {
        let mut n = vec![];
        if row >= 1 {
            n.push(self.height[row - 1][col]);
        }
        if row < self.height.len() - 1 {
            n.push(self.height[row + 1][col]);
        }
        if col >= 1 {
            n.push(self.height[row][col - 1]);
        }
        if col < self.line_width - 1 {
            n.push(self.height[row][col + 1]);
        }

        n
    }

    fn low_points(&self) -> Vec<u8> {
        let mut points = vec![];

        for (row_idx, row) in self.height.iter().enumerate() {
            for (p_idx, point) in row.iter().enumerate() {
                if self.neighbours(row_idx, p_idx).iter().all(|x| x > point) {
                    // Rust quibble - why do I have to dereference point for
                    // push, but not for the comparison above?
                    points.push(*point);
                }
            }
        }

        points
    }

    fn risk_level(&self) -> usize {
        let mut risk: usize = 0;
        for p in self.low_points() {
            // Rust quibble - 'widening' implicit numeric coercions would be nice
            risk += (p + 1) as usize;
        }

        risk
    }

    fn neighbours_full(&self, row: usize, col: usize) -> Vec<(usize, usize, u8)> {
        let mut n = vec![];
        if row >= 1 {
            n.push((row - 1, col, self.height[row - 1][col]));
        }
        if row < self.height.len() - 1 {
            n.push((row + 1, col, self.height[row + 1][col]));
        }
        if col >= 1 {
            n.push((row, col - 1, self.height[row][col - 1]));
        }
        if col < self.line_width - 1 {
            n.push((row, col + 1, self.height[row][col + 1]));
        }

        n
    }

    fn descend(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        // descend from given point to find the associated low-point coords
        // implied that this will always be unique for our input.
        let value = self.height[row][col];
        if value == 9 {
            return None;
        }
        // find where we end up by recursively heading in (any) 'lower'
        // direction. All will converge on the same lowest point.
        let lower: Option<(usize, usize, u8)> = self
            .neighbours_full(row, col)
            .into_iter() // Rust learning: 'Copy' doesn't mean can forget about refs...
            .min_by_key(|x| x.2);
        if let Some(lowest) = lower {
            if lowest.2 < value {
                return self.descend(lowest.0, lowest.1);
            }
        }

        Some((row, col))
    }

    fn find_basin_sizes(&self) -> HashMap<(usize, usize), i32> {
        let mut map = HashMap::new();

        for row in 0..self.height.len() {
            for col in 0..self.line_width {
                if let Some(low_rc) = self.descend(row, col) {
                    let count_at_rc = map.entry(low_rc).or_insert(0);
                    *count_at_rc += 1;
                }
            }
        }

        map
    }

    fn biggest_basin_mult(&self) -> i32 {
        let sizes = self.find_basin_sizes();

        let mut sorted = sizes.keys().into_iter().collect::<Vec<&(usize, usize)>>();

        // sort by descending key and take product of first three entries
        sorted.sort_by_key(|x| -sizes.get(x).unwrap());

        let mut result = 1;
        for idx in 0..3 {
            result *= sizes[sorted[idx]];
        }

        result
    }
}

pub fn step1() {
    let hm = HeightMap::new("inputs/day09.txt");

    println!("{}", hm.risk_level());
}

pub fn step2() {
    let hm = HeightMap::new("inputs/day09.txt");

    println!("{}", hm.biggest_basin_mult());
}
