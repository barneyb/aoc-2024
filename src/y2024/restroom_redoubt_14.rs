use crate::Part;
use regex::Regex;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    // 101 tiles wide and 103 tiles tall
    part_one_parameterized(input, 101, 103, 100)
}

fn part_one_parameterized(input: &str, width: i64, height: i64, ticks: usize) -> usize {
    let re = Regex::new("p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    let ticks = ticks as i64;
    let mid_x = width / 2;
    let mid_y = height / 2;
    let mut quads = [0; 4];
    input
        .lines()
        .map(|l| re.captures(l).unwrap())
        .map(|c| {
            let ns: Vec<i64> = c
                .iter()
                .map(|m| m.unwrap().as_str())
                .skip(1)
                .map(|n| n.parse().unwrap())
                .collect();
            ((ns[0], ns[1]), (ns[2], ns[3]))
        })
        .for_each(|((x, y), (dx, dy))| {
            let mut x = (x + dx * ticks) % width;
            let mut y = (y + dy * ticks) % height;
            if x < 0 {
                x += width
            }
            if y < 0 {
                y += height
            }
            if x < mid_x {
                if y < mid_y {
                    quads[0] += 1;
                } else if y > mid_y {
                    quads[1] += 1;
                }
            } else if x > mid_x {
                if y < mid_y {
                    quads[2] += 1;
                } else if y > mid_y {
                    quads[3] += 1;
                }
            }
        });
    quads.iter().product()
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn example_1() {
        // 11 tiles wide and 7 tiles tall
        assert_eq!(
            r"12",
            part_one_parameterized(EXAMPLE_1, 11, 7, 100).to_string()
        );
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 14, do_solve).unwrap();
    }
}
