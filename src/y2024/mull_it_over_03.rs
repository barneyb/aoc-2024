use crate::Part;
use regex::Regex;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            (
                c[1].parse::<usize>().unwrap(),
                c[2].parse::<usize>().unwrap(),
            )
        })
        .map(|(a, b)| a * b)
        .sum()
}

fn part_two(input: &str) -> usize {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for c in re.captures_iter(input) {
        match &c[0][0..3] {
            "do(" => enabled = true,
            "don" => enabled = false,
            "mul" if enabled => {
                let a = c[1].parse::<usize>().unwrap();
                let b = c[2].parse::<usize>().unwrap();
                sum += a * b;
            }
            _ => {}
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    const EXAMPLE_2: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn example_1() {
        assert_eq!(r"161", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"48", part_two(EXAMPLE_2).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 3, do_solve).unwrap();
    }
}
