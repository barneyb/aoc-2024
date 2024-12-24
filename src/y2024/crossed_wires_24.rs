use crate::Part;
use petgraph::algo::toposort;
use petgraph::{Direction, Graph};
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    let mut first_section = true;
    let mut signals = HashMap::new();
    let mut g = Graph::new();
    let mut lookup = HashMap::new();
    let mut ops = HashMap::new();
    for line in input.lines() {
        if line == "" {
            first_section = false;
        } else if first_section {
            assert_eq!(Some(':'), line.chars().nth(3));
            signals.insert(&line[0..3], line.chars().nth(5) == Some('1'));
        } else {
            let words: Vec<_> = line.split_ascii_whitespace().collect();
            lookup
                .entry(words[4])
                .or_insert_with(|| g.add_node(words[4]));
            ops.insert(words[4], words[1]);
            lookup
                .entry(words[0])
                .or_insert_with(|| g.add_node(words[0]));
            g.add_edge(lookup[words[0]], lookup[words[4]], ());
            lookup
                .entry(words[2])
                .or_insert_with(|| g.add_node(words[2]));
            g.add_edge(lookup[words[2]], lookup[words[4]], ());
        }
    }
    let topo = toposort(&g, None).unwrap();
    println!(
        "{:?}",
        topo.iter().map(|nx| g.node_weight(*nx)).collect::<Vec<_>>()
    );
    for nx in topo {
        let w = g.node_weight(nx).unwrap();
        if signals.contains_key(w) {
            continue;
        }
        let sigs: Vec<_> = g
            .neighbors_directed(nx, Direction::Incoming)
            .map(|nx| g.node_weight(nx).unwrap())
            .map(|w| signals[w])
            .collect();
        assert_eq!(2, sigs.len());
        signals.entry(w).or_insert_with(|| match ops[w] {
            "AND" => sigs[0] && sigs[1],
            "OR" => sigs[0] || sigs[1],
            "XOR" => sigs[0] != sigs[1],
            op => panic!("Unknown {op:?} operator?!"),
        });
    }
    let mut zs: Vec<_> = signals
        .keys()
        .filter(|w| w.chars().nth(0).unwrap() == 'z')
        .collect();
    zs.sort();
    zs.reverse();
    println!("zs: {zs:?}");
    let mut num = 0_usize;
    for z in zs.into_iter() {
        print!("wire {z} is {}", signals[z]);
        num <<= 1;
        if signals[z] {
            num += 1;
        }
        println!(": {num}")
    }
    num
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;

    const EXAMPLE_2: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;

    #[test]
    fn example_1() {
        assert_eq!(r"4", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"2024", part_one(EXAMPLE_2).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 24, do_solve).unwrap();
    }
}
