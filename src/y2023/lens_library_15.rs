use crate::Part;
use std::sync::mpsc::Sender;
use symbol_table::Symbol;
use symbol_table::SymbolTable;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn hash(input: &str) -> usize {
    let mut val = 0;
    for c in input.chars() {
        val += c as usize;
        val *= 17;
        val %= 256;
    }
    val
}

fn part_two(input: &str) -> usize {
    let mut map: Vec<Vec<(Symbol, usize)>> = vec![Vec::new(); 256];
    let symbols = SymbolTable::new();
    for ins in input.split(',') {
        let i = ins.chars().position(|c| c == '-' || c == '=').unwrap();
        let op = ins.chars().nth(i).unwrap();
        let lbl = &ins[0..i];
        let sym = symbols.intern(lbl);
        let bin = map.get_mut(hash(lbl)).unwrap();
        let opt_i = bin.iter().position(|(s, _)| *s == sym);
        if op == '-' {
            if let Some(i) = opt_i {
                bin.remove(i);
            }
        } else {
            let pair = (sym, ins[(i + 1)..].parse().unwrap());
            if let Some(i) = opt_i {
                bin[i] = pair;
            } else {
                bin.push(pair);
            }
        }
    }
    map.iter()
        .enumerate()
        .map(|(bin_n, bin)| {
            bin.iter()
                .enumerate()
                .map(|(slot_n, (_, fl))| (bin_n + 1) * (slot_n + 1) * fl)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn example_1() {
        assert_eq!(r"1320", part_one(EXAMPLE_1).to_string());
        assert_eq!(r"145", part_two(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2023, 15, do_solve).unwrap();
    }
}
