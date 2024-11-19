use crate::Part;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter, Write};
use std::sync::mpsc::Sender;
use Tile::*;

#[derive(Debug, Eq, PartialEq)]
enum Vein {
    Vert(i32, (i32, i32)),
    Horiz((i32, i32), i32),
}

type Veins = Vec<Vein>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Sand,
    Clay,
    Flowing,
    Settled,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Sand => '.',
            Clay => '#',
            Flowing => '|',
            Settled => '~',
        })
    }
}

type Point = (i32, i32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SpillOrSettle {
    Spill,
    Settle,
}

struct Map {
    tiles: HashMap<Point, Tile>,
    falls: VecDeque<Point>,
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
    flow_count: usize,
    settled_count: usize,
}

const SPRING_X: i32 = 500;

impl Map {
    fn get_tile(&self, p: &Point) -> Option<&Tile> {
        self.tiles.get(p)
    }

    fn set_tile(&mut self, p: Point, t: Tile) {
        match t {
            Flowing => self.flow_count += 1,
            Settled => self.settled_count += 1,
            _ => panic!("Set {p:?} to {t}?!"),
        }
        if let Some(prev) = self.tiles.insert(p, t) {
            match prev {
                Flowing => self.flow_count -= 1,
                _ => panic!("Overwrote a {prev} at {p:?} w/ a {t}?!"),
            }
        }
    }

    fn turn_on_spring(&mut self) {
        self.falls.push_back((SPRING_X, self.y_bounds.0));
        while let Some((x, sy)) = self.falls.pop_front() {
            for y in sy..=self.y_bounds.1 {
                let mut p = (x, y);
                match self.get_tile(&p) {
                    None | Some(Sand) => self.set_tile(p, Flowing),
                    Some(Clay | Settled) => {
                        while let SpillOrSettle::Settle = self.spill_or_settle(p) {
                            p = (x, p.1 - 1)
                        }
                        break;
                    }
                    Some(Flowing) => break,
                }
            }
        }
    }

    fn spill_or_settle(&mut self, p: Point) -> SpillOrSettle {
        let (x, y) = p;
        fn can_spread_over(_: i32, t: Option<&Tile>) -> bool {
            match t {
                Some(Clay | Settled) => true,
                _ => false,
            }
        }
        let min_floor = self.find_extent(p, -1, can_spread_over);
        let max_floor = self.find_extent(p, 1, can_spread_over);
        let spread_y = y - 1;
        let spread_from = (x, spread_y);
        let min_spread = self.find_extent(spread_from, -1, |x, t| match t {
            Some(Clay | Settled) => false,
            _ => x >= min_floor - 1,
        });
        let max_spread = self.find_extent(spread_from, 1, |x, t| match t {
            Some(Clay | Settled) => false,
            _ => x <= max_floor + 1,
        });
        let wall_min = min_spread > min_floor;
        let wall_max = max_spread < max_floor;
        if wall_min && wall_max {
            self.fill(Settled, min_spread, max_spread, spread_y);
            return SpillOrSettle::Settle;
        }
        self.fill(Flowing, min_spread, max_spread, spread_y);
        if !wall_min {
            self.falls.push_back((min_spread, spread_y + 1))
        }
        if !wall_max {
            self.falls.push_back((max_spread, spread_y + 1))
        }
        SpillOrSettle::Spill
    }

    fn find_extent<F>(&self, (mut x, y): Point, dx: i32, test: F) -> i32
    where
        F: Fn(i32, Option<&Tile>) -> bool,
    {
        while test(x, self.get_tile(&(x, y))) {
            x += dx;
        }
        x - dx // the previous one is the extent
    }

    fn fill(&mut self, t: Tile, x1: i32, x2: i32, y: i32) {
        for x in x1..=x2 {
            self.set_tile((x, y), t)
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x1, x2) = self.x_bounds;
        let (x1, x2) = (x1 - 1, x2 + 1);
        let (y1, y2) = self.y_bounds;
        let n_width = y2.to_string().len();
        let sand = Sand.to_string();
        write!(f, "{:1$} ", "", n_width,)?;
        for x in x1..=x2 {
            write!(f, "{}", if x == SPRING_X { "+" } else { &sand },)?;
        }
        writeln!(f)?;
        for y in y1..=y2 {
            write!(f, "{y:0$} ", n_width)?;
            for x in x1..=x2 {
                write!(
                    f,
                    "{}",
                    if let Some(t) = self.tiles.get(&(x, y)) {
                        t
                    } else {
                        &Sand
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let model = parse(input);
    let (a, b) = both_parts(&model);
    tx.send(Part::A(a.to_string())).unwrap();
    tx.send(Part::B(b.to_string())).unwrap();
}

fn parse(input: &str) -> Veins {
    // x=495, y=2..7
    // y=7, x=495..501
    let pattern = Regex::new(r"([xy])=(\d+), [yx]=(\d+)\.\.(\d+)").unwrap();
    input
        .lines()
        .map(|l| pattern.captures(l).unwrap())
        .map(|cs| {
            let a: i32 = cs.get(2).unwrap().as_str().parse().unwrap();
            let b: i32 = cs.get(3).unwrap().as_str().parse().unwrap();
            let c: i32 = cs.get(4).unwrap().as_str().parse().unwrap();
            match cs.get(1) {
                Some(m) => match m.as_str() {
                    "x" => Vein::Vert(a, (b, c)),
                    "y" => Vein::Horiz((b, c), a),
                    _ => panic!("Malformed vein spec?!"),
                },
                _ => panic!("Didn't capture vein spec?!"),
            }
        })
        .collect()
}

fn both_parts(veins: &Veins) -> (usize, usize) {
    let mut map = build_map(veins);
    map.turn_on_spring();
    (map.flow_count + map.settled_count, map.settled_count)
}

fn build_map(veins: &Veins) -> Map {
    let mut tiles = HashMap::new();
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_min = i32::MAX;
    let mut y_max = i32::MIN;
    for v in veins {
        match *v {
            Vein::Vert(x, (y1, y2)) => {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
                y_min = y_min.min(y1);
                y_max = y_max.max(y2);
                for y in y1..=y2 {
                    tiles.insert((x, y), Clay);
                }
            }
            Vein::Horiz((x1, x2), y) => {
                x_min = x_min.min(x1);
                x_max = x_max.max(x2);
                y_min = y_min.min(y);
                y_max = y_max.max(y);
                for x in x1..=x2 {
                    tiles.insert((x, y), Clay);
                }
            }
        }
    }
    Map {
        tiles,
        falls: VecDeque::new(),
        x_bounds: (x_min, x_max),
        y_bounds: (y_min, y_max),
        flow_count: 0,
        settled_count: 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    const EXAMPLE_1: &str = r#"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"#;

    lazy_static! {
        static ref MODEL_1: Veins = vec![
            Vein::Vert(495, (2, 7)),
            Vein::Horiz((495, 501), 7),
            Vein::Vert(501, (3, 7)),
            Vein::Vert(498, (2, 4)),
            Vein::Vert(506, (1, 2)),
            Vein::Vert(498, (10, 13)),
            Vein::Vert(504, (10, 13)),
            Vein::Horiz((498, 504), 13),
        ];
    }

    #[test]
    fn test_parse() {
        assert_eq!(*MODEL_1, parse(EXAMPLE_1))
    }

    #[test]
    fn example_1() {
        assert_eq!((57, 29), both_parts(&*MODEL_1));
    }

    #[test]
    fn initial_map() {
        assert_eq!(
            r#"   ......+.......
 1 ............#.
 2 .#..#.......#.
 3 .#..#..#......
 4 .#..#..#......
 5 .#.....#......
 6 .#.....#......
 7 .#######......
 8 ..............
 9 ..............
10 ....#.....#...
11 ....#.....#...
12 ....#.....#...
13 ....#######...
"#,
            build_map(&*MODEL_1).to_string()
        );
    }

    #[test]
    fn final_map() {
        let mut map = build_map(&*MODEL_1);
        map.turn_on_spring();
        assert_eq!(
            r#"   ......+.......
 1 ......|.....#.
 2 .#..#||||...#.
 3 .#..#~~#|.....
 4 .#..#~~#|.....
 5 .#~~~~~#|.....
 6 .#~~~~~#|.....
 7 .#######|.....
 8 ........|.....
 9 ...|||||||||..
10 ...|#~~~~~#|..
11 ...|#~~~~~#|..
12 ...|#~~~~~#|..
13 ...|#######|..
"#,
            map.to_string()
        );
    }

    /*
    139              |
    140   ||||||||||||| ||||#
    141   |#~~~~~~~~~~~~~~~~#
    142   |#~~~~~~~~~~||||||#
    143   |#~~~~~~~~#~#~~~~~#
    144   |#~~~~~~~~#~#~~~~~#
    145   |#~~~~~~~~#~#~~~~~#
    146   |#~~~~~~~~#~#~~~~~#
    147   |#~~~~~~~~###~~~~~#
    148   |#~~~~~~~~~~~~~~~~#
    149   |#~~~~~~~~~~~~~~~~#
    150   |##################
    151   |

     44444555555555
     99999000000000
     56789012345678
    0     |       #  x=508, y=0..1
    1||||||| |||# #  x=506, y=1..8
    2|#~~~~~~~~~#    x=496, y=2..8
    3|#~~~~|||||#
    4|#~~#~#~~~~#    x=499, y=4..6
    5|#~~#~#~~~~#    x=501, y=4..6
    6|#~~###~~~~#    y=6, x=499..501
    7|#~~~~~~~~~#
    8|###########    y=8, x=496..506
    9|
     */

    #[test]
    fn suspended_pool() {
        let input = r#"x=508, y=0..1
x=506, y=1..8
x=496, y=2..8
x=499, y=4..6
x=502, y=4..6
y=6, x=499..502
y=8, x=496..506"#;
        let veins = parse(input);
        let mut map = build_map(&veins);
        map.turn_on_spring();
        let result = map.to_string();
        println!("{result}");
        assert_eq!(
            r#"  .....+.........
0 .....|.......#.
1 |||||||||||#.#.
2 |#~~~~~~~~~#...
3 |#~~~~~~~~~#...
4 |#~~#~~#~~~#...
5 |#~~#~~#~~~#...
6 |#~~####~~~#...
7 |#~~~~~~~~~#...
8 |###########...
"#,
            &result
        );
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2018, 17, do_solve).unwrap();
    }
}
