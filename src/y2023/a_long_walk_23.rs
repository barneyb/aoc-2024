use crate::viz::graphviz::render_weighted;
use crate::Part;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

type Pt = (usize, usize);

fn part_one(input: &str) -> usize {
    let (start, goal, graph) = parse(input);
    let mut forks = Vec::new();
    forks.push(goal);
    let mut queue = VecDeque::from([(start.0, 1)]);
    let mut visited = HashSet::from([start]);
    while let Some(curr) = queue.pop_front() {
        if !visited.insert(curr) {
            continue;
        }
        let (x, y) = curr;
        if [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
            .iter()
            .filter(|p| graph.contains_key(p))
            .take(3)
            .count()
            > 2
        {
            forks.push(curr);
        }
        for n in graph.get(&curr).unwrap() {
            queue.push_back(*n);
        }
    }
    let targets: HashSet<_> = forks.into_iter().collect();
    let mut mini: HashMap<_, Vec<(Pt, usize)>> = HashMap::new();
    let mut queue: VecDeque<_> = targets.iter().map(|&t| (t, t, 0)).collect();
    queue.push_front((start, start, 0));
    let mut visited = HashSet::new();
    while let Some((start, curr, dist)) = queue.pop_front() {
        if targets.contains(&curr) {
            if dist > 0 {
                mini.entry(start).or_default().push((curr, dist));
            }
            for n in graph.get(&curr).unwrap() {
                queue.push_back((curr, *n, 1));
            }
        } else {
            if !visited.insert(curr) {
                continue;
            }
            for n in graph.get(&curr).unwrap() {
                queue.push_back((start, *n, dist + 1));
            }
        }
    }
    render_weighted(&mini);
    Dfs::longest(&mini, start, goal)
}

struct Dfs<'a, N> {
    graph: &'a HashMap<N, Vec<(N, usize)>>,
    goal: N,
    visited: HashSet<N>,
}

impl<'a, N> Dfs<'a, N>
where
    N: Copy + Eq + Hash + Debug,
{
    fn longest(graph: &'a HashMap<N, Vec<(N, usize)>>, start: N, goal: N) -> usize {
        Dfs {
            graph,
            goal,
            visited: HashSet::new(),
        }
        .dfs(start)
    }

    fn dfs(&mut self, start: N) -> usize {
        if start == self.goal {
            return 0;
        }
        if !self.visited.insert(start) {
            return 0;
        }
        let longest = self
            .graph
            .get(&start)
            .unwrap()
            .iter()
            .map(|&(e, d)| d + self.dfs(e))
            .max()
            .unwrap();
        self.visited.remove(&start);
        longest
    }
}

fn parse(input: &str) -> (Pt, Pt, HashMap<Pt, Vec<Pt>>) {
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = (grid[0].iter().position(|c| *c == '.').unwrap(), 0);
    let max_y = grid.len() - 1;
    let goal = (grid[max_y].iter().position(|c| *c == '.').unwrap(), max_y);
    let mut graph: HashMap<Pt, Vec<Pt>> = HashMap::new();
    graph.entry(goal).or_default();
    let mut add_edge = |src, tgt| {
        if src == goal || tgt == start {
            return;
        }
        graph.entry(src).or_default().push(tgt);
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
    (start, goal, graph)
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

    #[test]
    fn test_real_input() {
        crate::with_input(2023, 23, do_solve).unwrap();
    }
}
