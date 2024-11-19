use crate::Part;
use std::collections::{BTreeSet, HashSet};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one_vec(input).to_string())).unwrap();
    tx.send(Part::A(part_one_hash(input).to_string())).unwrap();
    tx.send(Part::B(part_two_vec(input).to_string())).unwrap();
    tx.send(Part::B(part_two_tree(input).to_string())).unwrap();
}

fn part_one_vec(input: &str) -> i32 {
    let mut nums: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    nums.sort();
    for a in nums.iter() {
        let b = 2020 - a;
        if let Ok(_) = nums.binary_search(&b) {
            return *a * b;
        }
    }
    panic!("No pair sums to 2020?!")
}

fn part_one_hash(input: &str) -> i32 {
    let nums: HashSet<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    for a in nums.iter() {
        let b = 2020 - a;
        if nums.contains(&b) {
            return *a * b;
        }
    }
    panic!("No pair sums to 2020?!")
}

fn part_two_vec(input: &str) -> i32 {
    let mut nums: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    nums.sort();
    for (i, a) in nums.iter().enumerate() {
        let b_and_c = 2020 - a;
        for &b in nums[(i + 1)..].iter() {
            if b >= b_and_c {
                break;
            }
            let c = b_and_c - b;
            if let Ok(_) = nums.binary_search(&c) {
                return *a * b * c;
            }
        }
    }
    panic!("No triplet sums to 2020?!")
}

fn part_two_tree(input: &str) -> i32 {
    let nums: BTreeSet<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    for a in nums.iter() {
        let b_and_c = 2020 - a;
        for &b in nums.range(a + 1..b_and_c) {
            if b >= b_and_c {
                break;
            }
            let c = b_and_c - b;
            if nums.contains(&c) {
                return *a * b * c;
            }
        }
    }
    panic!("No triplet sums to 2020?!")
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"1721
979
366
299
675
1456"#;

    #[test]
    fn example_1() {
        assert_eq!(r"514579", part_one_vec(EXAMPLE_1).to_string());
        assert_eq!(r"514579", part_one_hash(EXAMPLE_1).to_string());
        assert_eq!(r"241861950", part_two_vec(EXAMPLE_1).to_string());
        assert_eq!(r"241861950", part_two_tree(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2020, 1, do_solve).unwrap();
    }
}
