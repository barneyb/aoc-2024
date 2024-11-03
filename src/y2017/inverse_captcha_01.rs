use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> u32 {
    let mut prev = input.chars().rev().next().unwrap();
    let mut sum = 0;
    for c in input.chars() {
        if c == prev {
            sum += c.to_digit(10).unwrap();
        }
        prev = c;
    }
    sum
}

fn part_two(input: &str) -> u32 {
    let chars: Vec<char> = input.chars().collect();
    let (first, second) = chars.split_at(chars.len() / 2);
    let mut sum = 0;
    for (c, d) in first.iter().zip(second) {
        if c == d {
            sum += c.to_digit(10).unwrap() * 2;
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"1122"#;

    const EXAMPLE_2: &str = r#"1111"#;

    const EXAMPLE_3: &str = r#"1234"#;

    const EXAMPLE_4: &str = r#"91212129"#;

    const EXAMPLE_5: &str = r#"1212"#;

    const EXAMPLE_6: &str = r#"1221"#;

    const EXAMPLE_7: &str = r#"123425"#;

    const EXAMPLE_8: &str = r#"123123"#;

    const EXAMPLE_9: &str = r#"12131415"#;

    #[test]
    fn example_1() {
        assert_eq!(r"3", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"4", part_one(EXAMPLE_2).to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(r"0", part_one(EXAMPLE_3).to_string());
    }

    #[test]
    fn example_4() {
        assert_eq!(r"9", part_one(EXAMPLE_4).to_string());
    }

    #[test]
    fn example_5() {
        assert_eq!(r"6", part_two(EXAMPLE_5).to_string());
    }

    #[test]
    fn example_6() {
        assert_eq!(r"0", part_two(EXAMPLE_6).to_string());
    }

    #[test]
    fn example_7() {
        assert_eq!(r"4", part_two(EXAMPLE_7).to_string());
    }

    #[test]
    fn example_8() {
        assert_eq!(r"12", part_two(EXAMPLE_8).to_string());
    }

    #[test]
    fn example_9() {
        assert_eq!(r"4", part_two(EXAMPLE_9).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2017, 1, do_solve).unwrap();
    }
}
