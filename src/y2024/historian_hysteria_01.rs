use crate::hist::IntoHistogram;
use crate::Part;
use std::iter::zip;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let model = parse(input);
    tx.send(Part::Parse(String::new())).unwrap();
    tx.send(Part::A(part_one(&model).to_string())).unwrap();
    tx.send(Part::B(part_two(&model).to_string())).unwrap();
}

type Model = (Vec<usize>, Vec<usize>);

fn parse(input: &str) -> Model {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    for line in input.lines() {
        let mut words = line.split_ascii_whitespace();
        left.push(words.next().unwrap().parse().unwrap());
        right.push(words.next().unwrap().parse().unwrap());
    }
    left.sort();
    right.sort();
    (left, right)
}

fn part_one((left, right): &Model) -> usize {
    zip(left, right).map(|(l, r)| l.abs_diff(*r)).sum()
}

fn part_two((left, right): &Model) -> usize {
    let hist = right.into_histogram();
    left.iter()
        .filter(|&l| hist.contains_key(l))
        .map(|l| l * hist.count(l))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn example_1() {
        let model = parse(EXAMPLE_1);
        assert_eq!(r"11", part_one(&model).to_string());
        assert_eq!(r"31", part_two(&model).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 1, do_solve).unwrap();
    }
}
