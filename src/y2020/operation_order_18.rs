use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

#[derive(Debug, Default)]
struct Calculator {
    vals: Vec<usize>,
    ops: Vec<char>,
}

impl Calculator {
    fn new() -> Calculator {
        Default::default()
    }

    fn clear(&mut self) {
        self.vals.clear();
        self.ops.clear();
    }

    fn do_ops(&mut self) {
        while let Some('+' | '*') = self.ops.last() {
            let b = self.vals.pop().unwrap();
            let a = self.vals.pop().unwrap();
            self.vals.push(match self.ops.pop() {
                Some('+') => a + b,
                Some('*') => a * b,
                it => panic!("what?! an {it:?}?!"),
            });
        }
    }

    fn calculate(&mut self, expr: &str) -> usize {
        for c in expr.chars() {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    self.vals.push(c.to_digit(10).unwrap() as usize)
                }
                '+' | '*' => {
                    self.do_ops();
                    self.ops.push(c);
                }
                '(' => self.ops.push(c),
                ')' => {
                    self.do_ops();
                    self.ops.pop(); // the open paren
                }
                ' ' => { /* ignore */ }
                c => panic!("Unexpected '{c}'"),
            }
        }
        self.do_ops();
        debug_assert_eq!(0, self.ops.len());
        debug_assert_eq!(1, self.vals.len());
        self.vals.pop().unwrap()
    }
}

fn part_one(input: &str) -> usize {
    let mut calc = Calculator::new();
    input
        .lines()
        .map(|expr| {
            calc.clear();
            calc.calculate(expr)
        })
        .sum()
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"1 + 2 * 3 + 4 * 5 + 6"#;

    const EXAMPLE_2: &str = r#"1 + (2 * 3) + (4 * (5 + 6))"#;

    const EXAMPLE_3: &str = r#"2 * 3 + (4 * 5)"#;

    const EXAMPLE_4: &str = r#"5 + (8 * 3 + 9 + 3 * 4 * 3)"#;

    const EXAMPLE_5: &str = r#"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"#;

    const EXAMPLE_6: &str = r#"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"#;

    #[test]
    fn example_1() {
        assert_eq!(r"71", part_one(EXAMPLE_1).to_string());
        // assert_eq!(r"231", part_two(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"51", part_one(EXAMPLE_2).to_string());
        // assert_eq!(r"51", part_two(EXAMPLE_2).to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(r"26", part_one(EXAMPLE_3).to_string());
        // assert_eq!(r"46", part_two(EXAMPLE_3).to_string());
    }

    #[test]
    fn example_4() {
        assert_eq!(r"437", part_one(EXAMPLE_4).to_string());
        // assert_eq!(r"1445", part_two(EXAMPLE_4).to_string());
    }

    #[test]
    fn example_5() {
        assert_eq!(r"12240", part_one(EXAMPLE_5).to_string());
        // assert_eq!(r"669060", part_two(EXAMPLE_5).to_string());
    }

    #[test]
    fn example_6() {
        assert_eq!(r"13632", part_one(EXAMPLE_6).to_string());
        // assert_eq!(r"23340", part_two(EXAMPLE_6).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2020, 18, do_solve).unwrap();
    }
}
