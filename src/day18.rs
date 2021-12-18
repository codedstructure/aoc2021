use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug, Clone, PartialEq)]
enum Sfn {
    Regular(i32, i32),
    Pair(Box<Sfn>, Box<Sfn>, i32),
}

impl Sfn {
    fn from_str(s: &str) -> Sfn {
        Sfn::read_value(&mut s.chars(), 0)
    }

    fn read_value(s: &mut dyn Iterator<Item = char>, depth: i32) -> Sfn {
        let ch = s.next().unwrap();

        if let Some(x) = ch.to_digit(10) {
            Sfn::Regular(x as i32, depth)
        } else {
            // Start of a new pair
            assert!(ch == '[');
            let left = Sfn::read_value(s, depth + 1);
            assert!(s.next() == Some(','));
            let right = Sfn::read_value(s, depth + 1);
            assert!(s.next() == Some(']'));
            Sfn::pair(left, right, depth)
        }
    }

    fn split(&self) -> Sfn {
        match self {
            Sfn::Regular(x, d) => {
                if x > &9 {
                    Sfn::Pair(
                        Box::new(Sfn::Regular(x / 2, d + 1)),
                        Box::new(Sfn::Regular((x + 1) / 2, d + 1)),
                        *d,
                    )
                } else {
                    Sfn::Regular(*x, *d)
                }
            }
            Sfn::Pair(x, y, d) => {
                let mut first = *x.clone();
                let mut second = *y.clone();
                let z = first.split();
                if z != first {
                    // x split
                    first = z;
                } else {
                    // we only split the first thing.
                    let z = second.split();
                    if z != second {
                        // y split
                        second = z;
                    }
                }
                Sfn::Pair(Box::new(first), Box::new(second), *d)
            }
        }
    }

    fn depth(&self) -> i32 {
        match self {
            Sfn::Regular(_, d) => *d,
            Sfn::Pair(_, _, d) => *d,
        }
    }

    fn simple_pair(&self) -> bool {
        match self {
            Sfn::Regular(_, _) => false,
            Sfn::Pair(left, right, _) => {
                let a = match **left {
                    Sfn::Regular(_, _) => true,
                    Sfn::Pair(_, _, _) => false,
                };
                let b = match **right {
                    Sfn::Regular(_, _) => true,
                    Sfn::Pair(_, _, _) => false,
                };
                a && b
            }
        }
    }

    fn explosive(&self) -> bool {
        // Does this contain any parts which have depth > 4?
        if self.depth() > 4 {
            true
        } else {
            match self {
                Sfn::Regular(_, _) => false,
                Sfn::Pair(a, b, _) => a.explosive() || b.explosive(),
            }
        }
    }

    fn explode(&self) -> Sfn {
        if !self.explosive() {
            return self.clone();
        }
        let mut add_to_next = 0;
        let mut add_to_prev = 0;
        let mut last_reg_idx = 0;
        let mut acted = false;
        let s = self.visit(
            &mut acted,
            &mut add_to_next,
            &mut add_to_prev,
            &mut last_reg_idx,
        );
        s.add_to_previous(last_reg_idx - 1, add_to_prev)
    }

    fn add_to_previous(&self, add_idx: i32, value: i32) -> Sfn {
        let mut reg_idx = 0;
        self.add_to_previous_inner(&mut reg_idx, add_idx, value)
    }
    fn add_to_previous_inner(&self, reg_idx: &mut i32, last_reg_idx: i32, add_value: i32) -> Sfn {
        match self {
            Sfn::Regular(x, d) => {
                let mut v = *x;
                if *reg_idx == last_reg_idx {
                    v += add_value;
                }
                *reg_idx += 1;
                Sfn::Regular(v, *d)
            }
            Sfn::Pair(a, b, d) => Sfn::pair(
                a.add_to_previous_inner(reg_idx, last_reg_idx, add_value),
                b.add_to_previous_inner(reg_idx, last_reg_idx, add_value),
                *d,
            ),
        }
    }

