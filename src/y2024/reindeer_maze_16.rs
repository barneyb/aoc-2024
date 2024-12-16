use crate::geom2d::Dir;
use crate::geom2d::Dir::*;
use crate::Part;
use petgraph::prelude::NodeIndex;
use petgraph::Graph;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let maze = Maze::from(input);
    tx.send(Part::A(part_one(&maze).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

type Pt = (usize, usize);

struct Maze {
    map: Graph<Pt, (Dir, usize)>,
    #[allow(dead_code)]
    lookup: HashMap<Pt, NodeIndex>,
    start: NodeIndex,
    goal: NodeIndex,
}

impl From<&str> for Maze {
    fn from(input: &str) -> Self {
        let mut map = Graph::new();
        let mut start = None;
        let mut goal = None;
        let mut prev: Option<Vec<_>> = None;
        let mut lookup = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            let mut p = None;
            let chars: Vec<_> = line.chars().collect();
            for (x, &c) in chars.iter().enumerate() {
                if c == '#' {
                    p = None;
                    continue;
                }
                let pt = (x, y);
                let nx = map.add_node(pt);
                lookup.insert(pt, nx);
                if c == 'S' {
                    if let Some(sx) = start {
                        panic!(
                            "Found second start at ({x}, {y})?! First at {:?}",
                            map.node_weight(sx)
                        )
                    }
                    start = Some(nx);
                }
                if c == 'E' {
                    if let Some(gx) = goal {
                        panic!(
                            "Found second goal at ({x}, {y})?! First at {:?}",
                            map.node_weight(gx)
                        )
                    }
                    goal = Some(nx);
                }
                if let Some(_) = p {
                    let ox = lookup[&(x - 1, y)];
                    map.add_edge(nx, ox, (West, 1));
                    map.add_edge(ox, nx, (East, 1));
                }
                if let Some(cs) = &prev {
                    if cs[x] != '#' {
                        let ox = lookup[&(x, y - 1)];
                        map.add_edge(nx, ox, (North, 1));
                        map.add_edge(ox, nx, (South, 1));
                    }
                }
                p = Some('.');
            }
            prev = Some(chars);
        }
        Maze {
            map,
            lookup,
            start: start.unwrap(),
            goal: goal.unwrap(),
        }
    }
}

type Coords = (NodeIndex, Dir);
type State = (NodeIndex, Dir, usize);

fn part_one(maze: &Maze) -> usize {
    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back((maze.start, East, 0));
    let mut visited: HashMap<Coords, usize> = HashMap::new();
    let mut best = usize::MAX;
    while let Some((nx, h, cost)) = queue.pop_front() {
        if let Some(&c) = visited.get(&(nx, h)) {
            // already been here, facing this way
            if cost >= c {
                // and it was lower cost
                continue;
            }
        }
        visited.insert((nx, h), cost);
        if nx == maze.goal {
            best = best.min(cost);
            continue;
        }
        for ox in maze.map.neighbors(nx) {
            let &(d, mut c) = maze
                .map
                .edge_weight(maze.map.find_edge(nx, ox).unwrap())
                .unwrap();
            if d == h.turn_around() {
                continue;
            }
            if d != h {
                c += 1000;
            }
            queue.push_back((ox, d, cost + c))
        }
    }
    best
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    const EXAMPLE_2: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    #[test]
    fn example_1() {
        assert_eq!(r"7036", part_one(&EXAMPLE_1.into()).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"11048", part_one(&EXAMPLE_2.into()).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 16, do_solve).unwrap();
    }
}
