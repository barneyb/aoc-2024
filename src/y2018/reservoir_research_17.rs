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

    const EXAMPLE_1: &str = r#"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"#;

    #[test]
    fn example_1() {
        assert_eq!(r"57", part_one(EXAMPLE_1).to_string());
        assert_eq!(r"29", part_two(EXAMPLE_1).to_string());
    }

    // #[test]
    // fn test_real_input() {
    //     crate::with_input(2018, 17, do_solve).unwrap();
    // }
}
