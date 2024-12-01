use crate::Part;
use std::collections::HashMap;
use std::iter::zip;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let (left, right) = parse(input);
    tx.send(Part::Parse(String::new())).unwrap();
    tx.send(Part::A(part_one(&left, &right).to_string()))
        .unwrap();
    tx.send(Part::B(part_two(&left, &right).to_string()))
        .unwrap();
}

fn part_one(left: &Vec<usize>, right: &Vec<usize>) -> usize {
    zip(left, right).map(|(l, r)| l.abs_diff(*r)).sum()
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
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

fn part_two(left: &Vec<usize>, right: &Vec<usize>) -> usize {
    let mut hist: HashMap<usize, usize> = HashMap::new();
    for &r in right {
        *hist.entry(r).or_default() += 1
    }
    left.iter().map(|&l| l * *hist.entry(l).or_default()).sum()
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
        let (left, right) = parse(EXAMPLE_1);
        assert_eq!(r"11", part_one(&left, &right).to_string());
        assert_eq!(r"31", part_two(&left, &right).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 1, do_solve).unwrap();
    }
}
