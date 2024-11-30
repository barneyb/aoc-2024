use crate::viz::graphviz::render_dot;
use crate::Part;
use petgraph::dot::{Config, Dot};
use petgraph::prelude::NodeIndex;
use petgraph::Graph;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::Other(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

type Pt = (usize, usize);

fn part_one(input: &str) -> usize {
    let (start, goal, graph, lookup) = parse(input);
    render_dot(&Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    println!("{start:?} -> {goal:?}");
    println!("{:?} -> {:?}", lookup.get(&start), lookup.get(&goal));
    99999
}

fn parse(input: &str) -> (Pt, Pt, Graph<Pt, usize>, HashMap<Pt, NodeIndex>) {
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = (grid[0].iter().position(|c| *c == '.').unwrap(), 0);
    let max_y = grid.len() - 1;
    let goal = (grid[max_y].iter().position(|c| *c == '.').unwrap(), max_y);
    let mut graph: Graph<Pt, usize> = Graph::new();
    let mut lookup = HashMap::new();
    let mut add_edge = |src, tgt| {
        if src == goal || tgt == start {
            return;
        }
        let src = *lookup.entry(src).or_insert_with(|| graph.add_node(src));
        let tgt = *lookup.entry(tgt).or_insert_with(|| graph.add_node(tgt));
        graph.add_edge(src, tgt, 1);
    };
    // take the first step outside the loop
    let first = (start.0, 1);
    add_edge(start, first);
    let mut queue = VecDeque::from([first]);
    let mut visited = HashSet::from([start, goal]);
    while let Some(curr) = queue.pop_front() {
        if !visited.insert(curr) {
            continue;
        }
        let mut add = |tgt| {
            add_edge(curr, tgt);
            queue.push_back(tgt)
        };
        let (x, y) = curr;
        match grid[y][x] {
            '^' => add((x, y - 1)),
            '>' => add((x + 1, y)),
            'v' => add((x, y + 1)),
            '<' => add((x - 1, y)),
            '.' => {
                if let '.' | '^' | '>' | '<' = grid[y - 1][x] {
                    add((x, y - 1))
                }
                if let '.' | '>' | 'v' | '<' = grid[y + 1][x] {
                    add((x, y + 1))
                }
                if let '.' | '^' | '>' | 'v' = grid[y][x + 1] {
                    add((x + 1, y))
                }
                if let '.' | '^' | 'v' | '<' = grid[y][x - 1] {
                    add((x - 1, y))
                }
            }
            _ => {}
        }
    }
    (start, goal, graph, lookup)
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

    #[test]
    fn example_1() {
        assert_eq!(r"94", part_one(EXAMPLE_1).to_string());
        // assert_eq!(r"154", part_two(EXAMPLE_1).to_string());
    }

    // #[test]
    // fn test_real_input() {
    //     crate::with_input(2023, 23, do_solve).unwrap();
    // }
}