    fn visit(
        &self,
        acted: &mut bool,
        add_to_next: &mut i32,
        add_to_previous: &mut i32,
        last_reg_idx: &mut i32,
    ) -> Sfn {
        match self {
            Sfn::Regular(x, d) => {
                let add = *add_to_next;
                *add_to_next = 0;
                if !*acted {
                    *last_reg_idx += 1;
                }
                Sfn::Regular(x + add, *d)
            }
            Sfn::Pair(a, b, d) => {
                if !*acted && self.simple_pair() && self.depth() >= 4 {
                    // explode this pair
                    let (add_prev, add_next) = self.to_pair().unwrap();
                    *add_to_next = add_next;
                    *add_to_previous = add_prev;
                    *acted = true;

                    Sfn::Regular(0, *d)
                } else {
                    Sfn::pair(
                        a.visit(acted, add_to_next, add_to_previous, last_reg_idx),
                        b.visit(acted, add_to_next, add_to_previous, last_reg_idx),
                        *d,
                    )
                }
            }
        }
    }

    fn add(&self, other: Sfn) -> Sfn {
        Sfn::Pair(Box::new(self.deepen()), Box::new(other.deepen()), 0)
    }

    fn deepen(&self) -> Sfn {
        match self {
            Sfn::Regular(x, d) => Sfn::Regular(*x, d + 1),
            Sfn::Pair(x, y, d) => Sfn::pair(x.deepen(), y.deepen(), d + 1),
        }
    }

    fn reduce(&self) -> Sfn {
        let mut previous = self.clone();
        loop {
            let mut result = previous.explode();
            if result == previous {
                result = result.split();
            }
            if result == previous {
                break result;
            }
            previous = result.clone();
        }
    }

    fn to_num(&self) -> Option<i32> {
        match self {
            Sfn::Regular(x, _) => Some(*x),
            Sfn::Pair(_, _, _) => None,
        }
    }

    fn to_pair(&self) -> Option<(i32, i32)> {
        match self {
            Sfn::Regular(_, _) => None,
            Sfn::Pair(a, b, _) => {
                if self.simple_pair() {
                    Some((a.to_num().unwrap(), b.to_num().unwrap()))
                } else {
                    None
                }
            }
        }
    }

    fn magnitude(&self) -> i32 {
        match self {
            Sfn::Regular(x, _) => *x,
            Sfn::Pair(a, b, _) => 3 * a.magnitude() + 2 * b.magnitude(),
        }
    }

    fn make_pair(a: i32, b: i32, d: i32) -> Sfn {
        Sfn::pair(Sfn::Regular(a, d + 1), Sfn::Regular(b, d + 1), d)
    }

    fn pair(a: Sfn, b: Sfn, d: i32) -> Sfn {
        Sfn::Pair(Box::new(a), Box::new(b), d)
    }

    // Rust question: clippy wants me to implement this as fmt::Display,
    // but when I try that I get an infinite loop... What's going on?
    fn to_string(&self) -> String {
        match self {
            Sfn::Regular(x, _) => x.to_string(),
            Sfn::Pair(x, y, _) => {
                format!("[{},{}]", x.to_string(), y.to_string())
            }
        }
    }
}

pub fn step1() {
    let mut result: Option<Sfn> = None;

    for line in read_list("inputs/day18.txt") {
        let next_sfn = Sfn::from_str(&line);
        if let Some(sfn) = result {
            result = Some(sfn.add(next_sfn).reduce());
        } else {
            result = Some(next_sfn);
        }
    }

    // 3359
    println!("Magnitude: {}", result.unwrap().magnitude());
}

