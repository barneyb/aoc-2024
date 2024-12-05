use crate::Part;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let model = parse(input);
    tx.send(Part::Parse()).unwrap();
    tx.send(Part::A(part_one(&model).to_string())).unwrap();
    // tx.send(Part::Other(part_two(&model).to_string())).unwrap();
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

fn part_one(model: &Model) -> usize {
    let mut seen = HashSet::new();
    let mut sum = 0;
    'outer: for update in &model.updates {
        seen.clear();
        for n in update {
            if let Some(non_prec) = model.non_precedence.get(n) {
                if !non_prec.is_disjoint(&seen) {
                    continue 'outer;
                }
            }
            seen.insert(*n);
        }
        sum += update[update.len() / 2];
    }
    sum
}

// fn part_two(input: &str) -> usize {
//     99999
// }

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
        println!("{model:?}");
        assert_eq!(r"143", part_one(&model).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 5, do_solve).unwrap();
    }
}
