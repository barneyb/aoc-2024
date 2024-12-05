use crate::Part;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let model = parse(input);
    tx.send(Part::Parse()).unwrap();
    tx.send(Part::A(part_one(&model).to_string())).unwrap();
    tx.send(Part::B(part_two_sort(&model).to_string())).unwrap();
    tx.send(Part::B(part_two_heap(&model).to_string())).unwrap();
    tx.send(Part::B(part_two_ifs_and_vars(&model).to_string()))
        .unwrap();
}

type Model = Vec<usize>;

fn parse(input: &str) -> Model {
    let mut result = Vec::new();
    let mut sum = 0;
    for l in input.lines() {
        if l == "" {
            result.push(sum);
            sum = 0;
        } else {
            sum += l.parse::<usize>().unwrap();
        }
    }
    if sum > 0 {
        result.push(sum);
    }
    result
}

fn part_one(model: &Model) -> usize {
    *model.iter().max().unwrap()
}

fn part_two_sort(model: &Model) -> usize {
    let mut model = model.clone();
    model.sort();
    model[model.len() - 3..].iter().sum()
}

fn part_two_heap(model: &Model) -> usize {
    let mut heap = BinaryHeap::with_capacity(4);
    for c in model {
        heap.push(Reverse(c));
        if heap.len() > 3 {
            heap.pop();
        }
    }
    heap.iter().map(|r| r.0).sum()
}

fn part_two_ifs_and_vars(model: &Model) -> usize {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for &n in model {
        if n > b {
            if n > a {
                (b, c) = (a, b);
                a = n;
            } else {
                c = b;
                b = n;
            }
        } else if n > c {
            c = n;
        }
    }
    a + b + c
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn example_1() {
        let model = parse(EXAMPLE_1);
        assert_eq!(r"24000", part_one(&model).to_string());
        assert_eq!(r"45000", part_two_sort(&model).to_string());
        assert_eq!(r"45000", part_two_heap(&model).to_string());
        assert_eq!(r"45000", part_two_ifs_and_vars(&model).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2022, 1, do_solve).unwrap();
    }
}
