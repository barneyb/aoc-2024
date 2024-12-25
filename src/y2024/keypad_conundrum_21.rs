use crate::geom2d::step;
use crate::geom2d::Dir::*;
use crate::hist::Histogram;
use crate::Part;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Index;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

type Pt = (i32, i32);
type Move = (char, char);

struct Keypad {
    keys: HashMap<char, Pt>,
    gap_row: i32,
}

impl Keypad {
    fn compute_paths(&self, start: char, end: char) -> Vec<Histogram<Move>> {
        let mut paths = Vec::new();
        let mut shortest = usize::MAX;
        let goal = self[&end];
        let points: HashSet<_> = self.keys.values().collect();
        let mut queue = VecDeque::new();
        queue.push_back((self[&start], String::new()));
        while let Some((p, path)) = queue.pop_front() {
            if p == goal {
                shortest = path.len();
                paths.push(path + "A");
                continue;
            }
            if path.len() > shortest {
                continue;
            }
            let n = step(p, North);
            if points.contains(&n) {
                queue.push_back((n, path.clone() + "^"))
            }
            let n = step(p, East);
            if points.contains(&n) {
                queue.push_back((n, path.clone() + ">"))
            }
            let n = step(p, South);
            if points.contains(&n) {
                queue.push_back((n, path.clone() + "v"))
            }
            let n = step(p, West);
            if points.contains(&n) {
                queue.push_back((n, path.clone() + "<"))
            }
        }
        let mut result = Vec::with_capacity(paths.len());
        for p in paths {
            result.push(string_to_moves(&p));
        }
        result
    }

    fn is_gap_row(&self, row: i32) -> bool {
        self.gap_row == row
    }

    fn is_gap_col(&self, col: i32) -> bool {
        0 == col
    }

    fn the_one_true_path(&self, mv: &Move) -> Histogram<Move> {
        let mut buf = String::new();
        let &(curr, tgt) = mv;
        let curr = self[&curr];
        let tgt = self[&tgt];
        let mut dx = tgt.0 - curr.0;
        let mut dy = tgt.1 - curr.1;
        // dodge the gap moving <
        if dx < 0 && (!self.is_gap_col(tgt.0) || !self.is_gap_row(curr.1)) {
            buf.push_str(&"<".repeat(-dx as usize));
            dx = 0;
        }
        // dodge the gap moving v
        if dy > 0 && (!self.is_gap_row(tgt.1) || !self.is_gap_col(curr.0)) {
            buf.push_str(&"v".repeat(dy as usize));
            dy = 0;
        }
        if dx > 0 {
            buf.push_str(&">".repeat(dx as usize));
        }
        if dy < 0 {
            buf.push_str(&"^".repeat(-dy as usize));
        }
        if dy > 0 {
            buf.push_str(&"v".repeat(dy as usize));
        }
        if dx < 0 {
            buf.push_str(&"<".repeat(-dx as usize));
        }
        buf.push('A');
        string_to_moves(&buf)
    }
}

impl Index<&char> for Keypad {
    type Output = Pt;

    fn index(&self, index: &char) -> &Self::Output {
        &self.keys[index]
    }
}

lazy_static! {
    static ref NUMERIC: Keypad = Keypad {
        keys: HashMap::from([
            ('7', (0, 0)),
            ('8', (1, 0)),
            ('9', (2, 0)),
            ('4', (0, 1)),
            ('5', (1, 1)),
            ('6', (2, 1)),
            ('1', (0, 2)),
            ('2', (1, 2)),
            ('3', (2, 2)),
            ('0', (1, 3)),
            ('A', (2, 3)),
        ]),
        gap_row: 3
    };
    static ref DIRECTIONAL: Keypad = Keypad {
        keys: HashMap::from([
            ('^', (1, 0)),
            ('A', (2, 0)),
            ('<', (0, 1)),
            ('v', (1, 1)),
            ('>', (2, 1)),
        ]),
        gap_row: 0
    };
}

fn string_to_moves(p: &str) -> Histogram<Move> {
    let mut r = Histogram::new();
    let mut c = 'A';
    for t in p.chars() {
        r.increment((c, t));
        c = t;
    }
    r
}

fn shortest_path(code: &str, dir_bots: usize) -> usize {
    let mut curr = 'A';
    let mut sum = 0;
    for tgt in code.chars() {
        // println!("for {curr} -> {tgt}:");
        let mut best = usize::MAX;
        for path in NUMERIC.compute_paths(curr, tgt) {
            let mut gen = path;
            for _ in 0..dir_bots {
                // println!("  for {gen:#?}, after {b} dir bots: ");
                let mut next = Histogram::new();
                for (pmv, n) in gen {
                    for (nmv, m) in DIRECTIONAL.the_one_true_path(&pmv) {
                        next.add(nmv, n * m);
                    }
                }
                gen = next;
            }
            best = best.min(gen.total())
        }
        sum += best;
        curr = tgt;
    }
    sum
}

fn complexity(code: &str, dir_bots: usize) -> usize {
    &code[0..3].parse().unwrap() * shortest_path(code, dir_bots)
}

fn either_part(input: &str, dir_bots: usize) -> usize {
    input.lines().map(|l| complexity(l, dir_bots)).sum()
}

fn part_one(input: &str) -> usize {
    either_part(input, 2)
}

fn part_two(input: &str) -> usize {
    either_part(input, 25)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"029A
980A
179A
456A
379A"#;

    #[test]
    fn example_1a_0() {
        assert_eq!(12, shortest_path("029A", 0));
    }

    #[test]
    fn example_1a_1() {
        assert_eq!(28, shortest_path("029A", 1));
    }

    #[test]
    fn example_1a_2() {
        assert_eq!(68, shortest_path("029A", 2));
    }

    #[test]
    fn example_1a() {
        assert_eq!(1972, part_one("029A"));
    }

    #[test]
    fn example_1e_0() {
        assert_eq!(14, shortest_path("379A", 0));
    }

    #[test]
    fn example_1e_1() {
        assert_eq!(28, shortest_path("379A", 1));
    }

    #[test]
    fn example_1e_2() {
        assert_eq!(64, shortest_path("379A", 2));
    }

    #[test]
    fn example_1e() {
        assert_eq!(24256, part_one("379A"));
    }

    #[test]
    fn example_1() {
        assert_eq!(r"126384", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 21, do_solve).unwrap();
    }
}
