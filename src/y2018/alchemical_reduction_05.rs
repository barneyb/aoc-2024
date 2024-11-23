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

    const EXAMPLE_1: &str = r#"aA"#;

    const EXAMPLE_2: &str = r#"abBA"#;

    const EXAMPLE_3: &str = r#"abAB"#;

    const EXAMPLE_4: &str = r#"aabAAB"#;

    const EXAMPLE_5: &str = r#"dabAcCaCBAcCcaDA"#;

    #[test]
    fn example_1() {
        assert_eq!(r"0", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"0", part_one(EXAMPLE_2).to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(r"4", part_one(EXAMPLE_3).to_string());
    }

    #[test]
    fn example_4() {
        assert_eq!(r"6", part_one(EXAMPLE_4).to_string());
    }

    #[test]
    fn example_5() {
        assert_eq!(r"10", part_one(EXAMPLE_5).to_string());
        // assert_eq!(r"4", part_two(EXAMPLE_5).to_string());
    }

    // #[test]
    // fn test_real_input() {
    //     crate::with_input(2018, 5, do_solve).unwrap();
    // }
}
