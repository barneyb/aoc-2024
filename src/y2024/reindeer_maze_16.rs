use crate::geom2d::Dir;
use crate::geom2d::Dir::*;
use crate::Part;
use petgraph::prelude::{EdgeRef, NodeIndex};
use petgraph::Graph;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let maze = Maze::from(input);
    tx.send(Part::Parse()).unwrap();
    let (a, b) = both_parts(&maze);
    tx.send(Part::Both(a.to_string(), b.to_string())).unwrap();
}

struct Maze {
    map: Graph<(), (Dir, u32)>,
    start: NodeIndex,
    goal: NodeIndex,
}

impl From<&str> for Maze {
    fn from(input: &str) -> Self {
        let mut map = Graph::new();
        let mut start = None;
        let mut goal = None;
        let mut lookup = HashMap::new();
        let mut width = 0;
        for (y, line) in input.lines().enumerate() {
            if width == 0 {
                width = line.chars().count();
            }
            let row_offset = y * width;
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    continue;
                }
                let nx = map.add_node(());
                lookup.insert(x + row_offset, nx);
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
                if let Some(&ox) = lookup.get(&(x - 1 + row_offset)) {
                    map.add_edge(nx, ox, (West, 1));
                    map.add_edge(ox, nx, (East, 1));
                }
                if let Some(&ox) = lookup.get(&(x + row_offset - width)) {
                    map.add_edge(nx, ox, (North, 1));
                    map.add_edge(ox, nx, (South, 1));
                }
            }
        }
        Maze {
            map,
            start: start.unwrap(),
            goal: goal.unwrap(),
        }
    }
}

enum Cons<T> {
    Nil,
    Cons(T, Rc<Cons<T>>),
}

impl<T> Cons<T> {
    fn of(t: T) -> Self {
        Cons::Cons(t, Rc::new(Cons::Nil))
    }

    fn push(head: T, tail: Rc<Cons<T>>) -> Self {
        Cons::Cons(head, tail.clone())
    }
}

type Coords = (NodeIndex, Dir);
type State = (NodeIndex, Dir, u32, Rc<Cons<NodeIndex>>);

fn both_parts(maze: &Maze) -> (u32, u32) {
    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back((maze.start, East, 0, Rc::new(Cons::of(maze.start))));
    let mut visited: HashMap<Coords, u32> = HashMap::new();
    let mut best = u32::MAX;
    let mut good_seats = Vec::new();
    while let Some((nx, h, cost, path)) = queue.pop_front() {
        if cost > best {
            continue;
        }
        if nx == maze.goal {
            if cost < best {
                best = cost;
                good_seats.clear();
            }
            good_seats.push(path.clone());
            continue;
        }
        if let Some(&c) = visited.get(&(nx, h)) {
            // already been here, facing this way
            if cost > c {
                // and it was lower cost
                continue;
            }
        }
        visited.insert((nx, h), cost);
        for ex in maze.map.edges(nx) {
            let &(d, mut c) = ex.weight();
            if d == h.turn_around() {
                continue;
            }
            if d != h {
                c += 1000;
            }
            let ox = ex.target();
            let next_cost = cost + c;
            if next_cost <= best {
                queue.push_back((ox, d, next_cost, Rc::new(Cons::push(ox, path.clone()))))
            }
        }
    }
    let on_path = flatten(good_seats);
    (best, count_distinct(on_path))
}

fn flatten(good_seats: Vec<Rc<Cons<NodeIndex>>>) -> Vec<NodeIndex> {
    let mut on_path = Vec::new();
    for mut seat in good_seats {
        loop {
            match seat.deref() {
                Cons::Nil => break,
                Cons::Cons(nx, l) => {
                    on_path.push(*nx);
                    seat = l.to_owned();
                }
            }
        }
    }
    on_path
}

fn count_distinct<T>(mut on_path: Vec<T>) -> u32
where
    T: Copy + Eq + Ord,
{
    on_path.sort();
    let mut prev = on_path[0];
    let mut count = 1;
    for curr in on_path {
        if curr != prev {
            count += 1
        }
        prev = curr;
    }
    count
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
