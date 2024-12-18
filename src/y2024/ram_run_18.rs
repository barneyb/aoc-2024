use crate::Part;
use std::collections::{HashSet, VecDeque};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let bytes = parse(input);
    tx.send(Part::Parse()).unwrap();
    tx.send(Part::A(part_one(&bytes).to_string())).unwrap();
    tx.send(Part::B(part_two(&bytes).to_string())).unwrap();
}

type Pt = (usize, usize);

fn parse(input: &str) -> Vec<Pt> {
    input
        .lines()
        .map(|s| s.split(',').map(|n| n.parse().unwrap()))
        .map(|mut ns| (ns.next().unwrap(), ns.next().unwrap()))
        .collect()
}

fn part_one(bytes: &Vec<Pt>) -> usize {
    part_one_parameterized(bytes, 70, 1024)
}

fn part_one_parameterized(bytes: &Vec<Pt>, max: usize, count: usize) -> usize {
    let corruption: HashSet<&Pt> = bytes.iter().take(count).collect();
    either_part(&corruption, max).unwrap()
}

fn either_part(corruption: &HashSet<&Pt>, max: usize) -> Option<usize> {
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

fn part_two(bytes: &Vec<Pt>) -> String {
    part_two_parameterized(bytes, 70, 1024)
}

fn part_two_parameterized(bytes: &Vec<Pt>, max: usize, count: usize) -> String {
    let mut corruption: HashSet<&Pt> = bytes.iter().take(count).collect();
    let mut lo = count + 1;
    let mut hi = bytes.len();
    while lo < hi - 1 {
        let mid = (hi + lo) / 2;
        for b in lo..mid {
            corruption.insert(&bytes[b]);
        }
        if let Some(_) = either_part(&corruption, max) {
            lo = mid
        } else {
            for b in lo..mid {
                corruption.remove(&bytes[b]);
            }
            hi = mid - 1;
        }
    }
    let b = bytes[lo];
    format!("{},{}", b.0, b.1)
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
        let bytes = parse(EXAMPLE_1);
        assert_eq!(22, part_one_parameterized(&bytes, 6, 12));
        assert_eq!(r"6,1", part_two_parameterized(&bytes, 6, 12).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 18, do_solve).unwrap();
    }
}