pub fn step2() {
    let mut max_mag = 0;
    for line1 in read_list("inputs/day18.txt") {
        for line2 in read_list("inputs/day18.txt") {
            if line1 == line2 {
                continue;
            }
            let mag = Sfn::from_str(&line1)
                .add(Sfn::from_str(&line2))
                .reduce()
                .magnitude();
            if mag > max_mag {
                max_mag = mag;
            }
        }
    }
    // 4616
    println!("Max Magnitude: {}", max_mag);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regular_sfn() {
        let a = Sfn::Regular(1, 0);
        let b = Sfn::Regular(1, 0);
        let c = Sfn::Regular(5, 0);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_regular_addition() {
        let a = Sfn::Regular(1, 0);
        let b = Sfn::Regular(1, 0);
        let c = Sfn::Regular(5, 0);
        let d = Sfn::Regular(3, 0);

        assert_eq!(a.add(b), Sfn::from_str("[1,1]"));
        assert_eq!(c.add(d), Sfn::from_str("[5,3]"));
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Sfn::from_str("7"), Sfn::Regular(7, 0));
        assert_eq!(Sfn::from_str("[1,2]"), Sfn::make_pair(1, 2, 0));
        assert_eq!(
            Sfn::from_str("[[9,2],1]"),
            Sfn::pair(Sfn::make_pair(9, 2, 1), Sfn::Regular(1, 1), 0)
        );
        assert_eq!(
            Sfn::from_str("[[1,2],[[3,4],5]]"),
            Sfn::pair(
                Sfn::make_pair(1, 2, 1),
                Sfn::pair(Sfn::make_pair(3, 4, 2), Sfn::Regular(5, 2), 1),
                0
            )
        );
    }

    #[test]
    fn test_split() {
        assert_eq!(Sfn::Regular(10, 0).split(), Sfn::from_str("[5,5]"));
        assert_eq!(Sfn::Regular(11, 0).split(), Sfn::from_str("[5,6]"));
        assert_eq!(Sfn::Regular(12, 0).split(), Sfn::from_str("[6,6]"));
        assert_eq!(Sfn::make_pair(12, 3, 0).split(), Sfn::from_str("[[6,6],3]"));
        assert_eq!(Sfn::make_pair(3, 11, 0).split(), Sfn::from_str("[3,[5,6]]"));
        // Check it only applies to the first entry
        assert_eq!(
            Sfn::make_pair(10, 11, 0).split(),
            Sfn::from_str("[5,5]").add(Sfn::Regular(11, 0))
        );
    }

    #[test]
    fn test_explosive() {
        assert_eq!(Sfn::from_str("[2,3]").explosive(), false);
        assert_eq!(Sfn::from_str("[[2,3],1]").explosive(), false);
        assert_eq!(Sfn::from_str("[2,[3,4]]").explosive(), false);
        assert_eq!(Sfn::from_str("[[[[2,3],5],4],2]").explosive(), false);
        assert_eq!(Sfn::from_str("[[[[1,[2,3]],2],5],3]").explosive(), true);
    }

    #[test]
    fn test_simple_pair() {
        assert_eq!(Sfn::from_str("[5,2]").simple_pair(), true);
        assert_eq!(Sfn::from_str("[[1,5],2]").simple_pair(), false);
    }

    #[test]
    fn test_explode_left() {
        assert_eq!(
            Sfn::from_str("[[[[[9,8],1],2],3],4]").explode(),
            Sfn::from_str("[[[[0,9],2],3],4]")
        );
    }

    #[test]
    fn test_explode_right() {
        assert_eq!(
            Sfn::from_str("[7,[6,[5,[4,[3,2]]]]]").explode(),
            Sfn::from_str("[7,[6,[5,[7,0]]]]")
        );
    }

    #[test]
    fn test_explode() {
        assert_eq!(
            Sfn::from_str("[[6,[5,[4,[3,2]]]],1]").explode(),
            Sfn::from_str("[[6,[5,[7,0]]],3]")
        );
        assert_eq!(
            Sfn::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").explode(),
            Sfn::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        );
        assert_eq!(
            Sfn::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").explode(),
            Sfn::from_str("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Sfn::from_str("[[1,2],[[3,4],5]]").magnitude(), 143);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            Sfn::from_str("[[1,2],[[3,4],5]]").to_string(),
            "[[1,2],[[3,4],5]]".to_string()
        );
    }

    #[test]
    fn test_reduce() {
        assert_eq!(
            Sfn::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]")
                .add(Sfn::from_str("[1,1]"))
                .reduce(),
            Sfn::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }
}
