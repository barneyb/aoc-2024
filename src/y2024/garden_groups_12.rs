use crate::Part;
use petgraph::graph::NodeIndex;
use petgraph::prelude::UnGraph;
use petgraph::{Graph, Undirected};
use std::collections::{HashSet, VecDeque};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let map = parse(input);
    tx.send(Part::A(part_one(&map).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

struct Map {
    graph: Graph<(), (), Undirected, usize>,
    width: usize,
    height: usize,
}

fn parse(input: &str) -> Map {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let mut edges = Vec::with_capacity(width * height);
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
        height,
    }
}

fn part_one(map: &Map) -> usize {
    let mut components = Vec::new();
    let mut visited = HashSet::new();
    for n in map.graph.node_indices() {
        if !visited.contains(&n) {
            components.push(find_extent(n, &map.graph, &mut visited))
        }
    }
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

fn find_extent(
    start: NodeIndex<usize>,
    graph: &Graph<(), (), Undirected, usize>,
    visited: &mut HashSet<NodeIndex<usize>>,
) -> Vec<NodeIndex<usize>> {
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
        let map = parse(EXAMPLE_1);
        assert_eq!(r"140", part_one(&map).to_string());
    }

    #[test]
    fn example_2() {
        let map = parse(EXAMPLE_2);
        assert_eq!(r"772", part_one(&map).to_string());
    }

    #[test]
    fn example_3() {
        let map = parse(EXAMPLE_3);
        assert_eq!(r"1930", part_one(&map).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 12, do_solve).unwrap();
    }
}
