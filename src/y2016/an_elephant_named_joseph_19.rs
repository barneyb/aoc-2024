use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    let mut circle = build_circle(input);
    let mut count = circle.len() - 1;
    let mut curr = 1;
    while count > 1 {
        let to_kill = circle[curr];
        let following = circle[to_kill];
        circle[curr] = following;
        curr = following;
        count -= 1;
    }
    curr
}

fn build_circle(input: &str) -> Vec<usize> {
    let count: usize = input.parse().unwrap();
    let mut list = Vec::with_capacity(count + 1);
    list.push(0); // there's no elf 0 - waste a slot
    for i in 1..count {
        list.push(i + 1)
    }
    list.push(1); // link into ring
    list
}

fn part_two(input: &str) -> usize {
    let mut circle = build_circle(input);
    let mut count = circle.len() - 1;
    let mut curr = 1;
    let mut prev_opp = count / 2;
    while count > 1 {
        let to_kill = circle[prev_opp];
        let following = circle[to_kill];
        circle[prev_opp] = following;
        if count % 2 == 1 {
            prev_opp = following;
        }
        curr = circle[curr];
        count -= 1;
    }
    curr
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(3, part_one("5"));
        assert_eq!(5, part_one("6"));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, part_two("5"));
        assert_eq!(3, part_two("6"));
        assert_eq!(3, part_two("12"));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2016, 19, do_solve).unwrap();
    }
}
