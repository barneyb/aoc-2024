use crate::Part;
use std::ops::{Add, Mul};
use std::str::FromStr;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let equations: Vec<_> = parse(input);
    tx.send(Part::Parse()).unwrap();
    tx.send(Part::A(part_one(&equations).to_string())).unwrap();
    tx.send(Part::B(part_two(&equations).to_string())).unwrap();
}

fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|l| Equation::from_str(l).unwrap())
        .collect()
}

struct Equation {
    answer: usize,
    terms: Vec<usize>,
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        Ok(Equation {
            answer: parts.next().unwrap().parse().unwrap(),
            terms: parts
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect(),
        })
    }
}

impl Equation {
    fn is_valid<Op>(&self, operators: &[Op]) -> bool
    where
        Op: Fn(usize, usize) -> usize,
    {
        let mut curr = Vec::from([self.terms[0]]);
        let mut remaining = self.terms.len() - 1;
        for &right in &self.terms[1..] {
            remaining -= 1;
            let mut next = Vec::with_capacity(curr.len() * 2);
            for left in curr {
                for op in operators {
                    let v = op(left, right);
                    if v > self.answer {
                        continue;
                    }
                    // If the last term is a one, we'll hit the answer "too
                    // early", since multiplying by one will make it valid.
                    if remaining == 0 && v == self.answer {
                        return true;
                    }
                    next.push(v);
                }
            }
            curr = next;
        }
        false
    }
}

fn either_part<F>(equations: &Vec<Equation>, test: F) -> usize
where
    F: Fn(&&Equation) -> bool,
{
    equations.iter().filter(test).map(|e| e.answer).sum()
}

fn part_one(equations: &Vec<Equation>) -> usize {
    let ops = vec![usize::add, usize::mul];
    either_part(equations, |e| e.is_valid(&ops))
}

fn concat(a: usize, b: usize) -> usize {
    if b < 10 {
        a * 10 + b
    } else if b < 100 {
        a * 100 + b
    } else {
        // nothing over a thousand
        a * 1000 + b
    }
}

fn part_two(equations: &Vec<Equation>) -> usize {
    let ops = vec![usize::add, usize::mul, concat];
    either_part(equations, |e| e.is_valid(&ops))
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    impl Equation {
        fn new(answer: usize, terms: Vec<usize>) -> Equation {
            Equation { answer, terms }
        }
    }

    #[test]
    fn test_is_valid() {
        assert!(Equation::new(292, vec![11, 6, 16, 20],).is_valid(&vec![usize::add, usize::mul]));
        assert!(!Equation::new(161011, vec![16, 10, 13],).is_valid(&vec![usize::add, usize::mul]));
    }

    #[test]
    fn test_concat() {
        assert_eq!(156, concat(15, 6));
        assert_eq!(516, concat(5, 16));
        assert_eq!(123456, concat(123, 456));
    }

    #[test]
    fn test_is_valid_with_concat() {
        assert!(Equation::new(7290, vec![6, 8, 6, 15],).is_valid(&vec![
            usize::add,
            usize::mul,
            concat
        ]));
        assert!(!Equation::new(161011, vec![16, 10, 13],).is_valid(&vec![
            usize::add,
            usize::mul,
            concat
        ]));
    }

    #[test]
    fn example_1() {
        let eqs = parse(EXAMPLE_1);
        assert_eq!(r"3749", part_one(&eqs).to_string());
        assert_eq!(r"11387", part_two(&eqs).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 7, do_solve).unwrap();
    }
}
