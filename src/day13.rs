use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug)]
struct PaperDots {
    dots: HashSet<(i32, i32)>,

    instructions: Vec<(String, i32)>,
}

impl PaperDots {
    fn new(filename: &str) -> Self {
        let mut dots = HashSet::new();
        let mut instructions = Vec::new();
        // Rust TIL: I've been using iter() too much when I should be using
        // into_iter()...
        let mut lines = read_list(filename).into_iter();
        // Rust TIL: iteration takes ownership of iterator, so can't just
        // re-use after break. Using `by_ref()` solves that.
        // https://stackoverflow.com/a/57172670
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let mut coords = line.split(',');
            let x = coords.next().unwrap().parse::<i32>().unwrap();
            let y = coords.next().unwrap().parse::<i32>().unwrap();
            dots.insert((x, y));
        }
        for line in lines {
            let mut instr = line.split('=');
            let folddir: String = instr.next().unwrap().to_string();
            let value: i32 = instr.next().unwrap().parse().unwrap();
            instructions.push((folddir, value));
        }
        Self { dots, instructions }
    }

    fn fold_up(&mut self, value: i32) {
        let mut new_dots = HashSet::new();
        for point in self.dots.iter() {
            if point.1 > value {
                new_dots.insert((point.0, (2 * value) - point.1));
            } else {
                new_dots.insert(*point);
            }
        }
        self.dots = new_dots;
    }

    fn fold_left(&mut self, value: i32) {
        let mut new_dots = HashSet::new();
        for point in self.dots.iter() {
            if point.0 > value {
                new_dots.insert(((2 * value) - point.0, point.1));
            } else {
                new_dots.insert(*point);
            }
        }
        self.dots = new_dots;
    }

    fn fold_step(&mut self, instruction: &(String, i32)) {
        if instruction.0.contains('y') {
            self.fold_up(instruction.1);
        } else if instruction.0.contains('x') {
            self.fold_left(instruction.1);
        }
    }

    fn fold_all(&mut self) {
        for instr in self.instructions.clone().iter() {
            self.fold_step(&(instr.0.clone(), instr.1));
        }
    }

    fn draw(&self) {
        let height = self.dots.iter().max_by_key(|x| x.1).unwrap().1;
        let width = self.dots.iter().max_by_key(|x| x.0).unwrap().0;

        for line in 0..height + 1 {
            for x in 0..width + 1 {
                if self.dots.contains(&(x, line)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

pub fn step1() {
    let mut pd = PaperDots::new("inputs/day13.txt");

    let x = pd.instructions.get(0).unwrap();
    // Rust question - what's the 'deep-copy' equivalent for a Tuple?
    // or should I have pulled the tuple out to a struct deriving Clone?
    let x: (String, i32) = (x.0.clone(), x.1);
    pd.fold_step(&x);

    println!("{}", pd.dots.len());
}

pub fn step2() {
    let mut pd = PaperDots::new("inputs/day13.txt");

    pd.fold_all();
    pd.draw();
}
