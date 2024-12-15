use crate::geom2d::Dir;
use crate::Part;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use std::sync::mpsc::Sender;
use Dir::*;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

type Pt = (usize, usize);

fn step(p: Pt, d: Dir) -> Pt {
    step_by(p, d, 1)
}

fn step_by(p: Pt, d: Dir, n: usize) -> Pt {
    let (x, y) = p;
    match d {
        North => (x, y - n),
        East => (x + n, y),
        South => (x, y + n),
        West => (x - n, y),
    }
}

#[derive(Debug)]
struct Warehouse {
    width: usize,
    height: usize,
    floor: HashSet<Pt>,
    boxes: HashSet<Pt>,
    instructions: Vec<Dir>,
    bot: Pt,
}

impl FromStr for Warehouse {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut floor = HashSet::new();
        let mut boxes = HashSet::new();
        let mut instructions = Vec::new();
        let mut bot = None;
        let mut width = None;
        let mut height = None;
        let mut before_break = true;
        for (y, line) in input.lines().enumerate() {
            if line == "" {
                before_break = false;
                continue;
            }
            if before_break {
                height = Some(y + 1);
                let line: Vec<_> = line.chars().collect();
                width = Some(line.len());
                for (x, &c) in line.iter().enumerate() {
                    if c == '@' {
                        if let Some(p) = bot {
                            panic!("Found two bots?! {p:?} and ({x}, {y})?!")
                        }
                        bot = Some((x, y));
                    } else if c == 'O' {
                        boxes.insert((x, y));
                    }
                    if c != '#' {
                        floor.insert((x, y));
                    }
                }
            } else {
                instructions.extend(line.chars().map(Dir::from))
            }
        }
        Ok(Warehouse {
            width: width.unwrap(),
            height: height.unwrap(),
            floor,
            boxes,
            instructions,
            bot: bot.unwrap(),
        })
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y > 0 {
                f.write_char('\n')?;
            }
            for x in 0..self.width {
                let p = (x, y);
                if p == self.bot {
                    f.write_char('@')?;
                } else if self.boxes.contains(&p) {
                    f.write_char('O')?;
                } else if self.floor.contains(&p) {
                    f.write_char('.')?;
                } else {
                    f.write_char('#')?;
                }
            }
        }
        // let mut bs: Vec<_> = self.boxes.iter().collect();
        // bs.sort();
        // write!(f, "\nboxes: {bs:?}")?;
        Ok(())
    }
}

fn part_one(input: &str) -> usize {
    let mut wh: Warehouse = input.parse().unwrap();
    // println!("Initial State:\n{wh}");
    for &d in &wh.instructions {
        let tgt = step(wh.bot, d);
        let mut scan = tgt;
        while wh.boxes.contains(&scan) {
            scan = step(scan, d);
        }
        if wh.floor.contains(&scan) {
            if scan != tgt {
                wh.boxes.remove(&tgt);
                wh.boxes.insert(scan);
            }
            wh.bot = tgt;
        }
        // println!("\nMove {d}:\n{wh}");
    }
    wh.boxes.iter().map(|(x, y)| x + 100 * y).sum()
}

#[derive(Debug)]
struct BiggerWarehouse {
    width: usize,
    height: usize,
    floor: HashSet<Pt>,
    lefts: HashSet<Pt>,
    rights: HashSet<Pt>,
    instructions: Vec<Dir>,
    bot: Pt,
}

