use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug)]
struct NavSystem {
    lines: Vec<String>,
}

impl NavSystem {
    fn new(filename: &str) -> Self {
        Self {
            lines: read_list(filename),
        }
    }

    // given line, return remaining stack and optional illegal character
    fn preprocess(&self, line: &str) -> (Vec<char>, Option<char>) {
        let mut stack = vec![];
        let mut illegal = None;
        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                // These following patterns are painful...
                ')' => {
                    if stack.pop() != Some('(') {
                        illegal = Some(ch);
                        break;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        illegal = Some(ch);
                        break;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        illegal = Some(ch);
                        break;
                    }
                }
                '>' => {
                    if stack.pop() != Some('<') {
                        illegal = Some(ch);
                        break;
                    }
                }
                _ => (),
            }
        }

        (stack, illegal)
    }

    fn corrupted_line_score(&self, line: &str) -> Option<i128> {
        if let Some(illegal) = self.preprocess(line).1 {
            return match illegal {
                ')' => Some(3),
                ']' => Some(57),
                '}' => Some(1197),
                '>' => Some(25137),
                _ => None,
            }
        }
        None
    }

    fn line_score(&self, line: &str) -> Option<i128> {
        let (mut stack, illegal) = self.preprocess(line);
        if illegal.is_some() {
            return None;
        }

        // remaining stack is what needs completing.
        let mut score = 0;
        while let Some(ch) = stack.pop() {
            score *= 5;
            match ch {
                '(' => score += 1,
                '[' => score += 2,
                '{' => score += 3,
                '<' => score += 4,
                _ => (),
            }
        }
        Some(score)
    }

    fn syntax_error_score(&self) -> i128 {
        let mut total = 0;
        for line in &self.lines {
            if let Some(score) = self.corrupted_line_score(line) {
                total += score;
            }
        }
        total
    }

    fn autocomplete_score(&self) -> i128 {
        let mut scores: Vec<i128> = self
            .lines
            .iter()
            .map(|l| self.line_score(l))
            .flatten()  // Rust TIL - remove Nones & unwrap Somes
            .collect();

        // return median value
        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}

pub fn step1() {
    let nav = NavSystem::new("inputs/day10.txt");

    println!("{}", nav.syntax_error_score());
}

pub fn step2() {
    let nav = NavSystem::new("inputs/day10.txt");

    println!("{}", nav.autocomplete_score());
}
