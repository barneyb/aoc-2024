use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
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

fn is_safe(report: &Vec<usize>) -> bool {
    if report.len() == 1 {
        return true;
    }
    let mut prev = report[0];
    if prev < report[1] {
        // increasing
        for &curr in &report[1..] {
            if curr <= prev || curr - prev > 3 {
                return false;
            }
            prev = curr;
        }
    } else {
        // decreasing
        for &curr in &report[1..] {
            if curr >= prev || prev - curr > 3 {
                return false;
            }
            prev = curr;
        }
    }
    true
}

// fn part_two(input: &str) -> usize {
//     99999
// }

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
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 2, do_solve).unwrap();
    }
}
