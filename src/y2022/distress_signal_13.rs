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

    const EXAMPLE_1: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    #[test]
    fn example_1() {
        assert_eq!(r"13", part_one(EXAMPLE_1).to_string());
        // assert_eq!(r"140", part_two(EXAMPLE_1).to_string());
    }

    // #[test]
    // fn test_real_input() {
    //     crate::with_input(2022, 13, do_solve).unwrap();
    // }
}
