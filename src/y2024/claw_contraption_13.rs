use crate::Part;
use std::num::ParseIntError;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let contraptions = parse(input);
    tx.send(Part::A(part_one(&contraptions).to_string()))
        .unwrap();
    // tx.send(Part::Other(part_two(&contraptions).to_string())).unwrap();
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Pt {
    x: i64,
    y: i64,
}

impl Pt {
    fn new(x: i64, y: i64) -> Pt {
        Pt { x, y }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Contraption {
    a: Pt,
    b: Pt,
    prize: Pt,
}

impl Contraption {
    fn new(a: Pt, b: Pt, prize: Pt) -> Contraption {
        Contraption { a, b, prize }
    }

    /// If the machine is winnable, return the min tokens required, or `None` if
    /// not winnable.
    fn play(&self) -> Option<i64> {
        let b = (self.a.x * self.prize.y - self.a.y * self.prize.x)
            / (self.a.x * self.b.y - self.a.y * self.b.x);
        let a = (self.prize.x - b * self.b.x) / self.a.x;
        if a * self.a.x + b * self.b.x == self.prize.x
            && a * self.a.y + b * self.b.y == self.prize.y
        {
            Some(a * 3 + b)
        } else {
            None
        }
    }
}

fn to_int(cs: &[char]) -> Result<i64, ParseIntError> {
    cs.iter().collect::<String>().parse()
}

fn parse(input: &str) -> Vec<Contraption> {
    // Button A: X+26, Y+66
    // Button B: X+67, Y+21
    // Prize: X=12748, Y=12176
    let mut result = Vec::new();
    let mut a = None;
    let mut b = None;
    for line in input.lines().filter(|l| l.len() > 0) {
        let chars: Vec<_> = line.chars().collect();
        let i = chars.iter().position(|c| *c == 'X').unwrap();
        let j = chars.iter().position(|c| *c == ',').unwrap();
        let x = to_int(&chars[i + 2..j]).unwrap();
        let y = to_int(&chars[j + 4..]).unwrap();
        let v = Pt::new(x, y);
        match chars[7] {
            'A' => a = Some(v),
            'B' => b = Some(v),
            'X' => result.push(Contraption::new(a.take().unwrap(), b.take().unwrap(), v)),
            _ => panic!("Malformed line: '{line}'"),
        }
    }
    result
}

fn part_one(contraptions: &Vec<Contraption>) -> i64 {
    contraptions
        .iter()
        .map(|c| c.play())
        .filter(Option::is_some)
        .map(Option::unwrap)
        .sum()
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    const EXAMPLE_1: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    lazy_static! {
        static ref MODEL_1: Vec<Contraption> = Vec::from([
            Contraption::new(Pt::new(94, 34), Pt::new(22, 67), Pt::new(8400, 5400)),
            Contraption::new(Pt::new(26, 66), Pt::new(67, 21), Pt::new(12748, 12176)),
            Contraption::new(Pt::new(17, 86), Pt::new(84, 37), Pt::new(7870, 6450)),
            Contraption::new(Pt::new(69, 23), Pt::new(27, 71), Pt::new(18641, 10279)),
        ]);
    }

    #[test]
    fn parse_1() {
        assert_eq!(*MODEL_1, parse(EXAMPLE_1))
    }

    #[test]
    fn example_1() {
        assert_eq!(r"480", part_one(&*MODEL_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 13, do_solve).unwrap();
    }
}
