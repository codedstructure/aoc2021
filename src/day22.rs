use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Region {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl Region {
    fn from_str(s: &str) -> Self {
        let mut extents = s.split(',');
        let x = extents.next().unwrap().strip_prefix("x=").unwrap();
        let y = extents.next().unwrap().strip_prefix("y=").unwrap();
        let z = extents.next().unwrap().strip_prefix("z=").unwrap();

        let pairify = |v: Vec<i32>| {
            let mut vi = v.iter();
            (*vi.next().unwrap(), *vi.next().unwrap())
        };
        Region {
            x: pairify(x.split("..").map(|v| v.parse::<i32>().unwrap()).collect()),
            y: pairify(y.split("..").map(|v| v.parse::<i32>().unwrap()).collect()),
            z: pairify(z.split("..").map(|v| v.parse::<i32>().unwrap()).collect()),
        }
    }

    fn overlaps(&self, other: Region) -> bool {
        self.x.0 <= other.x.1
            && self.x.1 >= other.x.0
            && self.y.0 <= other.y.1
            && self.y.1 >= other.y.0
            && self.z.0 <= other.z.1
            && self.z.1 >= other.z.0
    }

    fn volume(&self) -> i32 {
        // +1s below because regions are inclusive of endpoints
        (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
    }

    fn contains(&self, other: Region) -> bool {
        other.x.1 <= self.x.1
            && other.x.0 >= self.x.0
            && other.y.1 <= self.y.1
            && other.y.0 >= self.y.0
            && other.z.1 <= self.z.1
            && other.z.0 >= self.z.0
    }

    fn is_init_region(&self) -> bool {
        self.overlaps(Region {
            x: (-50, 50),
            y: (-50, 50),
            z: (-50, 50),
        })
    }
}

#[derive(Debug)]
struct RegionSet {
    regions: HashSet<Region>,
}

impl RegionSet {
    fn new() -> Self {
        Self {
            regions: HashSet::new(),
        }
    }
    fn total_volume(&self) -> i32 {
        self.regions.iter().map(|r| r.volume()).sum()
    }

    fn contained(&self, r: Region) -> bool {
        for test in &self.regions {
            if test.contains(r) {
                return true;
            }
        }
        false
    }

    fn overlaps(&self, r: Region) -> bool {
        for test in &self.regions {
            if test.overlaps(r) {
                return true;
            }
        }
        false
    }

    fn add(&mut self, r: Region) {
        if self.contained(r) {
            // nothing to do, hurrah!
            return;
        } else if !self.overlaps(r) {
            // disjoint already, just add it.
            self.regions.insert(r);
            return;
        }
        unimplemented!();
        // work to do...
    }

    fn subtract(&mut self, r: Region) {
        if !self.overlaps(r) {
            // nothing to do, hurrah!
            return;
        }
        unimplemented!();
        // work to do...
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    r: Region,
    on: bool,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let mut words = s.split_whitespace();
        let on = match words.next() {
            Some("on") => true,
            Some("off") => false,
            _ => panic!("Invalid region state"),
        };
        let r = Region::from_str(words.next().unwrap());
        Instruction { r, on }
    }
}

#[derive(Debug)]
struct Reactor {
    instructions: Vec<Instruction>,

    regions: RegionSet,
}

impl Reactor {
    fn new(filename: &str) -> Self {
        let mut instructions = vec![];
        for line in read_list(filename) {
            let instr = Instruction::from_str(&line);
            instructions.push(instr);
        }

        let regions = RegionSet::new();

        Self {
            instructions,
            regions,
        }
    }

    fn evaluate(&mut self) {
        for instr in &self.instructions {
            if instr.r.is_init_region() {
                if instr.on {
                    self.regions.add(instr.r);
                } else {
                    self.regions.subtract(instr.r);
                }
            }
        }
    }
}

pub fn step1() {
    let mut reactor = Reactor::new("inputs/sample22.txt");

    println!("{:?}", reactor);
    reactor.evaluate();
    println!("total_volume {}", reactor.regions.total_volume());
}

pub fn step2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let i = Instruction::from_str("on x=1..10,y=11..20,z=-21..30");
        assert_eq!(i.r.x, (1, 10));
        assert_eq!(i.r.y, (11, 20));
        assert_eq!(i.r.z, (-21, 30));
        assert_eq!(i.on, true);
    }

    #[test]
    fn test_region_volume() {
        assert_eq!(Region::from_str("x=0..0,y=9..9,z=3..3").volume(), 1);
        assert_eq!(Region::from_str("x=1..10,y=9..9,z=3..3").volume(), 10);
        assert_eq!(Region::from_str("x=1..10,y=9..10,z=3..3").volume(), 20);
        assert_eq!(Region::from_str("x=1..10,y=9..10,z=3..5").volume(), 60);
    }

    #[test]
    fn test_overlaps() {
        let unit = Region {
            x: (1, 1),
            y: (1, 1),
            z: (1, 1),
        };
        let pair = Region {
            x: (1, 2),
            y: (1, 1),
            z: (1, 1),
        };

        assert!(unit.overlaps(unit));
        assert!(unit.contains(unit));

        assert!(unit.overlaps(pair));
        assert!(pair.overlaps(unit));
        assert!(!unit.contains(pair));
        assert!(pair.contains(unit));
    }

    #[test]
    fn test_add_disjoint() {
        let mut rs = RegionSet::new();
        rs.add(Region {
            x: (1, 5),
            y: (1, 5),
            z: (1, 5),
        });
        assert_eq!(rs.total_volume(), 125);
        rs.add(Region {
            x: (11, 15),
            y: (11, 15),
            z: (11, 15),
        });
        assert_eq!(rs.total_volume(), 250);
    }

    #[test]
    fn test_add_contained() {
        let mut rs = RegionSet::new();
        rs.add(Region {
            x: (1, 5),
            y: (1, 5),
            z: (1, 5),
        });
        rs.add(Region {
            x: (3, 3),
            y: (3, 3),
            z: (3, 3),
        });
        assert_eq!(rs.total_volume(), 125);
    }

    #[test]
    fn test_subtract_disjoint() {
        let mut rs = RegionSet::new();
        rs.add(Region {
            x: (1, 5),
            y: (1, 5),
            z: (1, 5),
        });
        assert_eq!(rs.total_volume(), 125);
        rs.subtract(Region {
            x: (11, 15),
            y: (11, 15),
            z: (11, 15),
        });
        assert_eq!(rs.total_volume(), 125);
    }
}
