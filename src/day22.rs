use std::{
    cmp::{max, min},
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
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl Region {
    fn from_str(s: &str) -> Self {
        let mut extents = s.split(',');
        let x = extents.next().unwrap().strip_prefix("x=").unwrap();
        let y = extents.next().unwrap().strip_prefix("y=").unwrap();
        let z = extents.next().unwrap().strip_prefix("z=").unwrap();

        let pairify = |v: Vec<i64>| {
            let mut vi = v.iter();
            (*vi.next().unwrap(), *vi.next().unwrap())
        };
        Region {
            x: pairify(x.split("..").map(|v| v.parse::<i64>().unwrap()).collect()),
            y: pairify(y.split("..").map(|v| v.parse::<i64>().unwrap()).collect()),
            z: pairify(z.split("..").map(|v| v.parse::<i64>().unwrap()).collect()),
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

    fn split_x(
        &self,
        s: (Option<(i64, i64)>, Option<(i64, i64)>, Option<(i64, i64)>),
    ) -> HashSet<Region> {
        let mut result = HashSet::new();
        //println!("split_x {:?} at {:?}", self, s);
        // Check s is in range; also impossible to split a one-width region.
        //assert!(s > self.x.0 && s <= self.x.1);
        if let Some(x_1) = s.0 {
            result.insert(Region {
                x: x_1,
                y: self.y,
                z: self.z,
            });
        }
        if let Some(x_2) = s.1 {
            result.insert(Region {
                x: x_2,
                y: self.y,
                z: self.z,
            });
        }
        if let Some(x_3) = s.2 {
            result.insert(Region {
                x: x_3,
                y: self.y,
                z: self.z,
            });
        }
        result
    }
    fn split_y(
        &self,
        s: (Option<(i64, i64)>, Option<(i64, i64)>, Option<(i64, i64)>),
    ) -> HashSet<Region> {
        let mut result = HashSet::new();
        //println!("split_y {:?} at {:?}", self, s);
        // Check s is in range; also impossible to split a one-width region.
        //assert!(s > self.x.0 && s <= self.x.1);
        if let Some(y_1) = s.0 {
            result.insert(Region {
                x: self.x,
                y: y_1,
                z: self.z,
            });
        }
        if let Some(y_2) = s.1 {
            result.insert(Region {
                x: self.x,
                y: y_2,
                z: self.z,
            });
        }
        if let Some(y_3) = s.2 {
            result.insert(Region {
                x: self.x,
                y: y_3,
                z: self.z,
            });
        }
        result
    }
    fn split_z(
        &self,
        s: (Option<(i64, i64)>, Option<(i64, i64)>, Option<(i64, i64)>),
    ) -> HashSet<Region> {
        let mut result = HashSet::new();
        // println!("split_z {:?} at {:?}", self, s);
        // Check s is in range; also impossible to split a one-width region.
        //assert!(s > self.x.0 && s <= self.x.1);
        if let Some(z_1) = s.0 {
            result.insert(Region {
                x: self.x,
                y: self.y,
                z: z_1,
            });
        }
        if let Some(z_2) = s.1 {
            result.insert(Region {
                x: self.x,
                y: self.y,
                z: z_2,
            });
        }
        if let Some(z_3) = s.2 {
            result.insert(Region {
                x: self.x,
                y: self.y,
                z: z_3,
            });
        }
        result
    }

    fn contained(span: (i64, i64), other: (i64, i64)) -> bool {
        other.0 >= span.0 && other.1 <= span.1
    }
    fn disjoint(span: (i64, i64), other: (i64, i64)) -> bool {
        other.1 < span.0 || other.0 > span.1
    }
    fn split_points(
        span: (i64, i64),
        other: (i64, i64),
    ) -> (Option<(i64, i64)>, Option<(i64, i64)>, Option<(i64, i64)>) {
        // The returned split point(s) are always *outside* the span.
        //println!(" split_points {:?} {:?} ", span, other);
        assert!(span.0 <= span.1);
        assert!(other.0 <= other.1);

        if Region::contained(span, other) {
            return (None, Some(other), None);
        }
        if Region::disjoint(span, other) {
            return (Some(span), None, Some(other));
        }

        let mut left = None;
        let mut left_center = span;
        let mut right = None;
        let mut right_center = span;

        // we have overlap
        if other.1 > span.1 {
            right = Some((span.1 + 1, other.1));
            right_center = (other.0, span.1);
        }
        if other.0 < span.0 {
            left = Some((other.0, span.0 - 1));
            left_center = (span.0, other.1);
        }
        let center = Some((
            max(left_center.0, right_center.0),
            min(left_center.1, right_center.1),
        ));
        //println!(" -> ({:?}, {:?})", left, right);
        (left, center, right)
    }

    fn split_against(&self, other: Region) -> HashSet<Region> {
        let mut splitx = HashSet::new();
        //println!("extension: {:?} / {:?}", self, other);
        let splits_x = Region::split_points(self.x, other.x);
        for rp in other.split_x(splits_x) {
            splitx.insert(rp);
        }
        //println!(" Following splitx: {:?}", splitx);
        let mut splitxy = HashSet::new();
        for v in splitx {
            if self.overlaps(v) {
                let splits_y = Region::split_points(self.y, v.y);
                for rp in v.split_y(splits_y) {
                    splitxy.insert(rp);
                }
            } else {
                splitxy.insert(v);
            }
        }
        //println!(" Following splitxy: {:?}", splitxy);
        let mut splitxyz = HashSet::new();
        for v in splitxy {
            if self.overlaps(v) {
                let splits_z = Region::split_points(self.z, v.z);
                for rp in v.split_z(splits_z) {
                    splitxyz.insert(rp);
                }
            } else {
                splitxyz.insert(v);
            }
        }
        //println!(" Following splitxyz: {:?}", splitxyz);
        assert!(splitxyz.len() <= 7);
        splitxyz
    }

    fn volume(&self) -> i64 {
        // +1s below because regions are inclusive of endpoints
        (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
    }

    fn is_contained_by(&self, other: Region) -> bool {
        other.x.0 <= self.x.0
            && other.x.1 >= self.x.1
            && other.y.0 <= self.y.0
            && other.y.1 >= self.y.1
            && other.z.0 <= self.z.0
            && other.z.1 >= self.z.1
    }

    fn is_init_region(&self) -> bool {
        self.is_contained_by(Region {
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

    fn total_volume(&self) -> i64 {
        self.check_disjoint();
        self.regions.iter().map(|r| r.volume()).sum()
    }

    fn check_disjoint(&self) {
        for r in &self.regions {
            for s in &self.regions {
                if s == r {
                    continue;
                }
                assert!(!r.overlaps(*s));
            }
        }
    }

    fn add(&mut self, new_region: Region) {
        let mut to_add = HashSet::new();
        let mut to_remove = HashSet::new();

        for this in &self.regions {
            if new_region.is_contained_by(*this) {
                // nothing to do, hurrah!
                return;
            }
        }

        for this in &self.regions {
            if this.overlaps(new_region) {
                // split the overlapped existing entry - first schedule
                // removing it, then add in all the non-overlapping parts
                // Since it started out as disjoint, we can directly add
                // the split parts without needing to recurse.
                to_remove.insert(*this);
                for ext in new_region.split_against(*this) {
                    if !ext.overlaps(new_region) {
                        // parts of previously existing region which
                        // aren't overlapping the new region should be
                        // re-added.
                        to_add.insert(ext);
                    }
                }
            }
        }
        to_add.insert(new_region);

        for reg in to_remove {
            self.regions.remove(&reg);
        }
        for reg in to_add {
            self.regions.insert(reg);
        }
    }

    fn subtract(&mut self, r: Region) {
        let mut to_add = HashSet::new();
        let mut to_remove = HashSet::new();

        for reg in &self.regions {
            if reg.overlaps(r) {
                to_remove.insert(*reg);
                let split_apart = r.split_against(*reg);
                for part in split_apart {
                    if !part.overlaps(r) {
                        to_add.insert(part);
                    }
                }
            }
        }
        for reg in to_remove {
            self.regions.remove(&reg);
        }
        for reg in to_add {
            self.regions.insert(reg);
        }
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

    fn evaluate(&mut self, all_instructions: bool) {
        for (_x, instr) in self.instructions.iter().enumerate() {
            //println!("  ** Instruction {}: {:?}", _x, instr);
            if all_instructions || instr.r.is_init_region() {
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
    let mut reactor = Reactor::new("inputs/day22.txt");

    reactor.evaluate(false);
    // 561032
    println!(
        "total_volume (init only) {}",
        reactor.regions.total_volume()
    );
}

pub fn step2() {
    let mut reactor = Reactor::new("inputs/day22.txt");

    reactor.evaluate(true);
    // 1322825263376414
    println!("total_volume {}", reactor.regions.total_volume());
}

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
        assert!(unit.is_contained_by(unit));

        assert!(unit.overlaps(pair));
        assert!(pair.overlaps(unit));
        assert!(unit.is_contained_by(pair));
        assert!(!pair.is_contained_by(unit));
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
    fn test_add_overlap() {
        let mut rs = RegionSet::new();
        rs.add(Region {
            x: (1, 2),
            y: (1, 2),
            z: (1, 2),
        });
        rs.add(Region {
            x: (2, 3),
            y: (2, 3),
            z: (2, 3),
        });
        assert_eq!(rs.total_volume(), 15);
        rs.add(Region {
            x: (1, 2),
            y: (1, 2),
            z: (1, 10),
        });
        assert_eq!(rs.total_volume(), 46);
        //assert_eq!(rs.regions.len(), 7);

        rs = RegionSet::new();
        rs.add(Region::from_str("x=1..3,y=1..3,z=1..1"));
        assert_eq!(rs.regions.len(), 1);
        assert_eq!(rs.total_volume(), 9);
        rs.add(Region::from_str("x=1..6,y=2..4,z=1..1"));
        // would be nice to just need 3 here...
        //assert_eq!(rs.regions.len(), 4);
        assert_eq!(rs.total_volume(), 21);

        rs = RegionSet::new();
        rs.add(Region::from_str("x=-20..26,y=-36..17,z=-47..7"));
        assert_eq!(rs.regions.len(), 1);
        assert_eq!(rs.total_volume(), 139590);
        rs.add(Region::from_str("x=-20..33,y=-21..23,z=-26..28"));
        //assert_eq!(rs.regions.len(), 8);
        assert_eq!(rs.total_volume(), 210918);
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

    #[test]
    fn test_subtract_overlap() {
        let mut rs = RegionSet::new();
        rs.add(Region {
            x: (1, 6),
            y: (2, 4),
            z: (1, 1),
        });
        rs.subtract(Region {
            x: (1, 3),
            y: (1, 3),
            z: (1, 1),
        });
        assert_eq!(rs.total_volume(), 12);

        rs = RegionSet::new();
        rs.add(Region {
            x: (1, 3),
            y: (1, 3),
            z: (1, 1),
        });
        rs.subtract(Region {
            x: (2, 2),
            y: (2, 2),
            z: (1, 1),
        });
        assert_eq!(rs.total_volume(), 8);
    }

    #[test]
    fn test_split_against() {
        let r1 = Region::from_str("x=1..2,y=1..2,z=1..2");
        let r2 = Region::from_str("x=1..2,y=1..2,z=1..2");
        let r3 = Region::from_str("x=2..3,y=2..3,z=2..3");
        assert_eq!(r1.split_against(r2), HashSet::from([r1]));
        assert_eq!(
            r1.split_against(r3),
            HashSet::from([
                Region {
                    x: (3, 3),
                    y: (2, 3),
                    z: (2, 3)
                },
                Region {
                    x: (2, 2),
                    y: (3, 3),
                    z: (2, 3)
                },
                Region {
                    x: (2, 2),
                    y: (2, 2),
                    z: (3, 3)
                },
                Region {
                    x: (2, 2),
                    y: (2, 2),
                    z: (2, 2)
                },
            ])
        );
    }

    #[test]
    fn test_split_points() {
        /*
        assert_eq!(Region::split_points((1, 2), (2, 3)), (None, Some(3)));
        assert_eq!(Region::split_points((2, 3), (1, 2)), (Some(1), None));
        assert_eq!(Region::split_points((2, 10), (3, 5)), (None, None));
        assert_eq!(Region::split_points((3, 5), (1, 50)), (Some(2), Some(6)));
        assert_eq!(Region::split_points((3, 5), (3, 5)), (None, None));
        assert_eq!(Region::split_points((-5, -2), (3, 5)), (None, None));
        */
    }
}
