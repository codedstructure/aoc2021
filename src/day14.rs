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
struct Polymer {
    template: String,
    rules: HashMap<String, String>,
}

impl Polymer {
    fn new(filename: &str) -> Self {
        let mut lines = read_list(filename).into_iter();

        let template = lines.next().unwrap();
        lines.next(); // skip blank line

        let mut rules = HashMap::new();
        for line in lines {
            let mut rule = line.split(" -> ");
            let before = rule.next().unwrap();
            let after = rule.next().unwrap();

            // Rust: seems there's a choice between noisy 'to_string()'
            // everywhere or noisy lifetimes everywhere?
            rules.insert(before.to_string(), after.to_string());
        }
        Self { template, rules }
    }

    fn polymerize(&mut self) {
        let mut result = String::new();

        let mut ch_iter = self.template.chars();
        let mut last_ch = ch_iter.next().unwrap();
        result.push(last_ch);
        for ch in ch_iter {
            let pair = last_ch.to_string() + &ch.to_string();
            last_ch = ch;
            result.push_str(self.rules.get(&pair).unwrap());
            result.push(ch);
        }

        self.template = result;
    }

    fn frequency(&self) -> HashMap<char, i32> {
        let mut result = HashMap::new();
        for ch in self.template.chars() {
            *result.entry(ch).or_insert(0) += 1;
        }
        result
    }

    fn most_common_count(&self) -> i32 {
        *self.frequency().iter().max_by_key(|i| i.1).unwrap().1
    }

    fn least_common_count(&self) -> i32 {
        *self.frequency().iter().min_by_key(|i| *i.1).unwrap().1
    }
}

pub fn step1() {
    let mut polymer = Polymer::new("inputs/day14.txt");

    for _ in 0..10 {
        polymer.polymerize();
    }
    println!(
        "{}",
        polymer.most_common_count() - polymer.least_common_count()
    );
}

#[derive(Debug)]
struct EfficientPolymer {
    pair_count: HashMap<String, i64>,
    element_count: HashMap<char, i64>,
    rules: HashMap<String, String>,
}

impl EfficientPolymer {
    fn new(filename: &str) -> Self {
        let mut lines = read_list(filename).into_iter();

        let mut pair_count = HashMap::new();
        let mut element_count = HashMap::new();

        let template = lines.next().unwrap();
        let mut ch_iter = template.chars();
        let mut last_ch = ch_iter.next().unwrap();

        // I missed this to begin with, resulting in an off-by-one error :(
        *element_count.entry(last_ch).or_insert(0) += 1;

        for ch in ch_iter {
            let pair = last_ch.to_string() + &ch.to_string();
            *pair_count.entry(pair).or_insert(0) += 1;
            *element_count.entry(ch).or_insert(0) += 1;
            last_ch = ch;
        }

        lines.next(); // skip blank line

        let mut rules = HashMap::new();
        for line in lines {
            let mut rule = line.split(" -> ");
            let before = rule.next().unwrap();
            let after = rule.next().unwrap();

            // Rust: seems there's a choice between noisy 'to_string()'
            // everywhere or noisy lifetimes everywhere?
            rules.insert(before.to_string(), after.to_string());
        }
        Self {
            pair_count,
            element_count,
            rules,
        }
    }

    fn polymerize(&mut self) {
        let mut result = HashMap::new();

        for (pair, count) in self.pair_count.iter() {
            let newchar = self.rules.get(pair).unwrap();
            let element = String::from(newchar);
            let first = pair.chars().next().unwrap().to_string() + &element;
            let second = element + &pair.chars().nth(1).unwrap().to_string();
            *result.entry(first).or_insert(0) += count;
            *result.entry(second).or_insert(0) += count;
            *self
                .element_count
                .entry(newchar.chars().next().unwrap())
                .or_insert(0) += count;
        }

        self.pair_count = result;
    }

    fn most_common_count(&self) -> i64 {
        *self.element_count.iter().max_by_key(|i| i.1).unwrap().1
    }

    fn least_common_count(&self) -> i64 {
        *self.element_count.iter().min_by_key(|i| i.1).unwrap().1
    }
}

pub fn step2() {
    let mut polymer = EfficientPolymer::new("inputs/day14.txt");

    for _ in 0..40 {
        polymer.polymerize();
    }
    println!(
        "{}",
        polymer.most_common_count() - polymer.least_common_count()
    );
}
