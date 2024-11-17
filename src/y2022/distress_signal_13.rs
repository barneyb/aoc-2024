use crate::Part;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::sync::mpsc::Sender;
use Val::*;

pub fn do_solve(vals: &str, tx: Sender<Part>) {
    let packets = parse(vals);
    tx.send(Part::A(part_one(&packets).to_string())).unwrap();
    tx.send(Part::B(part_two(&packets).to_string())).unwrap();
}

#[derive(Debug)]
enum Val {
    Int(u32),
    List(Vec<Val>),
}

impl From<&str> for Val {
    fn from(s: &str) -> Self {
        let mut stack: Vec<Vec<Val>> = Vec::new();
        let mut curr: Vec<Val> = Vec::new();
        let mut num = None;
        for (i, c) in s.chars().enumerate() {
            match c {
                '[' => {
                    stack.push(curr);
                    curr = Vec::new();
                }
                ']' => {
                    if let Some(n) = num.take() {
                        curr.push(Int(n))
                    }
                    let v = List(curr);
                    curr = stack.pop().unwrap();
                    curr.push(v);
                }
                ',' => {
                    if let Some(n) = num.take() {
                        curr.push(Int(n))
                    }
                }
                _ if c.is_ascii_digit() => {
                    let d = c.to_digit(10).unwrap();
                    num = Some(if let Some(n) = num { n * 10 + d } else { d })
                }
                _ => panic!("Found a '{c}' at {i}?!"),
            }
        }
        curr.pop().unwrap()
    }
}

impl_ord!(
    Val,
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Int(left) => match other {
                Int(right) => left.cmp(right),
                right => List(vec![Int(*left)]).cmp(right),
            },
            List(left) => match other {
                Int(right) => self.cmp(&List(vec![Int(*right)])),
                List(right) => left.cmp(right),
            },
        }
    }
);

impl Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Int(n) => write!(f, "{n}"),
            List(kids) => {
                let kids: Vec<_> = kids.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", kids.join(","))
            }
        }
    }
}

fn parse(input: &str) -> Vec<Val> {
    input
        .lines()
        .filter(|s| s.len() != 0)
        .map(Val::from)
        .collect()
}

fn part_one(packets: &[Val]) -> usize {
    let mut sum = 0;
    for (i, c) in packets.chunks(2).enumerate() {
        let a = &c[0];
        let b = &c[1];
        if a.cmp(b) == Ordering::Less {
            sum += i + 1; // one-indexed!
        }
    }
    sum
}

