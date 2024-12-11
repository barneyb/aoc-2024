use crate::Part;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_x = grid[0].len() - 1;
    let max_y = grid.len() - 1;
    let mut sum = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '0' {
                let mut curr = HashSet::from([(x, y)]);
                for tgt in '1'..='9' {
                    let mut next = HashSet::new();
                    for (x, y) in curr {
                        if x > 0 && grid[y][x - 1] == tgt {
                            next.insert((x - 1, y));
                        }
                        if x < max_x && grid[y][x + 1] == tgt {
                            next.insert((x + 1, y));
                        }
                        if y > 0 && grid[y - 1][x] == tgt {
                            next.insert((x, y - 1));
                        }
                        if y < max_y && grid[y + 1][x] == tgt {
                            next.insert((x, y + 1));
                        }
                    }
                    curr = next;
                }
                sum += curr.len()
            }
        }
    }
    sum
}

fn part_two(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_x = grid[0].len() - 1;
    let max_y = grid.len() - 1;
    let mut sum = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '0' {
                let mut curr = HashMap::from([((x, y), 1)]);
                for tgt in '1'..='9' {
                    let mut next = HashMap::new();
                    for ((x, y), n) in curr {
                        if x > 0 && grid[y][x - 1] == tgt {
                            *next.entry((x - 1, y)).or_default() += n;
                        }
                        if x < max_x && grid[y][x + 1] == tgt {
                            *next.entry((x + 1, y)).or_default() += n;
                        }
                        if y > 0 && grid[y - 1][x] == tgt {
                            *next.entry((x, y - 1)).or_default() += n;
                        }
                        if y < max_y && grid[y + 1][x] == tgt {
                            *next.entry((x, y + 1)).or_default() += n;
                        }
                    }
                    curr = next;
                }
                sum += curr.values().sum::<usize>()
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"0123
1234
8765
9876"#;

    const EXAMPLE_2: &str = r#"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"#;

    const EXAMPLE_3: &str = r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#;

    const EXAMPLE_4: &str = r#"10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01"#;

    const EXAMPLE_5: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    const EXAMPLE_6: &str = r#".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9...."#;

    const EXAMPLE_7: &str = r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#;

    const EXAMPLE_8: &str = r#"012345
123456
234567
345678
4.6789
56789."#;

    #[test]
    fn example_1() {
        assert_eq!(r"1", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"2", part_one(EXAMPLE_2).to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(r"4", part_one(EXAMPLE_3).to_string());
    }

    #[test]
    fn example_4() {
        assert_eq!(r"3", part_one(EXAMPLE_4).to_string());
    }

    #[test]
    fn example_5() {
        assert_eq!(r"36", part_one(EXAMPLE_5).to_string());
        assert_eq!(r"81", part_two(EXAMPLE_5).to_string());
    }

    #[test]
    fn example_6() {
        assert_eq!(r"3", part_two(EXAMPLE_6).to_string());
    }

    #[test]
    fn example_7() {
        assert_eq!(r"13", part_two(EXAMPLE_7).to_string());
    }

    #[test]
    fn example_8() {
        assert_eq!(r"227", part_two(EXAMPLE_8).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 10, do_solve).unwrap();
    }
}
