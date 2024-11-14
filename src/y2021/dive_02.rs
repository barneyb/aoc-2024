use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::Other(part_one(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    input.len()
}

fn part_two(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

    #[test]
    fn example_1() {
        assert_eq!(r"150", part_one(EXAMPLE_1).to_string());
        assert_eq!(r"900", part_two(EXAMPLE_1).to_string());
    }

    // #[test]
    // fn test_real_input() {
    //     crate::with_input(2021, 2, do_solve).unwrap();
    // }
}
