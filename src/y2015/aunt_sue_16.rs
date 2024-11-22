use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::Other(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    input.len()
}

// fn part_two(input: &str) -> usize {
//     input.len()
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(3, part_one("AoC"));
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(12, part_two("adventofcode"));
    // }

    // #[test]
    // fn test_real_input() {
    //     crate::with_input(2015, 16, do_solve).unwrap();
    // }
}
