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

    const EXAMPLE_1: &str = r#"1122"#;

    const EXAMPLE_2: &str = r#"1111"#;

    const EXAMPLE_3: &str = r#"1234"#;

    const EXAMPLE_4: &str = r#"91212129"#;

    const EXAMPLE_5: &str = r#"1212"#;

    const EXAMPLE_6: &str = r#"1221"#;

    const EXAMPLE_7: &str = r#"123425"#;

    const EXAMPLE_8: &str = r#"123123"#;

    const EXAMPLE_9: &str = r#"12131415"#;

    #[test]
    fn example_1() {
        assert_eq!(r"3", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"4", part_one(EXAMPLE_2).to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(r"0", part_one(EXAMPLE_3).to_string());
    }

    #[test]
    fn example_4() {
        assert_eq!(r"9", part_one(EXAMPLE_4).to_string());
    }

    #[test]
    fn example_5() {
        assert_eq!(r"6", part_two(EXAMPLE_5).to_string());
    }

    #[test]
    fn example_6() {
        assert_eq!(r"0", part_two(EXAMPLE_6).to_string());
    }

    #[test]
    fn example_7() {
        assert_eq!(r"4", part_two(EXAMPLE_7).to_string());
    }

    #[test]
    fn example_8() {
        assert_eq!(r"12", part_two(EXAMPLE_8).to_string());
    }

    #[test]
    fn example_9() {
        assert_eq!(r"4", part_two(EXAMPLE_9).to_string());
    }

    // #[test]
    // fn test_real_input() {
    //     crate::with_input(2017, 1, do_solve).unwrap();
    // }
}
