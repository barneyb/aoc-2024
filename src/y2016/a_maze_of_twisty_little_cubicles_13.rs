use crate::Part;
use std::collections::{HashSet, VecDeque};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let seed: u32 = input.parse().expect(&format!(
        "Input '{input}' should have been a natural number"
    ));
    steps_to((31, 39), seed, Some(tx));
}

fn steps_to(goal: (u32, u32), seed: u32, opt_tx: Option<Sender<Part>>) -> (usize, usize) {
    let mut queue = VecDeque::from([((1, 1), 0)]);
    let mut considered = HashSet::new();
    let mut within_fifty = HashSet::new();
    while let Some((p, steps)) = queue.pop_front() {
        if steps <= 50 {
            within_fifty.insert(p);
        }
        if p == goal {
            if let Some(tx) = &opt_tx {
                tx.send(Part::A(steps.to_string())).unwrap();
                tx.send(Part::B(within_fifty.len().to_string())).unwrap();
            }
            return (steps, within_fifty.len());
        }
        let (x, y) = p;
        let s = steps + 1;
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

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: u32 = 10;

    #[test]
    fn example_1() {
        assert_eq!(11, steps_to((7, 4), EXAMPLE_1, None).0);
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
        crate::with_input(2016, 13, do_solve).unwrap();
    }
}
