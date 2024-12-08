use crate::block_print::BLOCK;
use crate::Part;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let model = parse(input);
    tx.send(Part::Parse()).unwrap();
    tx.send(Part::A(part_one(&model).to_string())).unwrap();
    // tx.send(Part::Other(part_two(&model).to_string())).unwrap();
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Pt {
    pub x: isize,
    pub y: isize,
}

impl Display for Pt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Pt {
    pub(crate) fn new(x: isize, y: isize) -> Pt {
        Pt { x, y }
    }
}

impl Add for Pt {
    type Output = Pt;

    fn add(self, rhs: Self) -> Self::Output {
        Pt::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Pt {
    type Output = Pt;

    fn sub(self, rhs: Self) -> Self::Output {
        Pt::new(self.x - rhs.x, self.y - rhs.y)
    }
}

struct Model {
    antennas: HashMap<char, Vec<Pt>>,
    bounds: Pt,
}

fn parse(input: &str) -> Model {
    let mut antennas: HashMap<_, Vec<_>> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input.lines().enumerate() {
        if y == 0 {
            max_x = line.len() - 1;
        }
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            antennas
                .entry(c)
                .or_default()
                .push(Pt::new(x as isize, y as isize))
        }
        max_y = y;
    }
    Model {
        antennas,
        bounds: Pt::new(max_x as isize, max_y as isize),
    }
}

fn part_one(model: &Model) -> usize {
    let mut antinodes = HashSet::new();
    // println!("{}: {:?}", model.antennas.len(), model.antennas.keys());
    for anns in model.antennas.values() {
        let mut anti = HashSet::new();
        // println!("Freq {freq}: {} antennas", anns.len());
        for (i, &a) in anns.iter().enumerate() {
            // println!("   {a}");
            for &b in &anns[(i + 1)..] {
                let md = b - a;
                let lo = a - md;
                let hi = b + md;
                // println!("      {b} delta {md} -> {lo} & {hi}");
                anti.insert(lo);
                anti.insert(hi);
            }
        }
        // assert_eq!((anns.len() * (anns.len() - 1)), anti.len());
        // if *freq == 'A' || *freq == 'l' {
        //     draw_freq(model, freq, &anti)
        // }
        antinodes.extend(anti);
    }
    // println!("{antinodes:?}");
    let Pt { x: max_x, y: max_y } = model.bounds;
    antinodes = antinodes
        .into_iter()
        .filter(|p| p.x >= 0 && p.x <= max_x && p.y >= 0 && p.y <= max_y)
        .collect();
    // println!("{antinodes:?}");
    // draw_freq(model, &' ', &antinodes);
    antinodes.len()
}

#[allow(dead_code)]
fn draw_freq(model: &Model, freq: &char, antinodes: &HashSet<Pt>) {
    let Pt { x: max_x, y: max_y } = model.bounds;
    let ann: HashSet<_> = if let Some(ann) = model.antennas.get(freq) {
        ann.iter().collect()
    } else {
        Default::default()
    };
    print!("   ");
    for x in (0..=max_x).step_by(5) {
        print!("|{x:>2}  ")
    }
    for y in 0..=max_y {
        print!("\n{y:>2} ");
        for x in 0..=max_x {
            let p = Pt::new(x, y);
            print!(
                "{}",
                if antinodes.contains(&p) {
                    if ann.contains(&p) {
                        BLOCK
                    } else {
                        '#'
                    }
                } else if ann.contains(&p) {
                    *freq
                } else {
                    '.'
                }
            )
        }
    }
    println!()
}

// fn part_two(model: &Model) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn example_1() {
        let model = parse(EXAMPLE_1);
        assert_eq!(r"14", part_one(&model).to_string());
    }

    #[test]
    fn pt_ops() {
        assert_eq!(Pt::new(1, 2), Pt::new(4, 6) - Pt::new(3, 4));
        assert_eq!(Pt::new(4, 6), Pt::new(3, 4) + Pt::new(1, 2));
        assert_eq!(Pt::new(0, 2), Pt::new(3, 6) - Pt::new(3, 4));
        assert_eq!(Pt::new(3, 6), Pt::new(3, 4) + Pt::new(0, 2));
        assert_eq!(Pt::new(1, 0), Pt::new(4, 4) - Pt::new(3, 4));
        assert_eq!(Pt::new(4, 4), Pt::new(3, 4) + Pt::new(1, 0));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 8, do_solve).unwrap();
    }
}
