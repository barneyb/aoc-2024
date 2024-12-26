use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for block in input.split("\n\n") {
        let mut grid: Vec<Vec<_>> = block.lines().map(|l| l.chars().collect()).collect();
        assert_eq!(7, grid.len());
        let is_key = grid[6].iter().all(|c| *c == '#');
        if is_key {
            grid.reverse();
        }
        if !grid[0].iter().all(|c| *c == '#') {
            panic!("huh?! {grid:?}")
        }
        let mut heights = [0; 5];
        for y in 1..7 {
            for (x, &c) in grid[y].iter().enumerate() {
                if c == '#' {
                    heights[x] += 1;
                }
            }
        }
        if is_key {
            keys.push(heights)
        } else {
            locks.push(heights)
        }
    }
    let mut count = 0;
    for l in locks {
        'nope: for k in keys.iter() {
            for i in 0..5 {
                if l[i] + k[i] >= 6 {
                    continue 'nope;
                }
            }
            count += 1;
        }
    }
    count
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

    #[test]
    fn example_1() {
        assert_eq!(r"3", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 25, do_solve).unwrap();
    }
}
