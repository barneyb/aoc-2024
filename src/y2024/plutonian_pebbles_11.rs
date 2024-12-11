use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let stones = parse(input);
    tx.send(Part::Parse()).unwrap();
    tx.send(Part::A(part_one(&stones).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

type Stones = Vec<usize>;

fn parse(input: &str) -> Stones {
    input
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part_one(stones: &Stones) -> usize {
    let mut stones = blink(stones);
    for _ in 2..=25 {
        stones = blink(&stones);
    }
    stones.len()
}

fn blink(stones: &Stones) -> Stones {
    let mut next = Vec::with_capacity(stones.len() * 3 / 2);
    for &s in stones {
        if s == 0 {
            next.push(1);
        } else {
            let len = s.to_string().len();
            if len % 2 == 0 {
                let splitter = 10_usize.pow(len as u32 / 2);
                next.push(s / splitter);
                next.push(s % splitter);
            } else {
                next.push(s * 2024);
            }
        }
    }
    next
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"125 17"#;

    #[test]
    fn example_1() {
        let stones = parse(EXAMPLE_1);
        assert_eq!(r"55312", part_one(&stones).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 11, do_solve).unwrap();
    }
}
