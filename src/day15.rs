use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug)]
struct RiskMaze {
    risk: Vec<Vec<i32>>,

    line_width: usize,
}

impl RiskMaze {
    fn new(filename: &str) -> Self {
        let mut risk = vec![];
        let mut line_width = 0;
        for line in read_list(filename) {
            line_width = line.len(); // don't care about repeated setting
            risk.push(
                line.chars()
                    .map(|x| x.to_string().parse::<i32>().unwrap())
                    .collect(),
            );
        }
        Self { risk, line_width }
    }

    fn expand(&mut self) {
        let mut new_grid = vec![];
        for row in 0..5 * self.risk.len() {
            let mut new_row = vec![];
            for col in 0..5 * self.line_width {
                let section = col / self.line_width + row / self.risk.len();
                let mut raw = self.risk[row % self.risk.len()][col % self.line_width];
                for _ in 0..section {
                    raw += 1;
                    if raw > 9 {
                        raw = 1;
                    }
                }
                new_row.push(raw);
            }
            new_grid.push(new_row);
        }
        self.risk = new_grid;
        self.line_width *= 5;
    }

    fn neighbours(&self, pos: (usize, usize)) -> HashSet<(usize, usize)> {
        let mut n = HashSet::new();
        let row = pos.0;
        let col = pos.1;
        if row >= 1 {
            n.insert((row - 1, col));
        }
        if row < self.risk.len() - 1 {
            n.insert((row + 1, col));
        }
        if col >= 1 {
            n.insert((row, col - 1));
        }
        if col < self.line_width - 1 {
            n.insert((row, col + 1));
        }

        n
    }

    fn _broken_populate(&self) -> i32 {
        // This was an earlier attempt but fails with the expanded set.
        // I've left it here for fun - maybe it can be fixed one day, but
        // I think that would just be re-discovering Dijkstra's algorithm.
        let mut boundary = HashSet::new();
        let mut already = HashSet::new();
        let start = (0, 0);
        let target = (self.risk.len() - 1, self.line_width - 1);
        already.insert(start);
        let mut risk = HashMap::new();
        risk.insert(start, 0);
        boundary.insert((1, 0));
        boundary.insert((0, 1));
        while risk.get(&target).is_none() {
            // What cells are adjacent to our current boundary?
            let mut new_boundary = HashSet::new();
            for b in &boundary {
                new_boundary.extend(
                    self.neighbours(*b)
                        .difference(&already)
                        .copied()
                        .collect::<HashSet<(usize, usize)>>(),
                );
            }

            // determine min risk for each item in new_boundary and add to risk
            for pos in boundary.clone() {
                let pos_risk = self.risk[pos.0][pos.1];

                let mut min_risk = 99999999;
                for n in self.neighbours(pos) {
                    if already.contains(&n) {
                        let candidate = risk.get(&n).unwrap();
                        if candidate < &min_risk {
                            min_risk = *candidate;
                        }
                    }
                }
                let pos_min_risk = min_risk + pos_risk;
                if let Some(t) = risk.get(&pos) {
                    if pos_min_risk < *t {
                        risk.insert(pos, pos_min_risk);
                    }
                } else {
                    risk.insert(pos, pos_min_risk);
                }
            }
            already.extend(&boundary);
            boundary = new_boundary;
        }

        *risk.get(&target).unwrap()
    }

    fn bellman_ford(&self) -> i32 {
        // This is just easier than Dijkstra, and I don't care about
        // runtime speed too much.
        let start = (0, 0);
        let target = (self.risk.len() - 1, self.line_width - 1);
        let mut risk = HashMap::new();

        for row in 0..self.risk.len() {
            for col in 0..self.line_width {
                risk.insert((row, col), 99999);
            }
        }
        risk.insert(start, 0);

        let mut keep_going;
        loop {
            keep_going = false;
            for row in 0..self.risk.len() {
                for col in 0..self.line_width {
                    let a = (row, col);
                    for b in self.neighbours((row, col)) {
                        let w = self.risk[b.0][b.1];
                        let min_dist = risk.get(&a).unwrap() + w;
                        if min_dist < *risk.get(&b).unwrap() {
                            risk.insert(b, min_dist);
                            keep_going = true;
                        }
                    }
                }
            }
            if !keep_going {
                break;
            }
        }

        *risk.get(&target).unwrap()
    }
}

pub fn step1() {
    let rm = RiskMaze::new("inputs/day15.txt");
    // 602
    println!("{}", rm.bellman_ford());
}

pub fn step2() {
    let mut rm = RiskMaze::new("inputs/day15.txt");
    rm.expand();
    // 2935
    println!("{}", rm.bellman_ford());
}
