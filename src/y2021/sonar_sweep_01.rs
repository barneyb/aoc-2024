use crate::Part;
use std::collections::VecDeque;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    let mut depths = input.lines().map(|l| l.parse::<usize>().unwrap());
    let mut prev = depths.next().unwrap();
    let mut count = 0;
    for depth in depths {
        if depth > prev {
            count += 1;
        }
        prev = depth;
    }
    count
}

fn part_two(input: &str) -> usize {
    let mut depths = input.lines().map(|l| l.parse::<usize>().unwrap());
    let mut window = VecDeque::new();
    window.push_back(depths.next().unwrap());
    window.push_back(depths.next().unwrap());
    window.push_back(depths.next().unwrap());
    let mut sum: usize = window.iter().sum();
    let mut prev = sum;
    let mut count = 0;
    for d in depths {
        sum -= window.pop_front().unwrap();
        sum += d;
        window.push_back(d);
        if sum > prev {
            count += 1;
        }
        prev = sum;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"199
200
208
210
200
207
240
269
260
263"#;

    #[test]
    fn example_1() {
        assert_eq!(r"7", part_one(EXAMPLE_1).to_string());
        assert_eq!(r"5", part_two(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2021, 1, do_solve).unwrap();
    }
}
