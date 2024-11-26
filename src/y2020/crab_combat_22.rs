use crate::Part;
use std::collections::VecDeque;
use std::sync::mpsc::Sender;

type Decks = [Vec<usize>; 2];

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let decks = parse(input);
    tx.send(Part::Parse(format!(
        "{}, {}",
        decks[0].len(),
        decks[1].len()
    )))
    .unwrap();
    tx.send(Part::A(part_one(&decks).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

fn parse(input: &str) -> Decks {
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut switch = false;
    for l in input.lines() {
        if l == "" {
            switch = true
        } else if let Some('P') = l.chars().next() {
        } else {
            let n = l.parse().unwrap();
            if switch {
                b.push(n)
            } else {
                a.push(n)
            }
        }
    }
    [a, b]
}

fn part_one(decks: &Decks) -> usize {
    let mut da: VecDeque<usize> = decks[0].iter().map(|&n| n).collect();
    let mut db: VecDeque<usize> = decks[1].iter().map(|&n| n).collect();
    loop {
        if let Some(a) = da.pop_front() {
            if let Some(b) = db.pop_front() {
                if a > b {
                    da.push_back(a);
                    da.push_back(b);
                    if db.len() == 0 {
                        return calc_score(da);
                    }
                } else {
                    db.push_back(b);
                    db.push_back(a);
                    if da.len() == 0 {
                        return calc_score(db);
                    }
                }
            } else {
                panic!("da fuk!?")
            }
        } else {
            panic!("da fuk!?")
        }
    }
}

fn calc_score(deck: VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, n)| (i + 1) * n)
        .sum()
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    const EXAMPLE_1: &str = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;

    lazy_static! {
        static ref DECKS_1: Decks = [vec![9, 2, 6, 3, 1], vec![5, 8, 4, 7, 10]];
    }

    #[test]
    fn parse_example_1() {
        assert_eq!(*DECKS_1, parse(EXAMPLE_1));
    }

    #[test]
    fn test_score_calculation() {
        assert_eq!(
            306,
            calc_score(VecDeque::from([3, 2, 10, 6, 8, 5, 9, 4, 7, 1]))
        );
    }

    #[test]
    fn example_1() {
        assert_eq!(r"306", part_one(&*DECKS_1).to_string());
        // assert_eq!(r"291", part_two(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2020, 22, do_solve).unwrap();
    }
}
