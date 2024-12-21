use crate::hist::Histogram;
use crate::Part;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let track = parse(input);
    tx.send(Part::A(part_one(&track).to_string())).unwrap();
    tx.send(Part::B(part_two(&track).to_string())).unwrap();
}

type Pt = (usize, usize);

#[derive(Debug, Eq, PartialEq)]
struct Racetrack {
    start: Pt,
    finish: Pt,
    distance_from_start: HashMap<Pt, usize>,
}

impl Racetrack {
    /// I build a histogram, where the bins are the savings a cheat provides.
    fn cheat_value_histogram(&self, cheat_len: usize) -> Histogram<usize> {
        let mut hist = Histogram::new();
        for (&(x, y), &dist) in &self.distance_from_start {
            let mut add_shortcut = |b, len| {
                if let Some(d) = self.distance_from_start.get(&b) {
                    let savings = dist.abs_diff(*d);
                    if savings > len {
                        hist.increment(savings - len)
                    }
                }
            };
            // NW bearings
            for dy in 1..=y.min(cheat_len) {
                let max_x = cheat_len - dy;
                for dx in 0..=x.min(max_x) {
                    add_shortcut((x - dx, y - dy), dx + dy);
                }
            }
            // SW bearings
            for dy in 0..cheat_len {
                let max_x = cheat_len - dy;
                for dx in 1..=x.min(max_x) {
                    add_shortcut((x - dx, y + dy), dx + dy);
                }
            }
        }
        hist
    }
}

fn parse(input: &str) -> Racetrack {
    let mut track = HashSet::new();
    let mut start = None;
    let mut finish = None;
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '#' {
                continue;
            } else if c == 'S' {
                start = Some((x, y))
            } else if c == 'E' {
                finish = Some((x, y))
            }
            track.insert((x, y));
        }
    }
    let start = start.unwrap();
    Racetrack {
        distance_from_start: distances_from(&track, start),
        start,
        finish: finish.unwrap(),
    }
}

fn distances_from(track: &HashSet<Pt>, start: Pt) -> HashMap<Pt, usize> {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0_usize));
    let mut distances = HashMap::new();
    while let Some((p, steps)) = queue.pop_front() {
        if *distances.entry(p).or_insert(steps) < steps {
            continue;
        }
        let ns = steps + 1;
        let (x, y) = p;
        for n in [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
            if track.contains(&n) {
                queue.push_back((n, ns));
            }
        }
    }
    distances
}

fn either_part_parameterized(track: &Racetrack, cheat_len: usize, min_savings: usize) -> usize {
    let hist = track.cheat_value_histogram(cheat_len);
    hist.into_iter()
        .filter(|(s, _)| *s >= min_savings)
        .map(|(_, n)| n)
        .sum()
}

fn part_one(track: &Racetrack) -> usize {
    either_part_parameterized(track, 2, 100)
}

fn part_two(track: &Racetrack) -> usize {
    either_part_parameterized(track, 20, 100)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hist::IntoHistogram;
    use lazy_static::lazy_static;

    const EXAMPLE_0: &str = r"#####
#...#
#S#.#
###.#
#E#.#
#.#.#
#...#
#####";

    lazy_static! {
        static ref RACETRACK_0: Racetrack = Racetrack {
            start: (1, 2),
            finish: (1, 4),
            distance_from_start: HashMap::from([
                ((1, 2), 0),
                ((1, 1), 1),
                ((2, 1), 2),
                ((3, 1), 3),
                ((3, 2), 4),
                ((3, 3), 5),
                ((3, 4), 6),
                ((3, 5), 7),
                ((3, 6), 8),
                ((2, 6), 9),
                ((1, 6), 10),
                ((1, 5), 11),
                ((1, 4), 12)
            ]),
        };
    }

    #[test]
    fn parse_0() {
        assert_eq!(*RACETRACK_0, parse(EXAMPLE_0));
    }

    #[test]
    fn value_histogram_0() {
        let track = parse(EXAMPLE_0);
        assert_eq!(
            [2, 2, 4, 10].into_histogram(),
            track.cheat_value_histogram(2)
        );
    }

    const EXAMPLE_1: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    #[test]
    fn value_histogram_1() {
        let mut expected = Histogram::new();
        expected.add(2, 14);
        expected.add(4, 14);
        expected.add(6, 2);
        let track = parse(EXAMPLE_1);
        expected.add(8, 4);
        expected.add(10, 2);
        expected.add(12, 3);
        expected.increment(20);
        expected.increment(36);
        expected.increment(38);
        expected.increment(40);
        expected.increment(64);
        assert_eq!(expected, track.cheat_value_histogram(2));

        let hist = track.cheat_value_histogram(20);
        assert_eq!(32, hist[&50]);
        assert_eq!(31, hist[&52]);
        assert_eq!(29, hist[&54]);
        assert_eq!(39, hist[&56]);
        assert_eq!(25, hist[&58]);
        assert_eq!(23, hist[&60]);
        assert_eq!(20, hist[&62]);
        assert_eq!(19, hist[&64]);
        assert_eq!(12, hist[&66]);
        assert_eq!(14, hist[&68]);
        assert_eq!(12, hist[&70]);
        assert_eq!(22, hist[&72]);
        assert_eq!(4, hist[&74]);
        assert_eq!(3, hist[&76]);
    }

    #[test]
    fn example_1() {
        let track = parse(EXAMPLE_1);
        assert_eq!(8, either_part_parameterized(&track, 2, 11));
        assert_eq!(29, either_part_parameterized(&track, 20, 72));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 20, do_solve).unwrap();
    }
}
