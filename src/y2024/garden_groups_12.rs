use crate::geom2d::Dir;
use crate::Part;
use petgraph::graph::NodeIndex;
use petgraph::prelude::UnGraph;
use petgraph::{Graph, Undirected};
use std::collections::{HashSet, VecDeque};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let map = parse(input);
    tx.send(Part::Parse()).unwrap();
    let components = find_components(&map);
    tx.send(Part::Parse()).unwrap();
    tx.send(Part::A(part_one(&map, &components).to_string()))
        .unwrap();
    tx.send(Part::B(part_two(&map, &components).to_string()))
        .unwrap();
}

struct Map {
    graph: Graph<(), (), Undirected, usize>,
    width: usize,
}

fn parse(input: &str) -> Map {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let width = grid[0].len();
    assert_eq!(grid.len(), width);
    let mut edges = Vec::with_capacity(width * width);
    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            let me = y * width + x;
            if x > 0 && line[x - 1] == c {
                edges.push((me, me - 1))
            }
            if y > 0 && grid[y - 1][x] == c {
                edges.push((me, me - width))
            }
        }
    }
    Map {
        graph: UnGraph::from_edges(edges),
        width,
    }
}

type Nx = NodeIndex<usize>;

type Component = Vec<Nx>;

fn find_components(map: &Map) -> Vec<Component> {
    let mut components = Vec::new();
    let mut visited = HashSet::new();
    for n in map.graph.node_indices() {
        if !visited.contains(&n) {
            components.push(walk_component(n, &map.graph, &mut visited))
        }
    }
    components
}

fn walk_component(
    start: Nx,
    graph: &Graph<(), (), Undirected, usize>,
    visited: &mut HashSet<Nx>,
) -> Vec<Nx> {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut extent = Vec::new();
    while let Some(curr) = queue.pop_front() {
        if visited.insert(curr) {
            extent.push(curr);
            queue.extend(graph.neighbors(curr))
        }
    }
    extent
}

fn part_one(map: &Map, components: &Vec<Component>) -> usize {
    components
        .iter()
        .map(|c| {
            let area = c.len();
            let edge_count: usize = c.iter().map(|&ix| map.graph.neighbors(ix).count()).sum();
            let perim = 4 * area - edge_count;
            area * perim
        })
        .sum()
}

fn count_sides(c: &Vec<Nx>, width: usize) -> usize {
    use Dir::*;

    let c: HashSet<_> = c.iter().map(|nx| nx.index()).collect();
    let can_step = |curr, h| match h {
        North => curr >= width && c.contains(&(curr - width)),
        East => curr % width < width - 1 && c.contains(&(curr + 1)),
        South => curr / width < width - 1 && c.contains(&(curr + width)),
        West => curr % width > 0 && c.contains(&(curr - 1)),
    };
    let take_step = |curr, h| match h {
        North => curr - width,
        East => curr + 1,
        South => curr + width,
        West => curr - 1,
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

fn part_two(map: &Map, components: &Vec<Component>) -> usize {
    components
        .iter()
        .map(|c| {
            let area = c.len();
            let side_count = count_sides(c, map.width);
            area * side_count
        })
        .sum()
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
        let map = parse(EXAMPLE_1);
        let comps = find_components(&map);
        assert_eq!(r"140", part_one(&map, &comps).to_string());
        assert_eq!(r"80", part_two(&map, &comps).to_string());
    }

    #[test]
    fn example_2() {
        let map = parse(EXAMPLE_2);
        let comps = find_components(&map);
        assert_eq!(r"772", part_one(&map, &comps).to_string());
        assert_eq!(r"436", part_two(&map, &comps).to_string());
    }

    #[test]
    fn example_3() {
        let map = parse(EXAMPLE_3);
        let comps = find_components(&map);
        assert_eq!(r"1930", part_one(&map, &comps).to_string());
        assert_eq!(r"1206", part_two(&map, &comps).to_string());
    }

    #[test]
    fn example_4() {
        let map = parse(EXAMPLE_4);
        let comps = find_components(&map);
        assert_eq!(r"236", part_two(&map, &comps).to_string());
    }

    #[test]
    fn example_5() {
        let map = parse(EXAMPLE_5);
        let comps = find_components(&map);
        assert_eq!(r"368", part_two(&map, &comps).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 12, do_solve).unwrap();
    }
}
