use crate::Part;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use Attr::*;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let sues: Vec<_> = input.lines().map(Sue::from).collect();
    tx.send(Part::Parse(sues.len().to_string())).unwrap();
    tx.send(Part::A(part_one(&sues).to_string())).unwrap();
    tx.send(Part::B(part_two(&sues).to_string())).unwrap();
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Attr {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl From<&str> for Attr {
    fn from(value: &str) -> Self {
        match value {
            "children" => Children,
            "cats" => Cats,
            "samoyeds" => Samoyeds,
            "pomeranians" => Pomeranians,
            "akitas" => Akitas,
            "vizslas" => Vizslas,
            "goldfish" => Goldfish,
            "trees" => Trees,
            "cars" => Cars,
            "perfumes" => Perfumes,
            s => panic!("Unknown '{s}' attribute"),
        }
    }
}

lazy_static! {
    static ref DETECTED: HashMap<Attr, u8> = HashMap::from([
        (Children, 3),
        (Cats, 7),
        (Samoyeds, 2),
        (Pomeranians, 3),
        (Akitas, 0),
        (Vizslas, 0),
        (Goldfish, 5),
        (Trees, 3),
        (Cars, 2),
        (Perfumes, 1),
    ]);
}

#[derive(Debug, Eq, PartialEq)]
struct Sue {
    id: u32,
    attrs: HashMap<Attr, u8>,
}

impl From<&str> for Sue {
    fn from(value: &str) -> Self {
        // Sue 16: vizslas: 6, cats: 6, pomeranians: 10
        let colon = value.find(':').unwrap();
        let id = value[4..colon].parse().unwrap();
        let mut attrs = HashMap::new();
        for p in value[(colon + 1)..].split(',') {
            let colon = p.find(':').unwrap();
            attrs.insert(p[1..colon].into(), p[(colon + 2)..].parse().unwrap());
        }
        Sue { id, attrs }
    }
}

fn part_one(sues: &Vec<Sue>) -> u32 {
    for sue in sues {
        if sue
            .attrs
            .iter()
            .all(|(a, &n)| *DETECTED.get(a).unwrap() == n)
        {
            return sue.id;
        }
    }
    panic!("Failed to find a matching Aunt Sue?!")
}

fn part_two(sues: &Vec<Sue>) -> u32 {
    for sue in sues {
        if sue.attrs.iter().all(|(a, &n)| {
            let detected = *DETECTED.get(a).unwrap();
            match a {
                Cats | Trees => detected < n,
                Pomeranians | Goldfish => detected > n,
                _ => detected == n,
            }
        }) {
            return sue.id;
        }
    }
    panic!("Failed to find a matching Aunt Sue?!")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Sue {
                id: 384,
                attrs: HashMap::from([(Children, 6), (Goldfish, 10), (Trees, 1)])
            },
            "Sue 384: children: 6, goldfish: 10, trees: 1".into()
        )
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2015, 16, do_solve).unwrap();
    }
}
