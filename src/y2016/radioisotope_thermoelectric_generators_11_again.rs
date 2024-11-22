use crate::Part;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::sync::mpsc::Sender;

/*
This runs in about 100ms and 8 sec, compared to the usize based one which runs
in about 40ms and 2.5 sec.
 */
pub fn do_solve(_input: &str, tx: Sender<Part>) {
    tx.send(Part::A(either_part(parse()).to_string())).unwrap();
    let mut model = parse();
    model
        .get_mut(&1)
        .unwrap()
        .extend([('E', 'g'), ('E', 'm'), ('D', 'g'), ('D', 'm')]);
    tx.send(Part::B(either_part(model).to_string())).unwrap();
}

type Model = HashMap<u8, Vec<(char, char)>>;

fn parse() -> Model {
    HashMap::from([
        (
            1,
            Vec::from([('T', 'g'), ('T', 'm'), ('P', 'g'), ('S', 'g')]),
        ),
        (2, Vec::from([('P', 'm'), ('S', 'm')])),
        (
            3,
            Vec::from([('p', 'g'), ('p', 'm'), ('R', 'g'), ('R', 'm')]),
        ),
        (4, Vec::new()),
    ])
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct World {
    steps: usize,
    elevator: u8,
    items: Vec<u8>,
}

impl World {
    pub(crate) fn get_id(&self) -> usize {
        let mut id = self.elevator as usize;
        for it in self.items.iter() {
            id <<= 2;
            id += *it as usize;
        }
        id
    }
}

fn either_part(model: Model) -> usize {
    let mut queue = VecDeque::from([World {
        steps: 0,
        elevator: 1,
        items: flatten(model),
    }]);
    let mut visited = HashSet::new();
    loop {
        let world = queue.pop_front().unwrap();
        if !visited.insert(world.get_id()) {
            continue;
        }
        if is_complete(&world.items) {
            return world.steps;
        }
        let on_this_floor: Vec<_> = world
            .items
            .iter()
            .enumerate()
            .filter(|(_, f)| **f == world.elevator)
            .map(|(i, _)| i)
            .collect();
        let steps = world.steps + 1;
        for to_floor in neighboring_floors(world.elevator) {
            let mut enqueue = |items: &Vec<_>| {
                if is_safe(items, world.elevator) && is_safe(items, to_floor) {
                    queue.push_back(World {
                        steps,
                        elevator: to_floor,
                        items: items.clone(),
                    })
                }
            };
            for (i, &idx) in on_this_floor.iter().enumerate() {
                let mut items = world.items.clone();
                items[idx] = to_floor;
                enqueue(&items);
                for &idx in on_this_floor[(i + 1)..].iter() {
                    let mut items = items.clone();
                    items[idx] = to_floor;
                    enqueue(&items);
                }
            }
        }
    }
}

fn is_safe(items: &Vec<u8>, floor: u8) -> bool {
    let mut loose_microchip = false;
    let mut any_generator = false;
    for (i, _) in items.iter().enumerate().step_by(2) {
        if items[i] == floor {
            // generator
            if loose_microchip {
                return false;
            }
            any_generator = true;
        } else if items[i + 1] == floor {
            // microchip without generator
            if any_generator {
                return false;
            }
            loose_microchip = true;
        }
    }
    true
}

fn neighboring_floors(f: u8) -> Vec<u8> {
    match f {
        1 => Vec::from([2]),
        2 => Vec::from([1, 3]),
        3 => Vec::from([2, 4]),
        4 => Vec::from([3]),
        _ => panic!("There isn't a floor {f}?!"),
    }
}

fn is_complete(items: &Vec<u8>) -> bool {
    items.iter().all(|it| *it == 4)
}

/// take a floor-based model, and convert to a Vec of floors. Generators have
/// even indexes, and each one's corresponding microchip is the following odd.
fn flatten(model: Model) -> Vec<u8> {
    let mut elements = HashMap::new();
    let mut result = Vec::new();
    for (f, items) in model {
        for (e, t) in items {
            let offset = if let Some(i) = elements.get(&e) {
                *i
            } else {
                let i = elements.len();
                elements.insert(e, i.clone());
                i
            };
            let idx = offset * 2 + if t == 'm' { 1 } else { 0 };
            if idx >= result.len() {
                result.resize(idx + 1, 0);
            }
            result[idx] = f;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn example_1() {
        assert_eq!(
            r"11",
            either_part(HashMap::from([
                (1, Vec::from([('H', 'm'), ('L', 'm')])),
                (2, Vec::from([('H', 'g')])),
                (3, Vec::from([('L', 'g')])),
                (4, Vec::new()),
            ]))
            .to_string()
        );
    }

    #[ignore]
    #[test]
    fn test_real_input() {
        crate::with_input(2016, 11, do_solve).unwrap();
    }
}
