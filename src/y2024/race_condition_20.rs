use crate::geom2d::Dir::*;
use crate::geom2d::{step, step_by};
use crate::hist::Histogram;
use crate::Part;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let track = parse(input);
    tx.send(Part::A(part_one(&track).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

type Pt = (usize, usize);

#[derive(Debug, Eq, PartialEq)]
struct Racetrack {
    track: HashSet<Pt>,
    start: Pt,
    finish: Pt,
}

impl Racetrack {
    fn is_track(&self, p: &Pt) -> bool {
        self.track.contains(p)
    }

    fn find_cheats(&self) -> Vec<(Pt, Pt)> {
        let mut cheats = Vec::new();
        for &curr in &self.track {
            if curr.0 > 1 {
                let end = step_by(curr, West, 2);
                if !self.is_track(&step(curr, West)) && self.is_track(&end) {
                    cheats.push((curr, end))
                }
            }
            if curr.1 > 1 {
                let end = step_by(curr, North, 2);
                if !self.is_track(&step(curr, North)) && self.is_track(&end) {
                    cheats.push((curr, end))
                }
            }
        }
        cheats
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
    Racetrack {
        track,
        start: start.unwrap(),
        finish: finish.unwrap(),
    }
}

fn cheat_value_histogram(track: &Racetrack) -> Histogram<usize> {
    let mut queue = VecDeque::new();
    queue.push_back((track.start, 0_usize));
    let mut visited = HashMap::new();
    while let Some((p, steps)) = queue.pop_front() {
        if *visited.entry(p).or_insert(steps) < steps {
            continue;
        }
        let ns = steps + 1;
        let (x, y) = p;
        for n in [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
            if track.track.contains(&n) {
                queue.push_back((n, ns));
            }
        }
    }
    let mut hist = Histogram::new();
    for (a, b) in track.find_cheats() {
        hist.increment(visited[&a].abs_diff(visited[&b]) - 2)
    }
    hist
}

fn part_one_parameterized(track: &Racetrack, min_savings: usize) -> usize {
    let hist = cheat_value_histogram(&track);
    hist.into_iter()
        .filter(|(s, _)| *s >= min_savings)
        .map(|(_, n)| n)
        .sum()
}

fn part_one(track: &Racetrack) -> usize {
    part_one_parameterized(track, 100)
}

// fn part_two(input: &str) -> usize {
//     99999
// }

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
            track: HashSet::from([
                (1, 2),
                (1, 1),
                (2, 1),
                (3, 1),
                (3, 2),
                (3, 3),
                (3, 4),
                (3, 5),
                (3, 6),
                (2, 6),
                (1, 6),
                (1, 5),
                (1, 4)
            ]),
            start: (1, 2),
            finish: (1, 4),
        };
    }

    #[test]
    fn parse_0() {
        assert_eq!(*RACETRACK_0, parse(EXAMPLE_0));
    }

    #[test]
    fn cheats_0() {
        let track = parse(EXAMPLE_0);
        let mut cheats = track.find_cheats();
        cheats.sort();
        assert_eq!(
            vec![
                ((1, 4), (1, 2)),
                ((3, 2), (1, 2)),
                ((3, 4), (1, 4)),
                ((3, 5), (1, 5)),
            ],
            cheats
        );
    }

    #[test]
    fn value_histogram_0() {
        let track = parse(EXAMPLE_0);
        assert_eq!(
            [2, 2, 4, 10].into_histogram(),
            cheat_value_histogram(&track)
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
        assert_eq!(expected, cheat_value_histogram(&track));
    }

    #[test]
    fn example_1() {
        let track = parse(EXAMPLE_1);
        assert_eq!(8, part_one_parameterized(&track, 11));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 20, do_solve).unwrap();
    }
}
