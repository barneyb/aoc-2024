use crate::hist::Histogram;
use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let track = parse(input);
    tx.send(Part::A(part_one(&track).to_string())).unwrap();
    tx.send(Part::B(part_two(&track).to_string())).unwrap();
}

type Pt = (usize, usize);

#[derive(Debug, Eq, PartialEq)]
struct Racetrack {
    start: usize,
    finish: usize,
    distance_from_start: Vec<usize>,
    width: usize,
}

impl Racetrack {
    fn to_point(&self, i: usize) -> Pt {
        (i % self.width, i / self.width)
    }

    fn to_index(&self, p: Pt) -> usize {
        p.0 + p.1 * self.width
    }

    /// I build a histogram, where the bins are the savings a cheat provides.
    fn cheat_value_histogram(&self, cheat_len: usize) -> Histogram<usize> {
        let mut hist = Histogram::new();
        for (i, dist) in self
            .distance_from_start
            .iter()
            .enumerate()
            .filter(|(_, d)| **d > 0)
        {
            let (x, y) = self.to_point(i);
            // for (&(x, y), &dist) in &self.distance_from_start {
            let mut add_shortcut = |b, len| {
                let i = self.to_index(b);
                if i >= self.distance_from_start.len() {
                    return;
                }
                let d = self.distance_from_start[i];
                if d == 0 {
                    return;
                }
                let savings = dist.abs_diff(d);
                if savings > len {
                    hist.increment(savings - len)
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
    let mut start = None;
    let mut finish = None;
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let width = grid[0].len();
    let mut track: Vec<bool> = vec![false; width * grid.len()];
    for (y, line) in grid.iter().enumerate() {
        let row_offset = y * width;
        for (x, c) in line.iter().enumerate() {
            if *c == '#' {
                continue;
            }
            let i = row_offset + x;
            if *c == 'S' {
                start = Some(i)
            } else if *c == 'E' {
                finish = Some(i)
            }
            track[i] = true;
        }
    }
    let start = start.unwrap();
    Racetrack {
        distance_from_start: distances_from(&track, width, start),
        start,
        finish: finish.unwrap(),
        width,
    }
}

fn distances_from(track: &Vec<bool>, width: usize, start: usize) -> Vec<usize> {
    let mut distances = vec![0; track.len()];
    let mut curr = start;
    let mut prev = start + 3; // can't be in the first two neighbors
    let mut dist = 0;
    'step: loop {
        dist += 1;
        distances[curr] = dist;
        for next in [curr - width, curr + 1, curr + width, curr - 1] {
            if next != prev && track[next] {
                prev = curr;
                curr = next;
                continue 'step;
            }
        }
        break;
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

    const EXAMPLE_0: &str = r"#####
#...#
#S#.#
###.#
#E#.#
#.#.#
#...#
#####";

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
