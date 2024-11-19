use crate::Part;
use std::sync::mpsc::Sender;

pub fn part_one(input: &str) -> i32 {
    input.chars().fold(0, |f, c| match c {
        '(' => f + 1,
        ')' => f - 1,
        _ => f,
    })
}

pub fn part_two(input: &str) -> usize {
    let mut f = 0;
    for (i, c) in input.chars().enumerate() {
        f = match c {
            '(' => f + 1,
            ')' => f - 1,
            _ => f,
        };
        if f == -1 {
            return i + 1;
        }
    }
    panic!("never entered basement?!")
}

pub fn both_parts(input: &str, tx: Sender<Part>) -> (i32, usize) {
    let mut f = 0;
    let mut entered_basement = None;
    for (i, c) in input.chars().enumerate() {
        f = match c {
            '(' => f + 1,
            ')' => f - 1,
            _ => f,
        };
        if f == -1 && entered_basement.is_none() {
            let eb = i + 1;
            entered_basement = Some(eb);
            tx.send(Part::B(eb.to_string())).unwrap();
        }
    }
    tx.send(Part::A(f.to_string())).unwrap();
    (f, entered_basement.expect("Should have entered basement"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(0, part_one("(())"));
        assert_eq!(3, part_one("(()(()("));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(5, part_two("()())"));
    }

    #[test]
    fn test_real_input() {
        use crate::{with_input, Part};
        with_input(2015, 1, |input, tx| {
            tx.send(Part::A(part_one(input).to_string())).unwrap();
            tx.send(Part::B(part_two(input).to_string())).unwrap();
        })
        .unwrap();
    }

    #[test]
    fn test_real_input_as_one() {
        use crate::with_input;
        with_input(2015, 1, |input, tx| {
            both_parts(input, tx);
        })
        .unwrap();
    }
}
