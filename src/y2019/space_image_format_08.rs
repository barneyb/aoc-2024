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

    const EXAMPLE_1: &str = r#"123456789012"#;

    const EXAMPLE_2: &str = r#"0222112222120000"#;

    #[test]
    fn example_1() {
        /*
         im=3x2
         */
        assert_eq!(r"1", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        /*
         im=2x2
         */
        assert_eq!(r"⟋", part_two(EXAMPLE_2).to_string());
    }

    // #[test]
    // fn test_real_input() {
    //     crate::with_input(2019, 8, do_solve).unwrap();
    // }
}
