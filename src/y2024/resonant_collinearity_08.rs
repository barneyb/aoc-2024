use crate::block_print::BLOCK;
use crate::Part;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
use std::str::FromStr;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let model = input.parse().unwrap();
    tx.send(Part::Parse()).unwrap();
    tx.send(Part::A(part_one(&model).to_string())).unwrap();
    tx.send(Part::B(part_two(&model).to_string())).unwrap();
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

impl AddAssign for Pt {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl Sub for Pt {
    type Output = Pt;

    fn sub(self, rhs: Self) -> Self::Output {
        Pt::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Pt {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl Neg for Pt {
    type Output = Pt;

    fn neg(self) -> Self::Output {
        Pt::new(-self.x, -self.y)
    }
}

struct Model {
    antennas: HashMap<char, Vec<Pt>>,
    bounds: Pt,
}

impl FromStr for Model {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
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
        Ok(Model {
            antennas,
            bounds: Pt::new(max_x as isize, max_y as isize),
        })
    }
}

impl Model {
    fn is_within_area(&self, p: &Pt) -> bool {
        if p.x < 0 || p.y < 0 {
            return false;
        }
        p.x <= self.bounds.x && p.y <= self.bounds.y
    }

    #[allow(dead_code)]
    fn draw_freq(&self, freq: &char, antinodes: &HashSet<Pt>) {
        let Pt { x: max_x, y: max_y } = self.bounds;
        let ann: HashSet<_> = if let Some(ann) = self.antennas.get(freq) {
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
}

fn part_one(model: &Model) -> usize {
    let mut antinodes = HashSet::new();
    for anns in model.antennas.values() {
        for (i, &a) in anns.iter().enumerate() {
            for &b in &anns[(i + 1)..] {
                let md = b - a;
                for c in [a + -md, b + md] {
                    if model.is_within_area(&c) {
                        antinodes.insert(c);
                    }
                }
            }
        }
    }
    antinodes.len()
}

fn part_two(model: &Model) -> usize {
    let mut antinodes = HashSet::new();
    for anns in model.antennas.values() {
        for (i, &a) in anns.iter().enumerate() {
            antinodes.insert(a);
            for &b in &anns[(i + 1)..] {
                antinodes.insert(b);
                let md = b - a;
                let mut c = a - md;
                while model.is_within_area(&c) {
                    antinodes.insert(c);
                    c -= md;
                }
                c = b + md;
                while model.is_within_area(&c) {
                    antinodes.insert(c);
                    c += md;
                }
            }
        }
    }
    antinodes.len()
}

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
        let model = EXAMPLE_1.parse().unwrap();
        assert_eq!(r"14", part_one(&model).to_string());
        assert_eq!(r"34", part_two(&model).to_string());
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
