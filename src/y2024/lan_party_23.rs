use crate::hist::Histogram;
use crate::Part;
use petgraph::graph::UnGraph;
use petgraph::{Graph, Undirected};
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let net = build_network(input);
    tx.send(Part::A(part_one(&net).to_string())).unwrap();
    tx.send(Part::B(part_two(&net))).unwrap();
}

fn build_network(input: &str) -> Graph<&str, (), Undirected> {
    let mut st = HashMap::new();
    let mut g: Graph<&str, (), Undirected> = UnGraph::new_undirected();
    for l in input.lines() {
        let u = &l[..2];
        let u = *st.entry(u).or_insert_with(|| g.add_node(u));
        let v = &l[3..];
        let v = *st.entry(v).or_insert_with(|| g.add_node(v));
        g.add_edge(u, v, ());
    }
    g
}

fn part_one(net: &Graph<&str, (), Undirected>) -> usize {
    let mut triples = HashSet::new();
    for curr in net.node_indices() {
        for a in net.neighbors(curr) {
            for b in net.neighbors(a) {
                for c in net.neighbors(b) {
                    if c == curr {
                        let mut t = vec![a, b, c];
                        t.sort();
                        triples.insert(t);
                        break;
                    }
                }
            }
        }
    }
    triples
        .into_iter()
        .filter(|t| {
            t.iter()
                .any(|c| net.node_weight(*c).unwrap().starts_with('t'))
        })
        .count()
}

fn part_two(net: &Graph<&str, (), Undirected>) -> String {
    for curr in net.node_indices() {
        let mut hist = Histogram::new();
        for a in net.neighbors(curr) {
            hist.increment(a);
            for b in net.neighbors(a) {
                hist.increment(b)
            }
        }
        let mut sets: HashMap<usize, Vec<_>> = HashMap::new();
        for (nx, n) in hist {
            sets.entry(n)
                .or_default()
                .push(net.node_weight(nx).unwrap());
        }
        for (n, lbls) in sets {
            if lbls.len() == n {
                let mut to_sort: Vec<_> = lbls.iter().map(|l| **l).collect();
                to_sort.push(net.node_weight(curr).unwrap());
                to_sort.sort();
                let mut buf = String::new();
                for l in to_sort {
                    if !buf.is_empty() {
                        buf.push(',');
                    }
                    buf.push_str(l);
                }
                return buf;
            }
        }
    }
    panic!("didn't find any LAN party?!")
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

    #[test]
    fn example_1() {
        let net = build_network(EXAMPLE_1);
        assert_eq!(r"7", part_one(&net).to_string());
        assert_eq!(r"co,de,ka,ta", part_two(&net).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 23, do_solve).unwrap();
    }
}
