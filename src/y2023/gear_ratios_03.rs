use crate::Part;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
}

// number, start x, end x, y
type Num = (usize, usize, usize, usize);
type Point = (usize, usize);

fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut nums: Vec<Num> = Vec::new();
    let mut symbols: HashMap<Point, char> = HashMap::new();
    let mut curr = None;
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                let d = c.to_digit(10).unwrap() as usize;
                if let Some((n, sx, _, y)) = curr {
                    curr = Some((n * 10 + d, sx, x, y))
                } else {
                    curr = Some((d, x, x, y))
                }
            } else {
                if let Some(num) = curr {
                    nums.push(num);
                    curr = None;
                }
                if is_symbol(*c) {
                    symbols.insert((x, y), *c);
                }
            }
        }
        if let Some(num) = curr {
            nums.push(num);
            curr = None;
        }
    }
    // println!("{nums:?}");
    // println!("{symbols:?}");
    let mut sum = 0;
    'outer: for num in nums {
        if neighbors(&num).any(|p| symbols.contains_key(&p)) {
            let (n, ..) = num;
            sum += n;
            continue 'outer;
        }
        // println!(
        //     "No symbol for {num:?} among {:?}",
        //     neighbors(&num).collect::<Vec<_>>()
        // )
    }
    sum
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

fn neighbors(num: &Num) -> Neighbors {
    let (_, x1, x2, y) = *num;
    let left_edge = x1 == 0;
    let x1 = if left_edge { x1 } else { x1 - 1 };
    Neighbors {
        x1,
        x2,
        y,
        curr: if left_edge {
            if y == 0 {
                // start on right
                Some((x2 + 1, y))
            } else {
                // start above first digit
                Some((x1, y - 1))
            }
        } else {
            // start on left
            Some((x1, y))
        },
    }
}

struct Neighbors {
    x1: usize,
    x2: usize,
    y: usize,
    curr: Option<Point>,
}

impl Iterator for Neighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr.take();
        if let Some(p) = &curr {
            let (x, y) = *p;
            self.curr = if y == self.y {
                // on the number's row
                Some(if x == self.x1 {
                    // left of number
                    if y > 0 {
                        (x, y - 1)
                    } else {
                        (self.x2 + 1, y)
                    }
                } else {
                    // right of number
                    (x, y + 1)
                })
            } else if y < self.y {
                // above the number
                Some(if x <= self.x2 { (x + 1, y) } else { (x, y + 1) })
            } else {
                // below the number
                if x > self.x1 {
                    Some((x - 1, y))
                } else {
                    None
                }
            };
        }
        curr
    }
}

fn part_two(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"467..114..
...*......
..35...633
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn example_1() {
        assert_eq!(r"4361", part_one(EXAMPLE_1).to_string());
        // assert_eq!(r"467835", part_two(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2023, 3, do_solve).unwrap();
    }
}
