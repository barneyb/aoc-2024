use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    either_part(input, |d| d)
}

fn either_part<F>(input: &str, cost_to_move: F) -> usize
where
    F: Fn(usize) -> usize,
{
    let mut crabs: Vec<_> = input
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    crabs.sort();
    let fuel_to = |pos: usize| crabs.iter().map(|&c| cost_to_move(pos.abs_diff(c))).sum();
    let mut lo = crabs[0];
    let mut hi = crabs[crabs.len() - 1];
    while lo < hi {
        let mid = (lo + hi) / 2;
        let a = fuel_to(mid);
        let b = fuel_to(mid + 1);
        if a < b {
            hi = mid;
        } else {
            lo = mid + 1;
            if a == b {
                // no curvature
                break;
            }
        }
    }
    // sorta silly to recompute, but whatever
    fuel_to(lo)
}

fn part_two(input: &str) -> usize {
    either_part(input, |d| d * (d + 1) / 2)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"16,1,2,0,4,2,7,1,2,14"#;

    #[test]
    fn example_1() {
        assert_eq!(r"37", part_one(EXAMPLE_1).to_string());
        assert_eq!(r"168", part_two(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2021, 7, do_solve).unwrap();
    }
}