fn part_two(packets: &[Val]) -> usize {
    let a: Val = "[[2]]".into();
    let b: Val = "[[6]]".into();
    let mut all = Vec::with_capacity(packets.len() + 2);
    all.push(&a);
    all.push(&b);
    all.extend(packets);
    all.sort_unstable();
    let a = all.iter().enumerate().find(|(_, v)| ***v == a).unwrap().0;
    let b = all.iter().enumerate().find(|(_, v)| ***v == b).unwrap().0;
    (a + 1) * (b + 1) // one-indexed!
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    const EXAMPLE_1: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    lazy_static! {
        static ref PACKETS_1: Vec<Val> = vec![
            List(vec![Int(1), Int(1), Int(3), Int(1), Int(1)]),
            List(vec![Int(1), Int(1), Int(5), Int(1), Int(1)]),
            List(vec![List(vec![Int(1)]), List(vec![Int(2), Int(3), Int(4)])]),
            List(vec![List(vec![Int(1)]), Int(4)]),
            List(vec![Int(9)]),
            List(vec![List(vec![Int(8), Int(7), Int(6)])]),
            List(vec![List(vec![Int(4), Int(4)]), Int(4), Int(4)]),
            List(vec![List(vec![Int(4), Int(4)]), Int(4), Int(4), Int(4)]),
            List(vec![Int(7), Int(7), Int(7), Int(7)]),
            List(vec![Int(7), Int(7), Int(7)]),
            List(vec![]),
            List(vec![Int(3)]),
            List(vec![List(vec![List(vec![])])]),
            List(vec![List(vec![])]),
            List(vec![
                Int(1),
                List(vec![
                    Int(2),
                    List(vec![
                        Int(3),
                        List(vec![Int(4), List(vec![Int(5), Int(6), Int(7)])])
                    ])
                ]),
                Int(8),
                Int(9)
            ]),
            List(vec![
                Int(1),
                List(vec![
                    Int(2),
                    List(vec![
                        Int(3),
                        List(vec![Int(4), List(vec![Int(5), Int(6), Int(0)])])
                    ])
                ]),
                Int(8),
                Int(9)
            ]),
        ];
    }

    #[test]
    fn parse_1() {
        assert_eq!(*PACKETS_1, parse(EXAMPLE_1))
    }

    #[test]
    fn test_cmp_int() {
        assert_eq!(Ordering::Less, Int(1).cmp(&Int(2)));
        assert_eq!(Ordering::Equal, Int(1).cmp(&Int(1)));
        assert_eq!(Ordering::Greater, Int(2).cmp(&Int(1)));
    }

    #[test]
    fn test_cmp_list() {
        assert_eq!(Ordering::Less, List(vec![]).cmp(&List(vec![Int(2)])));
        assert_eq!(Ordering::Less, List(vec![Int(1)]).cmp(&List(vec![Int(2)])));
        assert_eq!(Ordering::Equal, List(vec![Int(2)]).cmp(&List(vec![Int(2)])));
        assert_eq!(
            Ordering::Greater,
            List(vec![Int(2)]).cmp(&List(vec![Int(1)]))
        );
        assert_eq!(Ordering::Greater, List(vec![Int(2)]).cmp(&List(vec![])));
    }

    #[test]
    fn test_cmp_mixed() {
        assert_eq!(Ordering::Less, Int(1).cmp(&List(vec![Int(2)])));
        assert_eq!(Ordering::Less, List(vec![Int(1)]).cmp(&Int(2)));
        assert_eq!(Ordering::Equal, Int(2).cmp(&List(vec![Int(2)])));
        assert_eq!(Ordering::Equal, List(vec![Int(2)]).cmp(&Int(2)));
        assert_eq!(Ordering::Greater, Int(2).cmp(&List(vec![Int(1)])));
        assert_eq!(Ordering::Greater, List(vec![Int(2)]).cmp(&Int(1)));
    }

    #[test]
    fn test_partial_cmp() {
        assert_eq!(Some(Ordering::Less), List(vec![]).partial_cmp(&Int(1)));
        assert_eq!(Some(Ordering::Greater), Int(1).partial_cmp(&List(vec![])));
        assert_eq!(
            Some(Ordering::Less),
            List(vec![List(vec![])]).partial_cmp(&List(vec![Int(1)]))
        );
        assert_eq!(
            Some(Ordering::Greater),
            List(vec![Int(1)]).partial_cmp(&List(vec![List(vec![])]))
        );
    }

    #[test]
    fn test_cmp_examples() {
        let ps = &*PACKETS_1;
        assert_eq!(Ordering::Less, ps[0].cmp(&ps[1]));
        assert_eq!(Ordering::Less, ps[2].cmp(&ps[3]));
        assert_eq!(Ordering::Greater, ps[4].cmp(&ps[5]));
        assert_eq!(Ordering::Less, ps[6].cmp(&ps[7]));
        assert_eq!(Ordering::Greater, ps[8].cmp(&ps[9]));
        assert_eq!(Ordering::Less, ps[10].cmp(&ps[11]));
        assert_eq!(Ordering::Greater, ps[12].cmp(&ps[13]));
        assert_eq!(Ordering::Greater, ps[14].cmp(&ps[15]));
    }

    #[test]
    fn example_1() {
        assert_eq!(1, part_one(&(*PACKETS_1)[0..2]));
        assert_eq!(1, part_one(&(*PACKETS_1)[2..4]));
        assert_eq!(r"13", part_one(&*PACKETS_1).to_string());
        assert_eq!(r"140", part_two(&*PACKETS_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2022, 13, do_solve).unwrap();
    }
}
