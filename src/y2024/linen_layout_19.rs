use crate::hist::Histogram;
use crate::Part;
use regex::{Regex, RegexSet};
use std::collections::BTreeSet;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let onsen = Onsen::new(input);
    tx.send(Part::A(part_one(&onsen).to_string())).unwrap();
    tx.send(Part::B(part_two(&onsen).to_string())).unwrap();
}

struct Onsen<'a> {
    towels: Vec<&'a str>,
    designs: Vec<&'a str>,
}

impl<'a> Onsen<'a> {
    fn new(input: &str) -> Onsen {
        let mut lines = input.lines();
        let towels: Vec<_> = lines.next().unwrap().split(',').map(|s| s.trim()).collect();
        lines.next();
        Onsen {
            towels,
            designs: lines.collect(),
        }
    }
}

fn part_one(onsen: &Onsen) -> usize {
    let mut re = "^(".to_string();
    for (i, t) in onsen.towels.iter().enumerate() {
        if i > 0 {
            re.push('|')
        }
        re.push_str(t);
    }
    re.push_str(")+$");
    let re = Regex::new(&re).unwrap();
    onsen.designs.iter().filter(|s| re.is_match(s)).count()
}

fn part_two(onsen: &Onsen) -> usize {
    let prefixed_towels: Vec<_> = onsen.towels.iter().map(|t| "^".to_string() + t).collect();
    let re_set = RegexSet::new(&prefixed_towels).unwrap();
    let mut hist = Histogram::new();
    let mut queue = BTreeSet::new();
    onsen
        .designs
        .iter()
        .map(|&design| {
            hist.clear();
            queue.clear();
            hist.increment(0);
            queue.insert(0);
            while let Some(idx) = queue.pop_first() {
                if idx == design.len() {
                    break;
                }
                let curr = hist.count(&idx);
                for m in re_set.matches(&design[idx..]) {
                    let i = idx + onsen.towels[m].len();
                    hist.add(i, curr);
                    queue.insert(i);
                }
            }
            hist.count(&design.len())
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    #[test]
    fn example_1() {
        let onsen = Onsen::new(EXAMPLE_1);
        assert_eq!(r"6", part_one(&onsen).to_string());
        assert_eq!(r"16", part_two(&onsen).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 19, do_solve).unwrap();
    }
}
