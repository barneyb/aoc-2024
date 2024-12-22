use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

#[rustfmt::skip]
fn prng(secret: usize) -> usize {
    let secret: usize = secret ^ (secret <<  6) & 0xffffff;
    let secret: usize = secret ^ (secret >>  5) & 0xffffff;
                        secret ^ (secret << 11) & 0xffffff
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .map(|s| {
            let mut n = s;
            for _ in 0..2000 {
                n = prng(n)
            }
            n
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    99999
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"1
10
100
2024"#;

    #[test]
    fn test_prng() {
        assert_eq!(15887950, prng(123));
        assert_eq!(16495136, prng(15887950));
        assert_eq!(527345, prng(16495136));
        assert_eq!(704524, prng(527345));
        assert_eq!(1553684, prng(704524));
        assert_eq!(12683156, prng(1553684));
        assert_eq!(11100544, prng(12683156));
        assert_eq!(12249484, prng(11100544));
        assert_eq!(7753432, prng(12249484));
        assert_eq!(5908254, prng(7753432));
    }

    #[test]
    fn example_1() {
        assert_eq!(37327623, part_one(EXAMPLE_1));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 22, do_solve).unwrap();
    }
}
