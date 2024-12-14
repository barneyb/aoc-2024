use crate::geom2d::Dir;
use crate::Part;
use std::collections::{HashSet, VecDeque};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

type Pt = (usize, usize);

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

fn find_regions(grid: &Grid, visited: &mut HashSet<Pt>) -> Vec<HashSet<Pt>> {
    let mut regions = Vec::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, &plant) in line.iter().enumerate() {
            let curr = (x, y);
            if visited.contains(&curr) {
                continue;
            }
            // in a new component!
            let mut region = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(curr);
            while let Some(curr) = queue.pop_front() {
                if !visited.insert(curr) {
                    continue;
                }
                region.insert(curr);
                let (x, y) = curr;
                let ns: Vec<_> = neighbors(&grid, x, y)
                    .into_iter()
                    .filter(|&(x, y)| grid[y][x] == plant)
                    .collect();
                queue.extend(ns);
            }
            regions.push(region)
        }
    }
    regions
}

fn part_one(input: &str) -> usize {
    let grid = parse(input);
    let mut visited = HashSet::new();
    let regions = find_regions(&grid, &mut visited);
    let mut total_price = 0;
    for r in regions {
        let area = r.len();
        let mut perim = 0;
        for &(x, y) in r.iter() {
            perim += 4 - neighbors(&grid, x, y)
                .iter()
                .filter(|n| r.contains(n))
                .count();
        }
        total_price += area * perim;
    }
    total_price
}

fn count_sides(c: &HashSet<Pt>, width: usize) -> usize {
    use Dir::*;

    let can_step = |(x, y), h| match h {
        North => y > 0 && c.contains(&(x, y - 1)),
        East => x < width - 1 && c.contains(&(x + 1, y)),
        South => y < width - 1 && c.contains(&(x, y + 1)),
        West => x > 0 && c.contains(&(x - 1, y)),
    };
    let take_step = |(x, y), h| match h {
        North => (x, y - 1),
        East => (x + 1, y),
        South => (x, y + 1),
        West => (x - 1, y),
    };
    let mut corners = 0;
    let starts: VecDeque<_> = c.iter().filter(|&&n| !can_step(n, North)).collect();
    let mut visited = HashSet::new();
    for &start in starts {
        let mut curr = start;
        let mut h = East;
        loop {
            if !visited.insert((curr, h)) {
                break;
            }
            if can_step(curr, h) {
                let a = take_step(curr, h);
                if can_step(a, h.turn_left()) {
                    corners += 1;
                    h = h.turn_left();
                    curr = take_step(a, h);
                } else {
                    curr = a;
                }
            } else {
                corners += 1;
                h = h.turn_right();
            }
            if curr == start && h == East {
                break;
            }
        }
    }
    corners
}

fn part_two(input: &str) -> usize {
    let grid = parse(input);
    let mut visited = HashSet::new();
    let regions = find_regions(&grid, &mut visited);
    let mut total_price = 0;
    for r in regions {
        let area = r.len();
        let sides = count_sides(&r, grid[0].len());
        total_price += area * sides;
    }
    total_price
}

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

    const EXAMPLE_4: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;

    const EXAMPLE_5: &str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

    #[test]
    fn example_1() {
        assert_eq!(r"140", part_one(EXAMPLE_1).to_string());
        assert_eq!(r"80", part_two(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"772", part_one(EXAMPLE_2).to_string());
        assert_eq!(r"436", part_two(EXAMPLE_2).to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(r"1930", part_one(EXAMPLE_3).to_string());
        assert_eq!(r"1206", part_two(EXAMPLE_3).to_string());
    }

    #[test]
    fn example_4() {
        assert_eq!(r"236", part_two(EXAMPLE_4).to_string());
    }

    #[test]
    fn example_5() {
        assert_eq!(r"368", part_two(EXAMPLE_5).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 12, do_solve).unwrap();
    }
}
