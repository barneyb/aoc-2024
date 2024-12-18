use crate::Part;
use std::collections::{HashSet, VecDeque};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    part_one_parameterized(input, 70, 1024)
}

type Pt = (usize, usize);

fn part_one_parameterized(input: &str, max: usize, bytes: usize) -> usize {
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
            return steps;
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
    99999
}

// fn part_two(input: &str) -> usize {
//     99999
// }

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
        assert_eq!(r"22", part_one_parameterized(EXAMPLE_1, 6, 12).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 18, do_solve).unwrap();
    }
}