impl FromStr for BiggerWarehouse {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut floor = HashSet::new();
        let mut lefts = HashSet::new();
        let mut rights = HashSet::new();
        let mut instructions = Vec::new();
        let mut bot = None;
        let mut width = None;
        let mut height = None;
        let mut before_break = true;
        for (y, line) in input.lines().enumerate() {
            if line == "" {
                before_break = false;
                continue;
            }
            if before_break {
                height = Some(y + 1);
                let line: Vec<_> = line.chars().collect();
                width = Some(line.len() * 2);
                for (x, &c) in line.iter().enumerate() {
                    let x = x * 2;
                    if c == '@' {
                        if let Some(p) = bot {
                            panic!("Found two bots?! {p:?} and ({x}, {y})?!")
                        }
                        bot = Some((x, y));
                    } else if c == 'O' {
                        lefts.insert((x, y));
                        rights.insert((x + 1, y));
                    }
                    if c != '#' {
                        floor.insert((x, y));
                        floor.insert((x + 1, y));
                    }
                }
            } else {
                instructions.extend(line.chars().map(Dir::from))
            }
        }
        Ok(BiggerWarehouse {
            width: width.unwrap(),
            height: height.unwrap(),
            floor,
            lefts,
            rights,
            instructions,
            bot: bot.unwrap(),
        })
    }
}

impl Display for BiggerWarehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y > 0 {
                f.write_char('\n')?;
            }
            for x in 0..self.width {
                let p = (x, y);
                if p == self.bot {
                    f.write_char('@')?;
                } else if self.lefts.contains(&p) {
                    f.write_char('[')?;
                } else if self.rights.contains(&p) {
                    f.write_char(']')?;
                } else if self.floor.contains(&p) {
                    f.write_char('.')?;
                } else {
                    f.write_char('#')?;
                }
            }
        }
        // let mut bs: Vec<_> = self.lefts.iter().collect();
        // bs.sort();
        // write!(f, "\nlefts: {bs:?}")?;
        // let mut bs: Vec<_> = self.rights.iter().collect();
        // bs.sort();
        // write!(f, "\nrights: {bs:?}")?;
        Ok(())
    }
}

fn part_two(input: &str) -> usize {
    let mut wh: BiggerWarehouse = input.parse().unwrap();
    // println!("Initial State:\n{wh}");
    let mut visited = HashSet::new();
    let mut lefts_to_move = vec![];
    'hit_wall: for &d in &wh.instructions {
        // println!("\nMove {d}:");
        let mut frontier = HashSet::from([wh.bot]);
        visited.clear();
        lefts_to_move.clear();
        while !frontier.is_empty() {
            let mut next = HashSet::new();
            for f in frontier {
                let s = step(f, d);
                if !visited.insert(s) {
                    continue;
                }
                if !wh.floor.contains(&s) {
                    // println!("Hit a wall at {s:?}");
                    continue 'hit_wall;
                }
                if wh.lefts.contains(&s) {
                    lefts_to_move.push(s);
                    if d != East {
                        next.insert(s);
                    }
                    next.insert(step(s, East)); // its other half
                }
                if wh.rights.contains(&s) {
                    if d != West {
                        next.insert(s);
                    }
                    let l = step(s, West);
                    lefts_to_move.push(l);
                    next.insert(l);
                }
            }
            frontier = next;
        }

        // println!("to move: {lefts_to_move:?}");
        visited.clear();
        for &l in lefts_to_move.iter().rev() {
            if !visited.insert(l) {
                continue;
            }
            let r = step(l, East);
            // remove current position
            wh.lefts.remove(&l);
            wh.rights.remove(&r);
            // insert new position
            wh.lefts.insert(step(l, d));
            wh.rights.insert(step(r, d));
        }
        wh.bot = step(wh.bot, d);
        // println!("{wh}");
    }
    wh.lefts.iter().map(|(x, y)| x + 100 * y).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    const EXAMPLE_1_WIDE: &str = r#"####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################"#;

    const EXAMPLE_2: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    const EXAMPLE_3: &str = r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#;

    #[test]
    fn example_1() {
        assert_eq!(r"10092", part_one(EXAMPLE_1).to_string());
        assert_eq!(r"9021", part_two(EXAMPLE_1).to_string());
    }

    #[test]
    fn twice_as_wide() {
        let wh: BiggerWarehouse = EXAMPLE_1.parse().unwrap();
        assert_eq!(EXAMPLE_1_WIDE, wh.to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"2028", part_one(EXAMPLE_2).to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(r"618", part_two(EXAMPLE_3).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 15, do_solve).unwrap();
    }
}
