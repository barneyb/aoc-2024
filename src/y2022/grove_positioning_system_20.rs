use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let numbers = parse(input);
    tx.send(Part::A(part_one(&numbers).to_string())).unwrap();
    tx.send(Part::B(part_two(&numbers).to_string())).unwrap();
}

fn parse(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<_>>()
}

fn part_one(numbers: &Vec<isize>) -> isize {
    solve(numbers, 1, 1)
}

fn part_two(numbers: &Vec<isize>) -> isize {
    solve(numbers, 811589153, 10)
}

fn solve(numbers: &Vec<isize>, decryption_key: isize, rounds: usize) -> isize {
    let mut pairs = numbers
        .iter()
        .enumerate()
        .map(|(order, n)| (order, n * decryption_key))
        .collect::<Vec<_>>();
    // for arithmetic use after the item being moved is removed
    let len_less_one = pairs.len() as isize - 1;
    for _ in 0..rounds {
        for idx in 0..pairs.len() {
            let idx_src = pairs.iter().position(|&(order, _)| order == idx).unwrap();
            let it = pairs.remove(idx_src);
            let mut idx_tgt = (idx_src as isize + it.1) % len_less_one;
            if idx_tgt < 0 {
                idx_tgt += len_less_one
            }
            pairs.insert(idx_tgt as usize, it);
        }
    }
    let idx_zero = pairs.iter().position(|(_, n)| *n == 0).unwrap();
    vec![1000, 2000, 3000]
        .into_iter()
        .map(|it| pairs[(idx_zero + it) % pairs.len()].1)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    const EXAMPLE_1: &str = r#"1
2
-3
3
-2
0
4"#;

    lazy_static! {
        static ref MODEL_1: Vec<isize> = vec![1, 2, -3, 3, -2, 0, 4];
    }

    #[test]
    fn test_parse() {
        assert_eq!(*MODEL_1, parse(EXAMPLE_1));
    }

    #[test]
    fn example_1() {
        assert_eq!(r"3", part_one(&*MODEL_1).to_string());
        assert_eq!(r"1623178306", part_two(&*MODEL_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2022, 20, do_solve).unwrap();
    }
}
