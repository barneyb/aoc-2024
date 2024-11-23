use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    let mut size: usize = input.parse().unwrap();
    let mut list = Vec::with_capacity(size + 1);
    list.push(0); // there's no elf 0 - waste a slot
    for i in 1..size {
        list.push(i + 1)
    }
    list.push(1); // link the ring
    let mut curr = 1;
    while size > 1 {
        let to_kill = list[curr];
        let following = list[to_kill];
        list[curr] = following;
        curr = following;
        size -= 1;
    }
    curr
}

// fn part_two(input: &str) -> usize {
//     input.len()
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"5"#;

    #[test]
    fn example_1() {
        assert_eq!(r"3", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2016, 19, do_solve).unwrap();
    }
}
