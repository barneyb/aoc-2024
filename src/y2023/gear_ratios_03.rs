use crate::Part;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

// numbers are never adjacent to each other
// numbers are never adjacent to multiple symbols
pub fn do_solve(input: &str, tx: Sender<Part>) {
    let schematic = parse(input);
    tx.send(Part::Other("parse".to_string())).unwrap();
    tx.send(Part::A(part_one(&schematic).to_string())).unwrap();
    tx.send(Part::B(part_two(&schematic).to_string())).unwrap();
    let graph = parse_graph(input);
    tx.send(Part::Other("graph".to_string())).unwrap();
    tx.send(Part::A(part_one_graph(&graph).to_string()))
        .unwrap();
    tx.send(Part::B(part_two_graph(&graph).to_string()))
        .unwrap();
}

/// number, start x, end x, y
type Num = (usize, usize, usize, usize);

/// x, y
type Point = (usize, usize);

#[derive(Debug, Eq, PartialEq)]
struct Schematic {
    nums: Vec<Num>,
    symbols: HashMap<Point, char>,
}

fn parse(input: &str) -> Schematic {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut nums: Vec<Num> = Vec::new();
    let mut symbols: HashMap<Point, char> = HashMap::new();
    let mut curr = None;
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                let d = c.to_digit(10).unwrap() as usize;
                if let Some((n, sx, _, y)) = curr {
                    curr = Some((n * 10 + d, sx, x, y))
                } else {
                    curr = Some((d, x, x, y))
                }
            } else {
                if let Some(num) = curr {
                    nums.push(num);
                    curr = None;
                }
                if is_symbol(*c) {
                    symbols.insert((x, y), *c);
                }
            }
        }
        if let Some(num) = curr {
            nums.push(num);
            curr = None;
        }
    }
    Schematic { nums, symbols }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Node {
    Num(Num),
    Symbol(Point, char),
}

type Graph = HashMap<Node, Vec<Node>>;

fn parse_graph(input: &str) -> Graph {
    let Schematic { nums, symbols } = parse(input);
    let mut graph: Graph = HashMap::new();
    for num in nums {
        if let Some(sp) = neighbors(&num).find(|p| symbols.contains_key(&p)) {
            let nn = Node::Num(num);
            let sn = Node::Symbol(sp, *symbols.get(&sp).unwrap());
            if let Some(adj) = graph.get_mut(&sn) {
                adj.push(nn.clone());
            } else {
                graph.insert(sn.clone(), vec![nn.clone()]);
            }
            graph.insert(nn, Vec::from([sn]));
        }
    }
    graph
}

fn part_one(schematic: &Schematic) -> usize {
    let Schematic { nums, symbols } = schematic;
    // println!("{nums:?}");
    // println!("{symbols:?}");
    let mut sum = 0;
    'outer: for num in nums {
        if neighbors(&num).any(|p| symbols.contains_key(&p)) {
            let (n, ..) = num;
            sum += n;
            continue 'outer;
        }
        // println!(
        //     "No symbol for {num:?} among {:?}",
        //     neighbors(&num).collect::<Vec<_>>()
        // )
    }
    sum
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

fn neighbors(num: &Num) -> Neighbors {
    let (_, x1, x2, y) = *num;
    let left_edge = x1 == 0;
    let x1 = if left_edge { x1 } else { x1 - 1 };
    Neighbors {
        x1,
        x2,
        y,
        curr: if left_edge {
            if y == 0 {
                // start on right
                Some((x2 + 1, y))
            } else {
                // start above first digit
                Some((x1, y - 1))
            }
        } else {
            // start on left
            Some((x1, y))
        },
    }
}

struct Neighbors {
    x1: usize,
    x2: usize,
    y: usize,
    curr: Option<Point>,
}

impl Iterator for Neighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr.take();
        if let Some(p) = &curr {
            let (x, y) = *p;
            self.curr = if y == self.y {
                // on the number's row
                Some(if x == self.x1 {
                    // left of number
                    if y > 0 {
                        (x, y - 1)
                    } else {
                        (self.x2 + 1, y)
                    }
                } else {
                    // right of number
                    (x, y + 1)
                })
            } else if y < self.y {
                // above the number
                Some(if x <= self.x2 { (x + 1, y) } else { (x, y + 1) })
            } else {
                // below the number
                if x > self.x1 {
                    Some((x - 1, y))
                } else {
                    None
                }
            };
        }
        curr
    }
}

fn part_one_graph(graph: &Graph) -> usize {
    let mut sum = 0;
    for (node, adj) in graph {
        if !adj.is_empty() {
            if let Node::Num((n, ..)) = node {
                sum += n;
            }
        }
    }
    sum
}

fn part_two(schematic: &Schematic) -> usize {
    let Schematic { nums, symbols } = schematic;
    let mut firsts = HashMap::new();
    let mut sum = 0;
    for num in nums {
        if let Some(p) = neighbors(&num).find(|p| symbols.get(&p) == Some(&'*')) {
            let (n, ..) = num;
            if let Some(f) = firsts.get(&p) {
                sum += n * f;
            } else {
                firsts.insert(p, *n);
            }
        }
    }
    sum
}

fn part_two_graph(graph: &Graph) -> usize {
    let mut sum = 0;
    for (node, adj) in graph {
        if let Node::Symbol(_, '*') = node {
            if adj.len() == 2 {
                sum += adj
                    .iter()
                    .map(|nn| {
                        if let Node::Num((n, ..)) = nn {
                            *n
                        } else {
                            panic!("Symbol {node:?} is adjacent to non-number {nn:?}?!")
                        }
                    })
                    .product::<usize>();
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    const EXAMPLE_1: &str = r#"467..114..
...*......
..35...633
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    lazy_static! {
        static ref SCHEMATIC_1: Schematic = Schematic {
            nums: vec![
                (467, 0, 2, 0),
                (114, 5, 7, 0),
                (35, 2, 3, 2),
                (633, 7, 9, 2),
                (617, 0, 2, 4),
                (58, 7, 8, 5),
                (592, 2, 4, 6),
                (755, 6, 8, 7),
                (664, 1, 3, 9),
                (598, 5, 7, 9)
            ],
            symbols: HashMap::from([
                ((6, 3), '#'),
                ((3, 4), '*'),
                ((5, 5), '+'),
                ((3, 1), '*'),
                ((3, 8), '$'),
                ((5, 8), '*')
            ])
        };
    }

    #[test]
    fn parse_1() {
        assert_eq!(*SCHEMATIC_1, parse(EXAMPLE_1));
    }
    #[test]
    fn example_1() {
        assert_eq!(r"4361", part_one(&*SCHEMATIC_1).to_string());
        assert_eq!(r"467835", part_two(&*SCHEMATIC_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2023, 3, do_solve).unwrap();
    }
}
