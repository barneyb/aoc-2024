use crate::Part;
use std::sync::mpsc::Sender;
use Ins::*;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let ins = parse(input);
    tx.send(Part::A(part_one(&ins).to_string())).unwrap();
    tx.send(Part::B(part_two(&ins).to_string())).unwrap();
}

#[derive(Debug, Eq, PartialEq)]
enum Ins {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn parse(input: &str) -> Vec<Ins> {
    input
        .lines()
        .map(|l| match l.chars().next() {
            Some('f') => Forward(l[8..].parse::<usize>().unwrap()),
            Some('u') => Up(l[3..].parse::<usize>().unwrap()),
            Some('d') => Down(l[5..].parse::<usize>().unwrap()),
            c => panic!("Unrecognized {c:?}"),
        })
        .collect()
}

fn part_one(instructions: &Vec<Ins>) -> usize {
    let mut pos = 0;
    let mut depth = 0;
    for ins in instructions {
        match ins {
            Forward(n) => pos += n,
            Down(n) => depth += n,
            Up(n) => depth -= n,
        }
    }
    pos * depth
}

fn part_two(instructions: &Vec<Ins>) -> usize {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for ins in instructions {
        match ins {
            Forward(n) => {
                pos += n;
                depth += aim * n;
            }
            Down(n) => aim += n,
            Up(n) => aim -= n,
        }
    }
    pos * depth
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    const EXAMPLE_1: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

    lazy_static! {
        static ref MODEL_1: Vec<Ins> =
            vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
    }

    #[test]
    fn test_parse() {
        assert_eq!(*MODEL_1, parse(EXAMPLE_1))
    }

    #[test]
    fn example_1() {
        assert_eq!(r"150", part_one(&*MODEL_1).to_string());
        assert_eq!(r"900", part_two(&*MODEL_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2021, 2, do_solve).unwrap();
    }
}
