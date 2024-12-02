use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    either_part(input, is_safe)
}

fn either_part<F>(input: &str, is_safe: F) -> usize
where
    F: Fn(&Vec<i32>) -> bool,
{
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .filter(is_safe)
        .count()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mut diffs = Vec::with_capacity(report.len() - 1);
    let mut prev = report[0];
    for &curr in &report[1..] {
        diffs.push(prev - curr);
        prev = curr;
    }
    if diffs[0] > 0 {
        if diffs.iter().all(|&d| 1 <= d && d <= 3) {
            return true;
        }
    } else if diffs.iter().all(|&d| -3 <= d && d <= -1) {
        return true;
    }
    false
}

fn part_two(input: &str) -> usize {
    either_part(input, |r| {
        if is_safe(r) {
            return true;
        }
        let mut clone = r.clone();
        for i in 0..clone.len() {
            let level = clone.remove(i);
            if is_safe(&clone) {
                return true;
            }
            clone.insert(i, level);
        }
        return false;
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn example_1() {
        assert_eq!(r"2", part_one(EXAMPLE_1).to_string());
        assert_eq!(r"4", part_two(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_dampen_first() {
        assert_eq!(0, part_one("9999 1 2 3"));
        assert_eq!(1, part_two("9999 1 2 3"));
    }

    #[test]
    fn test_dampen_second() {
        assert_eq!(0, part_one("1 9999 2 3"));
        assert_eq!(1, part_two("1 9999 2 3"));
    }

    #[test]
    fn test_dampen_last() {
        assert_eq!(0, part_one("1 2 3 9999"));
        assert_eq!(1, part_two("1 2 3 9999"));
    }

    #[test]
    fn test_dampen_penultimate() {
        assert_eq!(0, part_one("1 2 3 9999 5"));
        assert_eq!(1, part_two("1 2 3 9999 5"));
    }

    #[test]
    fn test_dampen_middle() {
        assert_eq!(0, part_one("1 2 9999 4 5"));
        assert_eq!(1, part_two("1 2 9999 4 5"));
    }

    #[test]
    fn test_dampen_direction_flip() {
        assert_eq!(0, part_one("1 2 4 3 5"));
        assert_eq!(1, part_two("1 2 4 3 5"));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 2, do_solve).unwrap();
    }
}
