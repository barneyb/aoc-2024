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
            tx.send(Part::A(Box::new(part_one(input)))).unwrap();
            tx.send(Part::B(Box::new(part_two(input)))).unwrap();
        })
        .unwrap();
    }
}
