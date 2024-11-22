use crate::Part;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

lazy_static! {
    static ref DETECTED: HashMap<&'static str, u8> = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);
}

#[derive(Debug, Eq, PartialEq)]
struct Sue {
    id: u32,
    attrs: HashMap<String, u8>,
}

impl From<&str> for Sue {
    fn from(value: &str) -> Self {
        // Sue 16: vizslas: 6, cats: 6, pomeranians: 10
        let colon = value.find(':').unwrap();
        let id = value[4..colon].parse().unwrap();
        let mut attrs = HashMap::new();
        for p in value[(colon + 1)..].split(',') {
            let colon = p.find(':').unwrap();
            attrs.insert(p[1..colon].to_string(), p[(colon + 2)..].parse().unwrap());
        }
        Sue { id, attrs }
    }
}

fn part_one(input: &str) -> u32 {
    for sue in input.lines().map(Sue::from) {
        if sue
            .attrs
            .iter()
            .all(|(a, &n)| *DETECTED.get(&a[..]).unwrap() == n)
        {
            return sue.id;
        }
    }
    panic!("Failed to find a matching Aunt Sue?!")
}

// fn part_two(input: &str) -> usize {
//     input.len()
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Sue {
                id: 384,
                attrs: HashMap::from([
                    ("children".to_string(), 6),
                    ("goldfish".to_string(), 10),
                    ("trees".to_string(), 1)
                ])
            },
            "Sue 384: children: 6, goldfish: 10, trees: 1".into()
        )
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2015, 16, do_solve).unwrap();
    }
}
