use crate::hist::Histogram;
use crate::Part;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let stones = parse(input);
    tx.send(Part::Parse()).unwrap();
    tx.send(Part::A(part_one(&stones).to_string())).unwrap();
    tx.send(Part::B(part_two(&stones).to_string())).unwrap();
}

type Stones = Vec<usize>;

fn parse(input: &str) -> Stones {
    input
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part_one(stones: &Stones) -> usize {
    part_n(stones, 25)
}

fn part_n(stones: &Stones, n: u32) -> usize {
    let mut hist: Histogram<usize> = stones.iter().map(|&s| s).collect();
    let mut known_blinks: HashMap<usize, Vec<usize>> = HashMap::new();
    for _ in 1..=n {
        let mut next = Histogram::new();
        for (s, count) in hist {
            for n in known_blinks.entry(s).or_insert_with(|| {
                if s == 0 {
                    vec![1]
                } else {
                    let len = s.to_string().len();
                    if len % 2 == 0 {
                        let splitter = 10_usize.pow(len as u32 / 2);
                        vec![s / splitter, s % splitter]
                    } else {
                        vec![s * 2024]
                    }
                }
            }) {
                next.add(*n, count);
            }
        }
        hist = next;
    }
    hist.counts().sum()
}

fn part_two(stones: &Stones) -> usize {
    part_n(stones, 75)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"125 17"#;

    fn blink(stones: &Stones) -> Stones {
        let mut next = Vec::with_capacity(stones.len() * 3 / 2);
        for &s in stones {
            if s == 0 {
                next.push(1);
            } else {
                let len = s.to_string().len();
                if len % 2 == 0 {
                    let splitter = 10_usize.pow(len as u32 / 2);
                    next.push(s / splitter);
                    next.push(s % splitter);
                } else {
                    next.push(s * 2024);
                }
            }
        }
        next
    }

    #[test]
    fn example_1() {
        let stones = parse(EXAMPLE_1);
        // the computational way
        assert_eq!(r"55312", part_one(&stones).to_string());
        // the iterative way
        let mut stones = blink(&stones);
        for _ in 2..=25 {
            stones = blink(&stones);
        }
        assert_eq!(55312, stones.len());
    }

    #[test]
    fn test_part_n() {
        let stones = parse(EXAMPLE_1);
        assert_eq!(r"55312", part_n(&stones, 25).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 11, do_solve).unwrap();
    }
}
