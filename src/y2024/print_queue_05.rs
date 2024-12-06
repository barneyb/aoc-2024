use crate::Part;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let model = parse(input);
    tx.send(Part::Parse()).unwrap();
    tx.send(Part::A(part_one(&model).to_string())).unwrap();
    tx.send(Part::B(part_two(&model).to_string())).unwrap();
}

#[derive(Debug)]
struct Model {
    non_precedence: HashMap<usize, HashSet<usize>>,
    updates: Vec<Vec<usize>>,
}

fn parse(input: &str) -> Model {
    let mut non_precedence: HashMap<_, HashSet<_>> = HashMap::new();
    let mut updates = Vec::new();
    let mut first_part = true;
    for line in input.lines() {
        if line == "" {
            first_part = false;
            continue;
        }
        if first_part {
            let i = line.chars().position(|c| c == '|').unwrap();
            let a: usize = line[0..i].parse().unwrap();
            let b: usize = line[i + 1..line.len()].parse().unwrap();
            non_precedence.entry(a).or_default().insert(b);
        } else {
            updates.push(line.split(',').map(|s| s.parse().unwrap()).collect())
        }
    }
    Model {
        non_precedence,
        updates,
    }
}

impl Model {
    fn first_misordered_index(&self, update: &[usize]) -> Option<usize> {
        let mut seen = HashSet::new();
        for (i, n) in update.iter().enumerate() {
            if let Some(non_prec) = self.non_precedence.get(n) {
                if !non_prec.is_disjoint(&seen) {
                    return Some(i);
                }
            }
            seen.insert(*n);
        }
        None
    }

    fn is_ordered(&self, update: &[usize]) -> bool {
        self.first_misordered_index(update).is_none()
    }

    fn reorder(&self, update: &Vec<usize>) -> Vec<usize> {
        let mut update = update.clone();
        while let Some(i) = self.first_misordered_index(&update) {
            update.swap(i, i - 1)
        }
        update
    }
}

fn part_one(model: &Model) -> usize {
    let mut sum = 0;
    for update in &model.updates {
        if model.is_ordered(update) {
            sum += update[update.len() / 2];
        }
    }
    sum
}

fn part_two(model: &Model) -> usize {
    let mut sum = 0;
    for update in &model.updates {
        if !model.is_ordered(update) {
            let update = model.reorder(update);
            sum += update[update.len() / 2];
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn example_1() {
        let model = parse(EXAMPLE_1);
        assert_eq!(r"143", part_one(&model).to_string());
        assert_eq!(r"123", part_two(&model).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 5, do_solve).unwrap();
    }
}
