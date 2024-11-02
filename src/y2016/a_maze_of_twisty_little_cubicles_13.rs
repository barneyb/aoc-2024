use std::collections::{HashSet, VecDeque};

pub fn part_one(input: &str) -> usize {
    let seed: u32 = input.parse().expect(&format!(
        "Input '{input}' should have been a natural number"
    ));
    steps_to((31, 39), seed)
}

fn steps_to(goal: (u32, u32), seed: u32) -> usize {
    let mut queue = VecDeque::from([((1, 1), 0)]);
    let mut considered = HashSet::new();
    while let Some((p, s)) = queue.pop_front() {
        if p == goal {
            return s;
        }
        let (x, y) = p;
        let s = s + 1;
        if y > 0 {
            let p = (x, y - 1);
            if considered.insert(p) && is_open(p, seed) {
                queue.push_back((p, s))
            }
        }
        let p = (x + 1, y);
        if considered.insert(p) && is_open(p, seed) {
            queue.push_back((p, s))
        }
        let p = (x, y + 1);
        if considered.insert(p) && is_open(p, seed) {
            queue.push_back((p, s))
        }
        if x > 0 {
            let p = (x - 1, y);
            if considered.insert(p) && is_open(p, seed) {
                queue.push_back((p, s))
            }
        }
    }
    panic!("Failed to reach {goal:?}")
}

fn is_open(p: (u32, u32), seed: u32) -> bool {
    let (x, y) = p;
    // Shush. I copy'n'pasted it.
    ((x * x + 3 * x + 2 * x * y + y + y * y) + seed).count_ones() % 2 == 0
}

// pub fn part_two(input: &str) -> usize {
//     input.len()
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: u32 = 10;

    #[test]
    fn example_1() {
        assert_eq!(r"11", steps_to((7, 4), EXAMPLE_1).to_string());
    }

    #[test]
    fn test_wall_or_open() {
        let seed = 10;
        // x = 0
        assert!(is_open((0, 0), seed));
        assert!(is_open((0, 1), seed));
        assert!(!is_open((0, 2), seed));
        assert!(!is_open((0, 3), seed));
        assert!(is_open((0, 4), seed));
        assert!(is_open((0, 5), seed));
        assert!(!is_open((0, 6), seed));
        // x = 4
        assert!(!is_open((4, 0), seed));
        assert!(is_open((4, 1), seed));
        assert!(is_open((4, 2), seed));
        assert!(!is_open((4, 3), seed));
        assert!(is_open((4, 4), seed));
        assert!(is_open((4, 5), seed));
        assert!(!is_open((4, 6), seed));
        // y = 4
        assert!(is_open((0, 4), seed));
        assert!(!is_open((1, 4), seed));
        assert!(!is_open((2, 4), seed));
        assert!(is_open((3, 4), seed));
        assert!(is_open((4, 4), seed));
        assert!(!is_open((5, 4), seed));
        assert!(is_open((6, 4), seed));
        assert!(is_open((7, 4), seed));
        assert!(!is_open((8, 4), seed));
        assert!(is_open((9, 4), seed));
    }

    #[test]
    fn test_real_input() {
        use crate::{with_input, Part};
        with_input(2016, 13, |input, tx| {
            tx.send(Part::A(part_one(input).to_string())).unwrap();
            // tx.send(Part::B(part_two(input).to_string())).unwrap();
        })
        .unwrap();
    }
}
