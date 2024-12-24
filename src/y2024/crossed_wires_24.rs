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

    const EXAMPLE_1: &str = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;

    #[test]
    fn example_1() {
        assert_eq!(r"z", part_one(EXAMPLE_1).to_string());
    }

    // #[test]
    // fn test_real_input() {
    //     crate::with_input(2024, 24, do_solve).unwrap();
    // }
}
