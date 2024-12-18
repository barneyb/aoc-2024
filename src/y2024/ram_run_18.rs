use crate::Part;
use std::collections::{HashSet, VecDeque};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    part_one_parameterized(input, 70, 1024).unwrap()
}

type Pt = (usize, usize);

fn part_one_parameterized(input: &str, max: usize, bytes: usize) -> Option<usize> {
    let corruption: HashSet<Pt> = input
        .lines()
        .take(bytes)
        .map(|s| s.split(',').map(|n| n.parse().unwrap()))
        .map(|mut ns| (ns.next().unwrap(), ns.next().unwrap()))
        .collect();
    let goal = (max, max);
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));
    let mut visited = HashSet::new();
    while let Some((p, steps)) = queue.pop_front() {
        if corruption.contains(&p) || !visited.insert(p) {
            continue;
        }
        if p == goal {
            return Some(steps);
        }
        let (x, y) = p;
        let steps = steps + 1;
        if x > 0 {
            queue.push_back(((x - 1, y), steps))
        }
        if y > 0 {
            queue.push_back(((x, y - 1), steps))
        }
        if x < max {
            queue.push_back(((x + 1, y), steps))
        }
        if y < max {
            queue.push_back(((x, y + 1), steps))
        }
    }
    None
}

fn part_two(input: &str) -> String {
    part_two_parameterized(input, 70, 1024)
}

fn part_two_parameterized(input: &str, max: usize, bytes: usize) -> String {
    let mut b = bytes + 1;
    while let Some(_) = part_one_parameterized(input, max, b) {
        b += 1;
    }
    input.lines().nth(b - 1).unwrap().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    #[test]
    fn example_1() {
        assert_eq!(22, part_one_parameterized(EXAMPLE_1, 6, 12).unwrap());
        assert_eq!(r"6,1", part_two_parameterized(EXAMPLE_1, 6, 12).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 18, do_solve).unwrap();
    }
}
