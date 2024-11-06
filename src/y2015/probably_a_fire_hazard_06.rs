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

    const EXAMPLE_1: &str = r#"turn on 0,0 through 999,999"#;

    const EXAMPLE_2: &str = r#"toggle 0,0 through 999,0"#;

    const EXAMPLE_3: &str = r#"turn off 499,499 through 500,500"#;

    const EXAMPLE_4: &str = r#"turn on 0,0 through 0,0"#;

    const EXAMPLE_5: &str = r#"toggle 0,0 through 999,999"#;

    #[test]
    fn example_1() {
        assert_eq!(r"1000000", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"1000", part_one(EXAMPLE_2).to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(r"0", part_one(EXAMPLE_3).to_string());
    }

    #[test]
    fn example_4() {
        assert_eq!(r"1", part_two(EXAMPLE_4).to_string());
    }

    #[test]
    fn example_5() {
        assert_eq!(r"2000000", part_two(EXAMPLE_5).to_string());
    }

    // #[test]
    // fn test_real_input() {
    //     crate::with_input(2015, 6, do_solve).unwrap();
    // }
}
