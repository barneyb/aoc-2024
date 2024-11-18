use crate::timing::Timing;
use crate::Part;
use petgraph::prelude::*;
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let map = Timing::ad_hoc("parse", || parse(input));
    tx.send(Part::A(part_one(&map).to_string())).unwrap();
    let map = Timing::ad_hoc("parse2", || parse_two(input));
    tx.send(Part::B(part_two(&map).to_string())).unwrap();
}

/// x, y
type Point = (usize, usize);

struct Map {
    graph: Graph<Point, (), Undirected, u32>,
    seat_lookup: HashMap<Point, NodeIndex>,
    width: usize,
    height: usize,
}

impl Map {
    #[allow(dead_code)]
    fn draw(&self, occupied: &HashSet<NodeIndex>) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "{}",
                    if let Some(idx) = self.seat_lookup.get(&(x, y)) {
                        if occupied.contains(&idx) {
                            '#'
                        } else {
                            'L'
                        }
                    } else {
                        '.'
                    }
                )
            }
            println!()
        }
    }
}

fn parse(input: &str) -> Map {
    let mut graph = Graph::new_undirected();
    let mut seat_lookup = HashMap::new();
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != 'L' {
                continue;
            }
            let p = (x, y);
            let idx = graph.add_node(p);
            seat_lookup.insert(p, idx);
            if x > 0 {
                if let Some(i) = seat_lookup.get(&(x - 1, y)) {
                    graph.add_edge(*i, idx, ());
                }
            }
            if y > 0 {
                for x in (if x > 0 { x - 1 } else { x })..=(x + 1) {
                    if let Some(i) = seat_lookup.get(&(x, y - 1)) {
                        graph.add_edge(*i, idx, ());
                    }
                }
            }
        }
    }
    Map {
        graph,
        seat_lookup,
        width: grid[0].len(),
        height: grid.len(),
    }
}

fn part_one(map: &Map) -> usize {
    // println!("{:?}", petgraph::dot::Dot::with_config(&map.graph, &[petgraph::dot::Config::EdgeNoLabel]));
    let mut occupied: HashSet<NodeIndex> = HashSet::new();
    loop {
        let mut next = HashSet::new();
        for idx in map.graph.node_indices() {
            let count = map
                .graph
                .neighbors(idx)
                .filter(|i| occupied.contains(i))
                .count();
            if occupied.contains(&idx) {
                if count < 4 {
                    next.insert(idx);
                }
            } else if count == 0 {
                next.insert(idx);
            }
        }
        if next == occupied {
            return next.len();
        }
        occupied = next;
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pt(i32, i32);

struct Map2 {
    adjacent: HashMap<Pt, Vec<Pt>>,
}

impl Add for Pt {
    type Output = Pt;

    fn add(self, rhs: Self) -> Self::Output {
        let Pt(x, y) = self;
        let Pt(dx, dy) = rhs;
        Pt(x + dx, y + dy)
    }
}

fn parse_two(input: &str) -> Map2 {
    let mut adjacent: HashMap<Pt, Vec<Pt>> = HashMap::new();
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let width = grid[0].len() as i32;
    let deltas = [Pt(-1, 0), Pt(-1, -1), Pt(0, -1), Pt(1, -1)];
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != 'L' {
                continue;
            }
            let u: Pt = Pt(x as i32, y as i32);
            let mut adj = Vec::new();
            for d in deltas {
                let mut v = u + d;
                while v.0 >= 0 && v.0 < width && v.1 >= 0 {
                    if let Some(a) = adjacent.get_mut(&v) {
                        a.push(u);
                        adj.push(v);
                        break;
                    }
                    v = v + d;
                }
            }
            adjacent.insert(u, adj);
        }
    }
    Map2 { adjacent }
}

fn part_two(map: &Map2) -> usize {
    let mut occupied: HashSet<Pt> = HashSet::new();
    loop {
        let mut next = HashSet::new();
        for (p, adj) in map.adjacent.iter() {
            let count = adj.into_iter().filter(|i| occupied.contains(i)).count();
            if occupied.contains(p) {
                if count < 5 {
                    next.insert(*p);
                }
            } else if count == 0 {
                next.insert(*p);
            }
        }
        if next == occupied {
            return next.len();
        }
        occupied = next;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;

    #[test]
    fn example_1() {
        assert_eq!(r"37", part_one(&parse(EXAMPLE_1)).to_string());
        assert_eq!(r"26", part_two(&parse_two(EXAMPLE_1)).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2020, 11, do_solve).unwrap();
    }
}
