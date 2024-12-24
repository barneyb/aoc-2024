use crate::Part;
use petgraph::algo::toposort;
use petgraph::prelude::NodeIndex;
use petgraph::{Direction, Graph};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::mpsc::Sender;
use symbol_table::{Symbol, SymbolTable};

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

fn swap(input: &str, a: &str, b: &str) -> String {
    const TEMP: &str = "__TEMP__";
    let mut input = input.replace(a, TEMP);
    input = input.replace(b, a);
    input.replace(TEMP, b)
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    And,
    Or,
    Xor,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Op::And),
            "OR" => Ok(Op::Or),
            "XOR" => Ok(Op::Xor),
            _ => Err(()),
        }
    }
}

impl Op {
    fn operate(&self, a: bool, b: bool) -> bool {
        match self {
            Op::And => a && b,
            Op::Or => a || b,
            Op::Xor => a != b,
        }
    }
}

struct Adder {
    symbols: SymbolTable,
    signals: HashMap<Symbol, bool>,
    g: Graph<Symbol, ()>,
    #[allow(dead_code)]
    lookup: HashMap<Symbol, NodeIndex>,
    ops: HashMap<Symbol, Op>,
}

impl FromStr for Adder {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut first_section = true;
        let symbols = SymbolTable::new();
        let mut signals = HashMap::new();
        let mut g = Graph::new();
        let mut lookup = HashMap::new();
        // let mut g2 = Graph::new();
        // let mut lookup2 = HashMap::new();
        let mut ops = HashMap::new();
        for line in input.lines() {
            if line == "" {
                first_section = false;
            } else if first_section {
                assert_eq!(Some(':'), line.chars().nth(3));
                signals.insert(
                    symbols.intern(&line[0..3]),
                    line.chars().nth(5) == Some('1'),
                );
            } else {
                let words: Vec<_> = line.split_ascii_whitespace().collect();
                let a = symbols.intern(words[0]);
                let op = words[1].parse::<Op>()?;
                let b = symbols.intern(words[2]);
                let out = symbols.intern(words[4]);
                lookup.entry(out).or_insert_with(|| g.add_node(out));
                // lookup2
                //     .entry(words[4])
                //     .or_insert_with(|| g2.add_node(words[4].to_string()));
                // *g2.node_weight_mut(lookup2[words[4]]).unwrap() =
                //     words[1].to_string() + "  " + words[4];
                ops.insert(out, op);
                lookup.entry(a).or_insert_with(|| g.add_node(a));
                // lookup2
                //     .entry(words[0])
                //     .or_insert_with(|| g2.add_node(words[0].to_string()));
                g.add_edge(lookup[&a], lookup[&out], ());
                // g2.add_edge(lookup2[words[0]], lookup2[words[4]], ());
                lookup.entry(b).or_insert_with(|| g.add_node(b));
                // lookup2
                //     .entry(words[2])
                //     .or_insert_with(|| g2.add_node(words[2].to_string()));
                g.add_edge(lookup[&b], lookup[&out], ());
                // g2.add_edge(lookup2[words[2]], lookup2[words[4]], ());
            }
        }
        // crate::viz::graphviz::render_dot(&petgraph::dot::Dot::with_config(&g2, &[petgraph::dot::Config::EdgeNoLabel]));
        Ok(Adder {
            symbols,
            signals,
            g,
            lookup,
            ops,
        })
    }
}

