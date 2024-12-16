use crate::geom2d::Dir;
use crate::geom2d::Dir::*;
use crate::Part;
use petgraph::prelude::NodeIndex;
use petgraph::Graph;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let maze = Maze::from(input);
    tx.send(Part::Parse()).unwrap();
    let (a, b) = both_parts(&maze);
    tx.send(Part::A(a.to_string())).unwrap();
    tx.send(Part::B(b.to_string())).unwrap();
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
type State = (NodeIndex, Dir, usize, Vec<NodeIndex>);

fn both_parts(maze: &Maze) -> (usize, usize) {
    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back((maze.start, East, 0, vec![maze.start]));
    let mut visited: HashMap<Coords, usize> = HashMap::new();
    let mut best = usize::MAX;
    let mut good_seats = HashSet::new();
    while let Some((nx, h, cost, path)) = queue.pop_front() {
        if let Some(&c) = visited.get(&(nx, h)) {
            // already been here, facing this way
            if cost > c {
                // and it was lower cost
                continue;
            }
        }
        visited.insert((nx, h), cost);
        if nx == maze.goal {
            if cost < best {
                best = cost;
                good_seats.clear();
                good_seats.extend(path);
            } else if cost == best {
                good_seats.extend(path);
            }
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
            let mut new_path = path.clone();
            new_path.push(ox);
            queue.push_back((ox, d, cost + c, new_path))
        }
    }
    (best, good_seats.len())
}

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
        assert_eq!((7036, 45), both_parts(&EXAMPLE_1.into()));
    }

    #[test]
    fn example_2() {
        assert_eq!((11048, 64), both_parts(&EXAMPLE_2.into()));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 16, do_solve).unwrap();
    }
}
