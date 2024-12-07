use crate::Part;
use std::str::FromStr;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
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
    #[allow(dead_code)] // for tests
    fn new(answer: usize, terms: Vec<usize>) -> Equation {
        Equation { answer, terms }
    }

    fn is_valid(&self) -> bool {
        let mut curr = Vec::from([self.terms[0]]);
        for a in &self.terms[1..] {
            let mut next = Vec::with_capacity(curr.len() * 2);
            for b in curr {
                next.push(a + b);
                next.push(a * b);
            }
            curr = next;
        }
        curr.iter().any(|n| *n == self.answer)
    }
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|l| Equation::from_str(l).unwrap())
        .filter(|e| e.is_valid())
        .map(|e| e.answer)
        .sum()
}

// fn part_two(input: &str) -> usize {
//     99999
// }

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

    #[test]
    fn test_is_valid() {
        assert!(Equation::new(292, vec![11, 6, 16, 20],).is_valid());
        assert!(!Equation::new(161011, vec![16, 10, 13],).is_valid());
    }

    #[test]
    fn example_1() {
        assert_eq!(r"3749", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 7, do_solve).unwrap();
    }
}
