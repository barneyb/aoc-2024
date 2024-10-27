use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn turn_left(&self) -> Heading {
        use Heading::*;

        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    fn turn_right(&self) -> Heading {
        use Heading::*;

        match self {
            West => North,
            North => East,
            East => South,
            South => West,
        }
    }
}

pub fn part_one(input: &str) -> u32 {
    use Heading::*;

    let mut h = North;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for ins in input.split(", ") {
        h = match &ins[0..1] {
            "L" => h.turn_left(),
            "R" => h.turn_right(),
            c => panic!("Unrecognized '{c:?}'"),
        };
        let d: i32 = ins[1..].parse().expect(&format!(
            "Instruction '{ins}' should have a numeric distance."
        ));
        match h {
            North => y -= d,
            East => x += d,
            South => y += d,
            West => x -= d,
        }
    }
    x.unsigned_abs() + y.unsigned_abs()
}

pub fn part_two(input: &str) -> u32 {
    use Heading::*;

    let mut h = North;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut visited = HashSet::new();
    for ins in input.split(", ") {
        h = match &ins[0..1] {
            "L" => h.turn_left(),
            "R" => h.turn_right(),
            c => panic!("Unrecognized '{c:?}'"),
        };
        let d: i32 = ins[1..].parse().expect(&format!(
            "Instruction '{ins}' should have a numeric distance."
        ));
        for _ in 0..d {
            match h {
                North => y -= 1,
                East => x += 1,
                South => y += 1,
                West => x -= 1,
            }
            if !visited.insert((x, y)) {
                return x.unsigned_abs() + y.unsigned_abs();
            }
        }
    }
    panic!("Path never intersected itself?!")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(5, part_one("R2, L3"));
        assert_eq!(2, part_one("R2, R2, R2"));
        assert_eq!(12, part_one("R5, L5, R5, R3"));
        assert_eq!(8, part_one("R8, R4, R4, R8"));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(4, part_two("R8, R4, R4, R8"));
    }

    #[test]
    fn test_real_input() {
        use crate::{with_input, Part};
        with_input(2016, 1, |input, tx| {
            tx.send(Part::A(Box::new(part_one(input)))).unwrap();
            tx.send(Part::B(Box::new(part_two(input)))).unwrap();
        })
        .unwrap();
    }
}
