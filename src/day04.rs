use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug)]
struct BingoBoard {
    numbers: Vec<i32>,
    // marked is a bitmap - bit 0 is the first (top-left) number
    marked: u32,
}

impl BingoBoard {
    fn new(numbers: Vec<i32>) -> Self {
        Self { numbers, marked: 0 }
    }

    fn complete(&self) -> bool {
        let bingo_mask = vec![
            0b00000_00000_00000_00000_11111,
            0b00000_00000_00000_11111_00000,
            0b00000_00000_11111_00000_00000,
            0b00000_11111_00000_00000_00000,
            0b11111_00000_00000_00000_00000,
            0b00001_00001_00001_00001_00001,
            0b00010_00010_00010_00010_00010,
            0b00100_00100_00100_00100_00100,
            0b01000_01000_01000_01000_01000,
            0b10000_10000_10000_10000_10000,
        ];
        for mask in bingo_mask {
            if self.marked & mask == mask {
                return true;
            }
        }
        false
    }

    fn score(&self, mult: i32) -> i32 {
        let mut total = 0;
        for (idx, num) in self.numbers.iter().enumerate() {
            // sum of unmarked numbers
            if self.marked & (1 << idx) == 0 {
                total += num;
            }
        }
        total * mult
    }

    fn called(&mut self, call: i32) {
        for (idx, &num) in self.numbers.iter().enumerate() {
            if num == call {
                self.marked |= 1 << idx;
            }
        }
    }
}

struct Game {
    boards: Vec<BingoBoard>,
    sequence: Vec<i32>,
}

impl Game {
    fn new(filename: &str) -> Self {
        let mut boards = vec![];
        let mut sequence = vec![];
        let mut board: Vec<i32> = vec![];
        for (idx, line) in read_list(filename).iter().enumerate() {
            if idx == 0 {
                sequence = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
            } else {
                if line.len() == 0 {
                    continue;
                }
                board.extend::<Vec<i32>>(
                    line.split_whitespace()
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect(),
                );
                if board.len() == 25 {
                    boards.push(BingoBoard::new(board.clone()));
                    board.clear();
                }
            }
        }
        Game { boards, sequence }
    }

    fn play(&mut self) -> i32 {
        for call in &self.sequence {
            println!("Calling {}", call);
            for board in &mut self.boards {
                board.called(*call);
                if board.complete() {
                    println!("Bingo!!");
                    println!("{:?}", board);
                    return board.score(*call);
                }
            }
        }
        -1
    }

    fn play_to_lose(&mut self) -> i32 {
        let mut last_score = -1;
        for call in &self.sequence {
            println!("Calling {}", call);
            for board in &mut self.boards {
                if !board.complete() {
                    board.called(*call);
                    if board.complete() {
                        println!("Bingo!!");
                        println!("{:?}", board);
                        last_score = board.score(*call);
                    }
                }
            }
        }
        last_score
    }
}

pub fn step1() {
    let mut game = Game::new("inputs/day04.txt");

    println!("Winning score: {}", game.play());
}

pub fn step2() {
    let mut game = Game::new("inputs/day04.txt");

    println!("Losing score: {}", game.play_to_lose());
}
