use crate::Part;
use std::collections::{HashSet, VecDeque};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

type Grid = Vec<Vec<char>>;

fn parse(input: &str) -> Grid {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn neighbors(grid: &Grid, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut ns = Vec::with_capacity(4);
    if x > 0 {
        ns.push((x - 1, y))
    }
    if x < grid[y].len() - 1 {
        ns.push((x + 1, y))
    }
    if y > 0 {
        ns.push((x, y - 1))
    }
    if y < grid.len() - 1 {
        ns.push((x, y + 1))
    }
    ns
}

fn part_one(input: &str) -> usize {
    let grid = parse(input);
    let mut visited = HashSet::new();
    let mut total_price = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, &plant) in line.iter().enumerate() {
            let curr = (x, y);
            if visited.contains(&curr) {
                continue;
            }
            // in a new component!
            let mut area = 0;
            let mut perim = 0;
            let mut queue = VecDeque::new();
            queue.push_back(curr);
            while let Some((x, y)) = queue.pop_front() {
                if !visited.insert((x, y)) {
                    continue;
                }
                area += 1;
                let ns: Vec<_> = neighbors(&grid, x, y)
                    .into_iter()
                    .filter(|&(x, y)| grid[y][x] == plant)
                    .collect();
                perim += 4 - ns.len();
                queue.extend(ns);
            }
            total_price += area * perim;
        }
    }
    total_price
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

    const EXAMPLE_2: &str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

    const EXAMPLE_3: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    #[test]
    fn example_1() {
        assert_eq!(r"140", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"772", part_one(EXAMPLE_2).to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(r"1930", part_one(EXAMPLE_3).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 12, do_solve).unwrap();
    }
}
