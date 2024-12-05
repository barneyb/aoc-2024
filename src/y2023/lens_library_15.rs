use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn hash(input: &str) -> usize {
    let mut val = 0;
    for c in input.chars() {
        val += c as usize;
        val *= 17;
        val %= 256;
    }
    val
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn example_1() {
        assert_eq!(r"1320", part_one(EXAMPLE_1).to_string());
        // assert_eq!(r"145", part_two(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2023, 15, do_solve).unwrap();
    }
}
