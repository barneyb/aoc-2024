use crate::geom2d::Dir;
use crate::Part;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let mut model = input.parse().unwrap();
    tx.send(Part::Parse()).unwrap();
    tx.send(Part::A(part_one(&model).unwrap().to_string()))
        .unwrap();
    tx.send(Part::B(part_two(&mut model).to_string())).unwrap();
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Pt {
    x: usize,
    y: usize,
}

impl Display for Pt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[rustfmt::skip]
impl Pt {
    fn new(x: usize, y: usize) -> Pt {
        Pt { x, y }
    }

    fn step(&self, dir: Dir) -> Self {
        match dir {
            Dir::North => Pt { x: self.x    , y: self.y - 1, },
            Dir::East  => Pt { x: self.x + 1, y: self.y    , },
            Dir::South => Pt { x: self.x    , y: self.y + 1, },
            Dir::West  => Pt { x: self.x - 1, y: self.y    , },
        }
    }
}

struct Model {
    /// where the guard started
    guard: Pt,
    /// which way the guard was facing (always North)
    heading: Dir,
    /// max x,y values of the area. Min are 0,0
    bounds: Pt,
    /// y-coordinates of obstructions, keyed by their x-coord. Vec means O(n),
    /// but n is small enough that HashSet's O(1) dominates.
    obs_by_x: Vec<Vec<usize>>,
    /// a single extra obstruction, in addition to the above.
    extra_obs: Option<Pt>,
}

impl Display for Model {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Model {
            guard,
            heading,
            obs_by_x,
            bounds,
            ..
        } = self;
        for y in 0..bounds.y {
            if y > 0 {
                writeln!(f)?;
            }
            for x in 0..bounds.x {
                if guard.x == x && guard.y == y {
                    write!(f, "{heading}")?;
                } else {
                    if let Some(ys) = obs_by_x.get(x) {
                        if ys.contains(&y) {
                            write!(f, "#")?;
                            continue;
                        }
                    }
                    write!(f, ".")?;
                }
            }
        }
        Ok(())
    }
}

impl FromStr for Model {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut guard = None;
        let mut heading = None;
        let mut obs_by_x: Vec<Vec<_>> = Vec::new();
        // let mut obs_by_y: HashMap<_, Vec<_>> = HashMap::new();
        let mut max_y = 0;
        let mut max_x = None;
        for (y, line) in s.lines().enumerate() {
            if y == 0 {
                let l = line.len();
                max_x = Some(l - 1);
                obs_by_x.reserve(l);
                for _ in 0..=l {
                    obs_by_x.push(Vec::new());
                }
            }
            for (x, c) in line.chars().enumerate() {
                match c {
                    '^' => {
                        if let Some(g) = guard {
                            eprintln!("Found second guard at ({x}, {y})?! First at {g:?}");
                            return Err("Found second guard?!");
                        } else {
                            guard = Some(Pt { x, y });
                            heading = Some(Dir::from(c))
                        }
                    }
                    '#' => {
                        obs_by_x[x].push(y);
                    }
                    '.' => {}
                    _ => {
                        eprintln!("Unrecognized '{c}' at ({x}. {y})");
                        return Err("Unrecognized character?!");
                    }
                }
            }
            max_y = y;
        }
        if let Some(guard) = guard {
            Ok(Model {
                guard,
                heading: heading.unwrap(),
                bounds: Pt::new(max_x.unwrap(), max_y),
                obs_by_x,
                extra_obs: None,
            })
        } else {
            Err("Didn't find the guard?!")
        }
    }
}

impl Model {
    fn is_obstacle(&self, p: Pt) -> bool {
        if let Some(e) = self.extra_obs {
            if p == e {
                return true;
            }
        }
        self.obs_by_x[p.x].contains(&p.y)
    }

    fn at_edge(&self, p: Pt) -> bool {
        p.y == 0 || p.x == self.bounds.x || p.y == self.bounds.y || p.x == 0
    }

    fn add_obstruction(&mut self, p: Pt) {
        if let Some(e) = self.extra_obs {
            panic!("There's already an extra obstruction at {e}?!");
        } else {
            self.extra_obs = Some(p);
        }
    }

    fn clear_obstruction(&mut self) {
        if let None = self.extra_obs {
            panic!("There's no extra obstruction to clear?!")
        } else {
            self.extra_obs = None;
        }
    }
}

/// If the guard exits, return `Ok` with the set of coordinates she visited. If
/// she entered a cycle, return `Err` with the cycle's entrance coordinates.
fn do_walk(model: &Model) -> Result<HashSet<(Pt, Dir)>, (Pt, Dir)> {
    let mut pos = model.guard;
    let mut h = model.heading;
    let mut visited = HashSet::from([(pos, h)]);
    loop {
        let next = pos.step(h);
        if model.is_obstacle(next) {
            h = h.turn_right();
            continue;
        }
        pos = next;
        if !visited.insert((pos, h)) {
            return Err((pos, h));
        }
        if model.at_edge(pos) {
            break;
        }
    }
    Ok(visited)
}

fn part_one(model: &Model) -> Result<usize, (Pt, Dir)> {
    match do_walk(model) {
        Ok(visited) => {
            let visited: HashSet<_> = visited.iter().map(|(p, _)| *p).collect();
            // println!("Left area at {pos}, headed {h:?}");
            Ok(visited.len())
        }
        Err(coords) => Err(coords),
    }
}

fn part_two(model: &mut Model) -> usize {
    let mut pos = model.guard;
    let mut h = model.heading;
    let mut positions = HashSet::<Pt>::new();
    loop {
        let next = pos.step(h);
        if model.is_obstacle(next) {
            h = h.turn_right();
            continue;
        }
        if !positions.contains(&next) {
            model.add_obstruction(next);
            if let Err(_) = do_walk(&model) {
                positions.insert(next);
            }
            model.clear_obstruction();
        }
        pos = next;
        if model.at_edge(pos) {
            break;
        }
    }
    positions.remove(&model.guard);
    positions.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn example_1() {
        let mut model = EXAMPLE_1.parse().unwrap();
        println!("{model}");
        assert_eq!(r"41", part_one(&model).unwrap().to_string());
        assert_eq!(r"6", part_two(&mut model).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 6, do_solve).unwrap();
    }
}
