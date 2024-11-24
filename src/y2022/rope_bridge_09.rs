use crate::geom2d::Dir;
use crate::Part;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

type Pt = (i32, i32);

fn part_one(input: &str) -> usize {
    either_part(input, 1)
}

fn part_two(input: &str) -> usize {
    either_part(input, 9)
}

fn either_part(input: &str, tail_len: usize) -> usize {
    let mut head = (0, 0);
    let mut trailing = vec![head; tail_len];
    let mut visited = HashSet::from([head]);
    for line in input.lines() {
        let dir: Dir = line.into();
        let n: u32 = line[2..].parse().unwrap();
        for _ in 0..n {
            head = step(head, dir);
            let mut prev = head;
            trailing = trailing
                .iter()
                .map(|&curr| {
                    let next = drag(prev, curr);
                    prev = next;
                    next
                })
                .collect();
            visited.insert(*trailing.iter().last().unwrap());
        }
    }
    visited.len()
}

#[rustfmt::skip]
fn step(curr: Pt, dir: Dir) -> Pt {
    let (x, y) = curr;
    match dir {
        Dir::North => (x    , y - 1),
        Dir::East  => (x + 1, y    ),
        Dir::South => (x    , y + 1),
        Dir::West  => (x - 1, y    ),
    }
}

#[rustfmt::skip]
fn drag(head: Pt, tail: Pt) -> Pt {
    let (x, y) = tail;
    // The bottom four arms have an unreachable +/-2 in their second part. I've
    // left them for visual symmetry.
    #[allow(unreachable_patterns)]
    match (x - head.0, y - head.1) {
        (      2,       0)                     => (x - 1, y    ),
        (     -2,       0)                     => (x + 1, y    ),
        (      0,       2)                     => (x    , y - 1),
        (      0,      -2)                     => (x    , y + 1),
        ( 1 |  2,       2) | (      2, 1 |  2) => (x - 1, y - 1),
        (      2, -1 | -2) | ( 1 |  2,     -2) => (x - 1, y + 1),
        (     -2, -1 | -2) | (-1 | -2,     -2) => (x + 1, y + 1),
        (-1 | -2,       2) | (     -2, 1 |  2) => (x + 1, y - 1),
        _ => tail,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    const EXAMPLE_2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    #[test]
    fn example_1() {
        assert_eq!(r"13", part_one(EXAMPLE_1).to_string());
        assert_eq!(r"1", part_two(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"36", part_two(EXAMPLE_2).to_string());
    }

    #[test]
    fn test_drag() {
        // same spot
        assert_eq!((0, 0), drag((0, 0), (0, 0)));
        // en passant capture away
        assert_eq!((0, 0), drag((1, 0), (0, 0)));
        // pawn's capture away
        assert_eq!((0, 0), drag((-1, 1), (0, 0)));
        // rook's capture away
        assert_eq!((0, 1), drag((0, 2), (0, 0)));
        // knight's capture away
        assert_eq!((-1, -1), drag((-1, -2), (0, 0)));
        // bishop's capture away
        assert_eq!((1, -1), drag((2, -2), (0, 0)));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2022, 9, do_solve).unwrap();
    }
}
