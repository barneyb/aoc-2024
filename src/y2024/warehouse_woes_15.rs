use crate::geom2d::Dir;
use crate::Part;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

type Pt = (usize, usize);

fn step(p: Pt, d: Dir) -> Pt {
    let (x, y) = p;
    match d {
        Dir::North => (x, y - 1),
        Dir::East => (x + 1, y),
        Dir::South => (x, y + 1),
        Dir::West => (x - 1, y),
    }
}

#[derive(Debug)]
struct Model {
    width: usize,
    height: usize,
    floor: HashSet<Pt>,
    boxes: HashSet<Pt>,
    instructions: Vec<Dir>,
    bot: Pt,
}

impl FromStr for Model {
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
        Ok(Model {
            width: width.unwrap(),
            height: height.unwrap(),
            floor,
            boxes,
            instructions,
            bot: bot.unwrap(),
        })
    }
}

impl Display for Model {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.width {
            if y > 0 {
                f.write_char('\n')?;
            }
            for x in 0..self.height {
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
        Ok(())
    }
}

fn part_one(input: &str) -> usize {
    let mut model: Model = input.parse().unwrap();
    // println!("Initial State:\n{model}");
    for &d in &model.instructions {
        let tgt = step(model.bot, d);
        let mut scan = tgt;
        while model.boxes.contains(&scan) {
            scan = step(scan, d);
        }
        if model.floor.contains(&scan) {
            if scan != tgt {
                model.boxes.remove(&tgt);
                model.boxes.insert(scan);
            }
            model.bot = tgt;
        }
        // println!("\nMove {d}:\n{model}");
    }
    model.boxes.iter().map(|(x, y)| x + 100 * y).sum()
}

// fn part_two(input: &str) -> usize {
//     99999
// }

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

    const EXAMPLE_2: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    #[test]
    fn example_1() {
        assert_eq!(r"10092", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"2028", part_one(EXAMPLE_2).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 15, do_solve).unwrap();
    }
}
