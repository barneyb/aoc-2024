use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    let mut chars: Vec<_> = input.chars().collect();
    let mut i = chars.len() - 1;
    while i > 0 {
        i -= 1;
        let a = chars[i];
        let b = chars[i + 1];
        if if a.is_ascii_uppercase() {
            b == a.to_ascii_lowercase()
        } else {
            b == a.to_ascii_uppercase()
        } {
            chars.drain(i..=i + 1);
            if i >= chars.len() && i > 0 {
                i -= 1;
            }
        }
    }
    chars.len()
}

fn part_two(input: &str) -> usize {
    ('a'..='z')
        .map(|c| input.replace(&[c, c.to_ascii_uppercase()], ""))
        .filter(|s| s.len() < input.len())
        .map(|s| part_one(&s))
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"aA"#;

    const EXAMPLE_2: &str = r#"abBA"#;

    const EXAMPLE_3: &str = r#"abAB"#;

    const EXAMPLE_4: &str = r#"aabAAB"#;

    const EXAMPLE_5: &str = r#"dabAcCaCBAcCcaDA"#;

    #[test]
    fn example_1() {
        assert_eq!(r"0", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"0", part_one(EXAMPLE_2).to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(r"4", part_one(EXAMPLE_3).to_string());
    }

    #[test]
    fn example_4() {
        assert_eq!(r"6", part_one(EXAMPLE_4).to_string());
    }

    #[test]
    fn example_5() {
        assert_eq!(r"10", part_one(EXAMPLE_5).to_string());
        assert_eq!(r"4", part_two(EXAMPLE_5).to_string());
    }

    #[test]
    fn react_twice_at_start() {
        assert_eq!(2, part_one("aABbxx"));
    }

    #[test]
    fn react_twice_at_end() {
        assert_eq!(2, part_one("xxaABb"));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2018, 5, do_solve).unwrap();
    }
}
