pub fn part_one(input: &str) -> usize {
    let mut sum: usize = 0;
    for l in input.lines() {
        let mut min = usize::MAX;
        let mut max = usize::MIN;
        for s in l.split_whitespace() {
            let n = s.parse().unwrap();
            if n < min {
                min = n
            }
            if n > max {
                max = n
            }
        }
        sum += max - min;
    }
    sum
}

pub fn part_two(input: &str) -> usize {
    let mut sum: usize = 0;
    for l in input.lines() {
        let ns = l
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();
        'line: for (i, l) in ns.iter().enumerate() {
            for r in &ns[i + 1..] {
                let (big, small) = if l < r { (r, l) } else { (l, r) };
                let quot = big / small;
                let rem = big % small;
                if rem == 0 {
                    sum += quot;
                    break 'line;
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"5 1 9 5
7 5 3
2 4 6 8"#;

    const EXAMPLE_2: &str = r#"5 9 2 8
9 4 7 3
3 8 6 5"#;

    #[test]
    fn example_1() {
        assert_eq!(r"18", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"9", part_two(EXAMPLE_2).to_string());
    }

    #[test]
    fn test_real_input() {
        use crate::{with_input, Part};
        with_input(2017, 2, |input, tx| {
            tx.send(Part::A(Box::new(part_one(input)))).unwrap();
            tx.send(Part::B(Box::new(part_two(input)))).unwrap();
        })
        .unwrap();
    }
}