impl Adder {
    fn evaluate(&self) -> HashMap<Symbol, bool> {
        let mut signals = self.signals.clone();
        let topo = toposort(&self.g, None).unwrap();
        for nx in topo {
            let w = self.g.node_weight(nx).unwrap();
            if signals.contains_key(w) {
                continue;
            }
            let symbols: Vec<_> = self
                .g
                .neighbors_directed(nx, Direction::Incoming)
                .map(|nx| self.g.node_weight(nx).unwrap())
                .collect();
            let sigs: Vec<_> = symbols.iter().map(|w| signals[w]).collect();
            // println!(
            //     "  {} {:?} {} -> {}  |  {} {:?} {} = {}",
            //     self.symbols.resolve(*symbols[0]),
            //     self.ops[w],
            //     self.symbols.resolve(*symbols[1]),
            //     self.symbols.resolve(*w),
            //     sigs[0],
            //     self.ops[w],
            //     sigs[1],
            //     self.ops[w].operate(sigs[0], sigs[1])
            // );
            signals.insert(*w, self.ops[w].operate(sigs[0], sigs[1]));
        }
        signals
    }

    fn set(&mut self, prefix: char, mut value: usize) {
        for n in 0..45 {
            self.signals.insert(
                self.symbols.intern(&format!("{prefix}{n:0>2}")),
                value & 1 == 1,
            );
            value >>= 1;
        }
    }

    fn get_from(&self, prefix: char, signals: &HashMap<Symbol, bool>) -> usize {
        let mut ws: Vec<_> = signals
            .keys()
            .map(|s| self.symbols.resolve(*s))
            .filter(|w| w.chars().nth(0).unwrap() == prefix)
            .collect();
        ws.sort();
        ws.reverse();
        let mut num = 0_usize;
        for z in ws.into_iter() {
            let s = self.symbols.intern(z);
            num <<= 1;
            if signals[&s] {
                num += 1;
            }
        }
        num
    }
}

fn part_one(input: &str) -> usize {
    let adder: Adder = input.parse().unwrap();
    let signals = adder.evaluate();
    adder.get_from('z', &signals)
}

fn part_two(input: &str) -> String {
    // my swaps, found by visual inspection of the "circuit"
    let to_swap: Vec<(&str, &str)> = vec![
        ("kfp", "hbs"),
        ("dhq", "z18"),
        ("z22", "pdg"),
        ("jcp", "z27"),
    ];
    let mut input = input.to_string();
    let mut wires = Vec::new();
    for (a, b) in to_swap {
        input = swap(&input, &format!(" -> {a}"), &format!(" -> {b}"));
        wires.push(a);
        wires.push(b);
    }
    let mut adder: Adder = input.parse().unwrap();
    for i in 0..45 {
        let x = 1 << i;
        let y = 1 << i;
        adder.set('x', x);
        adder.set('y', y);
        let z = adder.get_from('z', &adder.evaluate());
        assert_eq!(
            z,
            x + y,
            "At bit {i}:\n  {x:>45b}\n+ {y:>45b}\n{}\n {z:>46b} <-- incorrect\n {:>46b} <-- correct",
            "-".repeat(47),
            x + y
        );
    }
    wires.sort();
    let mut buf = String::new();
    for w in wires {
        if !buf.is_empty() {
            buf.push(',')
        }
        buf.push_str(w);
    }
    buf
}

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

    const EXAMPLE_3: &str = r#"x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00"#;

    #[test]
    fn example_1() {
        assert_eq!(r"4", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"2024", part_one(EXAMPLE_2).to_string());
    }

    impl Adder {
        fn get(&self, prefix: char) -> usize {
            self.get_from(prefix, &self.signals)
        }
    }

    #[test]
    fn example_3_broken() {
        let adder: Adder = EXAMPLE_3.parse().unwrap();
        assert_eq!(42, adder.get('x'));
        assert_eq!(44, adder.get('y'));
        assert_eq!(9, adder.get_from('z', &adder.evaluate()));
    }

    #[test]
    fn example_3_fixed() {
        let input = swap(EXAMPLE_3, "-> z05", "-> z00");
        let input = swap(&input, "-> z01", "-> z02");
        let adder: Adder = input.parse().unwrap();
        assert_eq!(42, adder.get('x'));
        assert_eq!(44, adder.get('y'));
        assert_eq!(40, adder.get_from('z', &adder.evaluate()));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 24, do_solve).unwrap();
    }
}
