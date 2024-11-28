use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    let mut crabs: Vec<usize> = input
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    crabs.sort();
    let mut lo = crabs[0];
    let mut hi = crabs[crabs.len() - 1];
    while lo < hi {
        let mid = (lo + hi) / 2;
        if is_positive_slope(&crabs, mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    // sorta silly to recompute, but whatever
    fuel_to(&crabs, lo)
}

fn is_positive_slope(crabs: &Vec<usize>, pos: usize) -> bool {
    let a = fuel_to(crabs, pos);
    let b = fuel_to(crabs, pos + 1);
    a < b
}

fn fuel_to(crabs: &Vec<usize>, pos: usize) -> usize {
    crabs.iter().map(|c| c.abs_diff(pos)).sum()
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"16,1,2,0,4,2,7,1,2,14"#;

    #[test]
    fn example_1() {
        assert_eq!(r"37", part_one(EXAMPLE_1).to_string());
        // assert_eq!(r"168", part_two(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2021, 7, do_solve).unwrap();
    }
}
