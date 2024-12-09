use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

const BLANK: isize = -1;

fn part_one(input: &str) -> usize {
    let digits: Vec<_> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect();
    let sum: isize = digits.iter().sum();
    let mut disk: Vec<isize> = Vec::with_capacity(sum as usize);
    let mut file_id = 0;
    for (i, d) in digits.iter().enumerate() {
        let f = if i % 2 == 0 {
            let v = file_id;
            file_id += 1;
            v
        } else {
            BLANK
        };
        disk.extend(&vec![f; *d as usize])
    }
    let mut i = 0;
    while let Some(f) = disk.get(i) {
        if *f == BLANK {
            disk[i] = disk.pop().unwrap();
        }
        i += 1;
        while let Some(v) = disk.get(disk.len() - 1) {
            if *v == BLANK {
                disk.pop();
            } else {
                break;
            }
        }
    }
    // for &i in disk.iter() {
    //     print!(
    //         "{}",
    //         if i == BLANK {
    //             ".".to_string()
    //         } else {
    //             i.to_string()
    //         }
    //     )
    // }
    // println!();
    disk.iter().enumerate().map(|(i, f)| i * *f as usize).sum()
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_0: &str = r#"12345"#;
    const EXAMPLE_1: &str = r#"2333133121414131402"#;

    #[test]
    fn example_0() {
        assert_eq!(r"60", part_one(EXAMPLE_0).to_string());
    }

    #[test]
    fn example_1() {
        assert_eq!(r"1928", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 9, do_solve).unwrap();
    }
}
