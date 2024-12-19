use crate::Part;
use regex::Regex;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    let mut lines = input.lines();
    let towels: Vec<_> = lines.next().unwrap().split(',').map(|s| s.trim()).collect();
    lines.next();
    let stacks: Vec<_> = lines.collect();
    let mut re = "^(".to_string();
    for (i, t) in towels.iter().enumerate() {
        if i > 0 {
            re.push('|')
        }
        re.push_str(t);
    }
    re.push_str(")+$");
    let re = Regex::new(&re).unwrap();
    stacks.iter().filter(|s| re.is_match(s)).count()
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    #[test]
    fn example_1() {
        assert_eq!(r"6", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 19, do_solve).unwrap();
    }
}
