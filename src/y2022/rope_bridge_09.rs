use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::Other(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    99999
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    const EXAMPLE_2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    #[test]
    fn example_1() {
        assert_eq!(r"13", part_one(EXAMPLE_1).to_string());
        // assert_eq!(r"1", part_two(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        // assert_eq!(r"36", part_two(EXAMPLE_2).to_string());
    }

    // #[test]
    // fn test_real_input() {
    //     crate::with_input(2022, 9, do_solve).unwrap();
    // }
}
