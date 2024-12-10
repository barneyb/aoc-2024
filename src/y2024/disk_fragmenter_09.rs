use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

const BLANK: isize = -1;

fn create_disk(input: &str) -> Vec<isize> {
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
    disk
}

fn part_one(input: &str) -> usize {
    let mut disk = create_disk(input);
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
    checksum(&disk)
}

fn checksum(disk: &Vec<isize>) -> usize {
    disk.iter()
        .enumerate()
        .filter(|(_, id)| **id != BLANK)
        .map(|(i, id)| i * *id as usize)
        .sum()
}

fn part_two(input: &str) -> usize {
    let mut disk = create_disk(input);
    let mut files = Vec::new();
    // using a VecDeque and discarding empty gaps is appreciably slower.
    let mut gaps = Vec::new();
    let mut itr = disk.iter().enumerate();
    let (mut start, mut prev) = itr.next().unwrap();
    for (i, id) in itr {
        if prev == id {
            continue;
        }
        let pair = (start, i - start);
        if *prev == BLANK {
            gaps.push(pair);
        } else {
            files.push(pair);
        }
        start = i;
        prev = id;
    }
    files.push((start, disk.len() - start));
    // println!("disk  : {disk:?}");
    // println!("files : {files:?}");
    // println!("gaps  : {gaps:?}");
    for (start, len) in files.into_iter().rev() {
        for (gap_start, gap_len) in gaps.iter_mut() {
            if start <= *gap_start {
                break;
            }
            if len <= *gap_len {
                // println!("Move file {} from {start} to {gap_start}", disk[start]);
                for offset in 0..len {
                    disk.swap(start + offset, *gap_start + offset)
                }
                *gap_start += len;
                *gap_len -= len;
                break;
            }
        }
    }
    // println!("defrag: {disk:?}");
    checksum(&disk)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_0: &str = r#"12345"#;
    const EXAMPLE_1: &str = r#"2333133121414131402"#;

    #[test]
    fn example_0() {
        assert_eq!(r"60", part_one(EXAMPLE_0).to_string());
        assert_eq!(r"132", part_two(EXAMPLE_0).to_string());
    }

    #[test]
    fn example_1() {
        assert_eq!(r"1928", part_one(EXAMPLE_1).to_string());
        assert_eq!(r"2858", part_two(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 9, do_solve).unwrap();
    }
}
