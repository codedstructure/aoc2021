// Day 21 - Dirac Dice

use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
struct DetDie {
    state: i32,
    roll_count: i32,
}

impl Iterator for DetDie {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.roll_count += 1;
        let value = self.state + 1;
        self.state = (self.state + 1) % 100;
        Some(value)
    }
}

struct Player {
    position: i32,
    score: i32,
}

impl Player {
    fn new(start: i32) -> Self {
        Self {
            position: (start - 1) % 10,
            score: 0,
        }
    }

    fn advance(&mut self, amount: i32) {
        self.position = (self.position + amount) % 10;
        self.score += self.position + 1; // position is zero-based
    }
}

pub fn step1() {
    // Rust: the by_ref() tripped me up for a while.
    // I guess iterators which 'generate' rather than 'traverse' would
    // normally want this, but rust only has the one Trait for both...
    let mut dd: DetDie = Default::default();
    let dd = dd.by_ref();

    // Player 1 starting position: 8
    // Player 2 starting position: 6
    let mut p1 = Player::new(8);
    let mut p2 = Player::new(6);

    let losing_score = loop {
        p1.advance(dd.take(3).sum());
        if p1.score >= 1000 {
            break p2.score;
        }
        p2.advance(dd.take(3).sum());
        if p2.score >= 1000 {
            break p1.score;
        }
    };

    println!("Final result: {}", dd.roll_count * losing_score);
}

fn run_game(
    remain: i32,
    pos: i32,
    ways: i128,
    throw_count: i32,
    throw_way_map: &mut HashMap<i32, i128>,
) -> i128 {
    let roll_dist: HashMap<i32, i128> =
        HashMap::from_iter([(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]);

    if remain > 0 {
        let mut new_ways = 0;
        for roll_sum in 3..=9 {
            let possibilities = roll_dist.get(&roll_sum).unwrap();
            let new_pos = ((pos - 1) + roll_sum) % 10 + 1;
            new_ways += run_game(
                remain - new_pos,
                new_pos,
                ways * possibilities,
                throw_count + 1,
                throw_way_map,
            );
        }

        return new_ways;
    }

    // We've finished by this point
    *throw_way_map.entry(throw_count).or_insert(0) += ways;
    ways
}

pub fn step2() {
    // Player 1 starting position: 8
    // Player 2 starting position: 6
    let mut p1_throw_ways = HashMap::new();
    let p1_complete = run_game(21, 8, 1, 0, &mut p1_throw_ways);
    println!("p1: {}", p1_complete);
    println!("{:?}", p1_throw_ways);

    let mut p2_throw_ways = HashMap::new();
    let p2_complete = run_game(21, 6, 1, 0, &mut p2_throw_ways);
    println!("p2: {}", p2_complete);
    println!("{:?}", p2_throw_ways);

    let mut p1_win_count = 0;
    let mut p2_win_count = 0;
    let mut total_universes_p1 = 1;
    let mut total_universes_p2 = 1;

    // 10 rounds is sufficient for this input data
    for round in 1..=10 {
        // three dice rolls, each splitting universe three ways.
        total_universes_p1 *= 27;
        if let Some(p1_wins_this_throw) = p1_throw_ways.get(&round) {
            // Some potential universes have terminated...
            total_universes_p1 -= p1_wins_this_throw;
            // Scale p1 wins by number of universes from previous (i.e. p2) throw
            p1_win_count += p1_wins_this_throw * total_universes_p2;
        }
        total_universes_p2 *= 27;
        if let Some(p2_wins_this_throw) = p2_throw_ways.get(&round) {
            // Some potential universes have terminated...
            total_universes_p2 -= p2_wins_this_throw;
            // Scale p2 wins by number of universes from previous (i.e. p1) throw
            p2_win_count += p2_wins_this_throw * total_universes_p1;
        }
        println!(
            "round {:2}: universes p1: {:8}, p2: {:8}",
            round, total_universes_p1, total_universes_p2
        );
    }

    println!("p1 win universes: {}", p1_win_count);
    println!("p2 win universes: {}", p2_win_count);
}
