use crate::geom2d::Dir;
use crate::Part;
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
    /// max x,y values of the area. Min are 0,0
    bounds: Pt,
    /// bounds.x + 1
    width: usize,
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
                    write!(f, "{}", Dir::North)?;
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
            let max_x = max_x.unwrap();
            Ok(Model {
                guard,
                bounds: Pt::new(max_x, max_y),
                width: max_x + 1,
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

    fn tile_count(&self) -> usize {
        self.width * (self.bounds.y + 1)
    }

    fn to_i(&self, p: &Pt) -> usize {
        p.x + p.y * self.width
    }
}

/// If the guard exits, return `Ok` with the set of coordinates she visited. If
/// she entered a cycle, return `Err` with the cycle's entrance coordinates.
fn do_walk(model: &Model) -> Result<usize, (Pt, Dir)> {
    let mut pos = model.guard;
    let mut h = Dir::North;
    let mut visited = vec![0_u8; model.tile_count()];
    visited[model.to_i(&model.guard)] = 1;
    loop {
        let next = pos.step(h);
        if model.is_obstacle(next) {
            h = h.turn_right();
            continue;
        }
        pos = next;
        let i = model.to_i(&pos);
        let mask = match h {
            Dir::North => 1,
            Dir::East => 2,
            Dir::South => 4,
            Dir::West => 8,
        };
        if visited[i] & mask == 0 {
            visited[i] |= mask;
        } else {
            // already been here
            return Err((pos, h));
        }
        if model.at_edge(pos) {
            break;
        }
    }
    Ok(visited.iter().filter(|v| **v != 0).count())
}

fn part_one(model: &Model) -> Result<usize, (Pt, Dir)> {
    match do_walk(model) {
        Ok(visited) => Ok(visited),
        Err(coords) => Err(coords),
    }
}

fn part_two(model: &mut Model) -> usize {
    let mut pos = model.guard;
    let mut h = Dir::North;
    let mut positions = vec![false; model.tile_count()];
    loop {
        let next = pos.step(h);
        if model.is_obstacle(next) {
            h = h.turn_right();
            continue;
        }
        let i = model.to_i(&next);
        if !positions[i] {
            model.add_obstruction(next);
            if let Err(_) = do_walk(&model) {
                positions[i] = true;
            }
            model.clear_obstruction();
        }
        pos = next;
        if model.at_edge(pos) {
            break;
        }
    }
    positions[model.to_i(&model.guard)] = false;
    positions.iter().filter(|v| **v).count()
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
