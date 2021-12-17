use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_hex_chars(filename: &str) -> Vec<char> {
    let f = File::open(filename).expect("Could not read file");
    let mut line = String::new();
    BufReader::new(f).read_line(&mut line).unwrap();
    line.chars().collect()
}

struct BitIter {
    input: Box<dyn Iterator<Item = char>>,
    current_char: Option<u32>,
    nibble_offset: i32,
}

impl BitIter {
    fn new(input: Vec<char>) -> Self {
        Self {
            input: Box::new(input.into_iter()),
            current_char: None,
            nibble_offset: 0,
        }
    }
}

impl Iterator for BitIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result: Self::Item = 0;
        if self.current_char.is_none() {
            self.current_char = self.input.next().unwrap_or('0').to_digit(16)
        }
        if let Some(x) = self.current_char {
            result = (x as i32 >> (3 - self.nibble_offset)) & 1;
        }
        self.nibble_offset += 1;
        if self.nibble_offset == 4 {
            self.current_char = None;
            self.nibble_offset = 0;
        }
        Some(result)
    }
}

struct PacketReader {
    bits: BitIter,
    total_ver: i128,
    bit_pos: i32,
}

impl PacketReader {
    fn new(input: Vec<char>) -> Self {
        Self {
            bits: BitIter::new(input),
            total_ver: 0,
            bit_pos: 0,
        }
    }

    fn read_packet(&mut self) -> i128 {
        let pver = self.bits_value(3);
        self.total_ver += pver as i128;
        let ptype = self.bits_value(3);
        if ptype == 4 {
            self.read_literal()
        } else {
            self.read_operator(ptype)
        }
    }

    fn read_literal(&mut self) -> i128 {
        let mut literal_value: i128 = 0;
        // read 5-bit blocks and accumulate in literal_value
        loop {
            let x: i128 = self.bits_value(5).into();
            literal_value = (literal_value << 4) | (x & 15);
            if x & 16 != 16 {
                break literal_value;
            }
        }
    }

    fn read_operator(&mut self, ptype: i32) -> i128 {
        let len_type = self.bits_value(1);
        let op_type = match ptype {
            0 => |x, y| x + y,
            1 => |x, y| x * y,
            2 => |x, y| if x < y { x } else { y },
            3 => |x, y| if x > y { x } else { y },
            5 => |x, y| if x > y { 1 } else { 0 },
            6 => |x, y| if x < y { 1 } else { 0 },
            7 => |x, y| if x == y { 1 } else { 0 },
            _ => panic!("Unknown operator"),
        };
        // determine initial value for fold
        let mut value = 0;
        if ptype == 1 {
            value = 1; // product
        } else if ptype == 2 {
            value = i128::MAX; // minimum so far
        }

        if len_type == 0 {
            let plen = self.bits_value(15);
            let end_pos = self.bit_pos + plen;
            // fold
            if ptype > 4 {
                value = op_type(self.read_packet(), self.read_packet());
            } else {
                while self.bit_pos < end_pos {
                    value = op_type(value, self.read_packet())
                }
            }
        } else {
            let pcount = self.bits_value(11);
            // fold
            if ptype > 4 {
                value = op_type(self.read_packet(), self.read_packet());
            } else {
                for _ in 0..pcount {
                    value = op_type(value, self.read_packet())
                }
            }
        }

        value
    }

    fn bits_value(&mut self, x: i32) -> i32 {
        self.bit_pos += x;
        self.bits
            .by_ref()
            .take(x as usize)
            .fold(0, |acc, v| acc << 1 | v)
    }
}

pub fn step1() {
    let mut pr = PacketReader::new(read_hex_chars("inputs/day16.txt"));

    let _ = pr.read_packet();
    // 967
    println!("Total version: {}", pr.total_ver);
}

pub fn step2() {
    let mut pr = PacketReader::new(read_hex_chars("inputs/day16.txt"));

    // 12883091136209
    println!("Final result: {}", pr.read_packet());
}
