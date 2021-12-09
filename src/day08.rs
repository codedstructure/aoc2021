use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

pub fn step1() {
    let mut count = 0;
    for entry in read_list("inputs/day08.txt") {
        let mut entry_parts = entry.split(" | ");
        let _controls = entry_parts.next().unwrap();
        let outputs = entry_parts.next().unwrap();
        for out in outputs.split(' ') {
            match out.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            }
        }
    }
    println!("Count: {}", count);
}

fn digitize(value: &str) -> u8 {
    let mut result = 0;
    for ch in value.chars() {
        match ch {
            'a' => result |= 1,
            'b' => result |= 2,
            'c' => result |= 4,
            'd' => result |= 8,
            'e' => result |= 16,
            'f' => result |= 32,
            'g' => result |= 64,
            _ => unimplemented!("invalid char"),
        }
    }
    result
}

fn decode_entry(entry: &str) -> usize {
    let mut entry_parts = entry.split(" | ");
    let mut map = HashMap::new();
    let controls = entry_parts.next().unwrap().split(' ').map(|s| digitize(s));
    let outputs = entry_parts.next().unwrap().split(' ').map(|s| digitize(s));

    let mut one_pattern = 0;
    let mut four_pattern = 0;
    for sample in controls.clone() {
        match sample.count_ones() {
            2 => {
                one_pattern = sample;
                map.insert(sample, 1);
            } // 2 segments => '1'
            3 => {
                map.insert(sample, 7);
            } // 3 segments => '7'
            4 => {
                four_pattern = sample;
                map.insert(sample, 4);
            } // 4 segments => '4'
            7 => {
                map.insert(sample, 8);
            } // 7 segments => '8'
            _ => (),
        }
    }

    // we need to have already determined one/four patterns to
    // disambiguate. Fortunately that provides enough.
    for sample in controls {
        // 5 segments => {2,3,5}
        if sample.count_ones() == 5 {
            if sample & one_pattern == one_pattern {
                // 3 shares segments with 1
                map.insert(sample, 3);
            } else if (sample & four_pattern).count_ones() == 3 {
                // 4 shares 3 segments with 5
                map.insert(sample, 5);
            } else {
                // the other 5-segment value.
                map.insert(sample, 2);
            }
        }
        // 6 segments => {0,6,9}
        if sample.count_ones() == 6 {
            if sample & four_pattern == four_pattern {
                // 9 shares all digits with 4
                map.insert(sample, 9);
            } else if sample & one_pattern == one_pattern {
                // 1 shares all segments with zero
                map.insert(sample, 0);
            } else {
                // the other 6-segment value.
                map.insert(sample, 6);
            }
        }
    }

    assert!(map.len() == 10);

    let mut result = 0;
    for digit in outputs {
        result = 10 * result + map[&digit];
    }

    result
}

pub fn step2() {
    let mut total = 0;

    assert!(
        5353 == decode_entry(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
        )
    );

    for entry in read_list("inputs/day08.txt") {
        total += decode_entry(&entry);
    }
    println!("Total: {}", total);
}
