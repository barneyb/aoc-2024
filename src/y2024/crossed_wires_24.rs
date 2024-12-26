use crate::Part;
use petgraph::algo::toposort;
use petgraph::prelude::NodeIndex;
use petgraph::{Graph, Incoming};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::sync::mpsc::Sender;
use symbol_table::{Symbol, SymbolTable};

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
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

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Op::And => "AND",
            Op::Or => "OR",
            Op::Xor => "XOR",
        })
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
                ops.insert(out, op);
                lookup.entry(a).or_insert_with(|| g.add_node(a));
                g.add_edge(lookup[&a], lookup[&out], ());
                lookup.entry(b).or_insert_with(|| g.add_node(b));
                g.add_edge(lookup[&b], lookup[&out], ());
            }
        }
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
                .neighbors_directed(nx, Incoming)
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

    fn find_bad_wires(&self) -> Vec<&str> {
        let mut wires = Vec::new();
        for ix in self.g.node_indices() {
            let sym = self.g[ix];
            if let Some(op) = self.ops.get(&sym) {
                let wire = self.symbols.resolve(sym);
                if wire.starts_with('z') {
                    // z wires MUST be XOR, except the final carry/overflow bit
                    if wire != "z45" && *op != Op::Xor {
                        wires.push(wire);
                    }
                    continue;
                }
                if *op == Op::And {
                    // AND signals pass to one gate
                    if self.g.neighbors(ix).count() != 1 {
                        // unless it's the first half-adder
                        if self
                            .g
                            .neighbors_directed(ix, Incoming)
                            .map(|ix| self.g[ix])
                            .map(|s| self.symbols.resolve(s))
                            .all(|w| !w.ends_with("00"))
                        {
                            wires.push(wire);
                            continue;
                        }
                    }
                } else {
                    // OR/XOR signals pass to two gates
                    if self.g.neighbors(ix).count() != 2 {
                        // unless it's the final overflow bit
                        if wire != "z45" {
                            wires.push(wire);
                            continue;
                        }
                    }
                }
                if *op == Op::Xor {
                    // non-z XOR are always fed by input wires
                    if self
                        .g
                        .neighbors_directed(ix, Incoming)
                        .map(|ix| self.g[ix])
                        .map(|s| self.symbols.resolve(s))
                        .any(|w| !w.starts_with("x") && !w.starts_with("y"))
                    {
                        wires.push(wire);
                        continue;
                    }
                }
            }
        }
        wires
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
        for w in ws.into_iter() {
            num <<= 1;
            if signals[&self.symbols.intern(w)] {
                num += 1;
            }
        }
        num
    }

    #[allow(dead_code)]
    fn render_pdf(&self) {
        let mut g2 = Graph::new();
        for ax in self.g.node_indices() {
            let s = self.g[ax];
            let mut lbl = self.symbols.resolve(s).to_string();
            if let Some(op) = self.ops.get(&s) {
                lbl.push(' ');
                lbl.push_str(&op.to_string())
            }
            let nx = g2.add_node(lbl);
            assert_eq!(ax, nx);
        }
        for ax in self.g.node_indices() {
            for bx in self.g.neighbors(ax) {
                g2.add_edge(ax, bx, ());
            }
        }
        crate::viz::graphviz::render_dot(&petgraph::dot::Dot::with_config(
            &g2,
            &[petgraph::dot::Config::EdgeNoLabel],
        ));
    }
}

fn part_one(input: &str) -> usize {
    let adder: Adder = input.parse().unwrap();
    let signals = adder.evaluate();
    adder.get_from('z', &signals)
}

fn part_two(input: &str) -> String {
    let adder: Adder = input.parse().unwrap();
    // adder.render_pdf();
    let mut wires: Vec<_> = adder.find_bad_wires();
    wires.sort();
    wires.join(",")
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
    fn test_real_input() {
        crate::with_input(2024, 24, do_solve).unwrap();
    }
}
