use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: Point,
    end: Point,

    dx: i32,
    dy: i32,
}

impl Line {
    fn from_line(l: &str) -> Self {
        let mut line_iter = l.split(" -> ");
        let mut first = line_iter
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap());
        let mut second = line_iter
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap());
        let start = Point::new(first.next().unwrap(), first.next().unwrap());
        let end = Point::new(second.next().unwrap(), second.next().unwrap());
        let dx = end.x - start.x;
        let dy = end.y - start.y;

        Self { start, end, dx, dy }
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn span(&self) -> LineSpan {
        LineSpan::new(*self)
    }
}

struct LineSpan {
    line: Line,
    cursor: Option<Point>,
}

impl LineSpan {
    fn new(line: Line) -> Self {
        Self {
            line,
            cursor: Some(line.start),
        }
    }
}

impl Iterator for LineSpan {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.cursor;
        if self.cursor == Some(self.line.end) {
            self.cursor = None;
        } else if let Some(mut cursor) = item {
            cursor.x += self.line.dx.signum();
            cursor.y += self.line.dy.signum();
            self.cursor = Some(cursor);
        }
        item
    }
}

#[derive(Debug)]
struct Grid {
    lines: Vec<Line>,
}

impl Grid {
    fn new(filename: &str, orthogonal: bool) -> Self {
        let mut lines = vec![];
        for line in read_list(filename) {
            let candidate = Line::from_line(&line);
            if !orthogonal || (candidate.is_horizontal() || candidate.is_vertical()) {
                lines.push(candidate);
            }
        }

        Self { lines }
    }

    fn count_danger_points(&self) -> i32 {
        let mut count = 0;
        let mut map = HashMap::new();

        for l in &self.lines {
            for p in l.span() {
                if map.get(&p) == Some(&1) {
                    // this tracks the 1 -> >=2 transition
                    count += 1;
                }
                let count_at_p = map.entry(p).or_insert(0);
                *count_at_p += 1;
            }
        }
        count
    }
}

pub fn step1() {
    let grid = Grid::new("inputs/day05.txt", true);

    println!("Orthogonal danger points: {}", grid.count_danger_points());
}

pub fn step2() {
    let grid = Grid::new("inputs/day05.txt", false);

    println!("All danger points: {}", grid.count_danger_points());
}
