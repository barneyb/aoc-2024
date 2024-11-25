use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::Other(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

fn part_one(_input: &str) -> usize {
    99999
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"1 + 2 * 3 + 4 * 5 + 6"#;

    const EXAMPLE_2: &str = r#"1 + (2 * 3) + (4 * (5 + 6))"#;

    const EXAMPLE_3: &str = r#"2 * 3 + (4 * 5)"#;

    const EXAMPLE_4: &str = r#"5 + (8 * 3 + 9 + 3 * 4 * 3)"#;

    const EXAMPLE_5: &str = r#"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"#;

    const EXAMPLE_6: &str = r#"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"#;

    #[test]
    fn example_1() {
        assert_eq!(r"71", part_one(EXAMPLE_1).to_string());
        // assert_eq!(r"231", part_two(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"51", part_one(EXAMPLE_2).to_string());
        // assert_eq!(r"51", part_two(EXAMPLE_2).to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(r"26", part_one(EXAMPLE_3).to_string());
        // assert_eq!(r"46", part_two(EXAMPLE_3).to_string());
    }

    #[test]
    fn example_4() {
        assert_eq!(r"437", part_one(EXAMPLE_4).to_string());
        // assert_eq!(r"1445", part_two(EXAMPLE_4).to_string());
    }

    #[test]
    fn example_5() {
        assert_eq!(r"12240", part_one(EXAMPLE_5).to_string());
        // assert_eq!(r"669060", part_two(EXAMPLE_5).to_string());
    }

    #[test]
    fn example_6() {
        assert_eq!(r"13632", part_one(EXAMPLE_6).to_string());
        // assert_eq!(r"23340", part_two(EXAMPLE_6).to_string());
    }

    // #[test]
    // fn test_real_input() {
    //     crate::with_input(2020, 18, do_solve).unwrap();
    // }
}
