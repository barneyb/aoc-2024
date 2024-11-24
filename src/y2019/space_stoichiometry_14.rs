use crate::Part;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use symbol_table::GlobalSymbol;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let cookbook = parse(input);
    tx.send(Part::Parse(cookbook.len().to_string())).unwrap();
    tx.send(Part::A(part_one(&cookbook).to_string())).unwrap();
    tx.send(Part::B(part_two(&cookbook).to_string())).unwrap();
}

const ONE_TRILLION: usize = 1_000_000_000_000;

lazy_static! {
    static ref ORE: GlobalSymbol = "ORE".into();
    static ref FUEL: GlobalSymbol = "FUEL".into();
}

#[derive(Debug, Eq, PartialEq)]
struct Recipe {
    ingredients: HashMap<GlobalSymbol, usize>,
    quantity: usize,
    name: GlobalSymbol,
}

impl Recipe {
    fn new(ingredients: &[(usize, &str)], quantity: usize, name: &str) -> Recipe {
        let mut ing_map = HashMap::with_capacity(ingredients.len());
        for &(q, n) in ingredients {
            ing_map.insert(n.into(), q);
        }
        Recipe {
            ingredients: ing_map,
            quantity,
            name: GlobalSymbol::from(name),
        }
    }
}

impl From<&str> for Recipe {
    fn from(value: &str) -> Self {
        // 17 NVRVD, 3 JNWZP => 8 VPVL
        let mut ingredients = Vec::new();
        let mut output = false;
        let mut q = None;
        for w in value.split(' ') {
            if w == "=>" {
                output = true
            } else if let None = q {
                q = Some(w.parse().unwrap());
            } else if output {
                return Recipe::new(&ingredients, q.unwrap(), w);
            } else {
                let name = w.trim_end_matches(',');
                ingredients.push((q.take().unwrap(), name))
            }
        }
        panic!("...hmm")
    }
}

type Cookbook = HashMap<GlobalSymbol, Recipe>;

fn parse(input: &str) -> Cookbook {
    let mut map = HashMap::new();
    for l in input.lines() {
        let r: Recipe = l.into();
        map.insert(r.name, r);
    }
    map
}

struct Solver<'a> {
    cookbook: &'a Cookbook,
    pool: HashMap<GlobalSymbol, usize>,
}

impl<'a> Solver<'a> {
    fn get_available(&self, name: &GlobalSymbol) -> usize {
        if let Some(n) = self.pool.get(name) {
            *n
        } else {
            0
        }
    }

    fn set_available(&mut self, name: GlobalSymbol, quantity: usize) {
        if quantity == 0 {
            self.pool.remove(&name);
        } else {
            self.pool.insert(name, quantity);
        }
    }

    fn add_available(&mut self, name: GlobalSymbol, quantity: usize) {
        if quantity > 0 {
            self.set_available(name, self.get_available(&name) + quantity);
        }
    }

    fn ore_needed(&mut self, mut quantity: usize, name: GlobalSymbol) -> usize {
        let avail = self.get_available(&name);
        if avail >= quantity {
            self.set_available(name, avail - quantity);
            return 0; // already had enough on hand
        } else if avail > 0 {
            self.set_available(name, 0);
            quantity -= avail;
        }
        let recipe = self
            .cookbook
            .get(&name)
            .expect(&format!("Unknown '{name}' recipe?!"));
        let factor = quantity.div_ceil(recipe.quantity);
        let ore: usize = recipe
            .ingredients
            .iter()
            .map(|(&n, &q)| {
                if n == *ORE {
                    return q * factor;
                }
                self.ore_needed(q * factor, n)
            })
            .sum();
        let waste = factor * recipe.quantity - quantity;
        self.add_available(name, waste);
        ore
    }
}

fn part_one(cookbook: &Cookbook) -> usize {
    ore_for_fuel(cookbook, 1)
}

fn ore_for_fuel(cookbook: &Cookbook, fuel: usize) -> usize {
    Solver {
        cookbook,
        pool: HashMap::new(),
    }
    .ore_needed(fuel, *FUEL)
}

fn part_two(cookbook: &Cookbook) -> usize {
    let for_one = ore_for_fuel(cookbook, 1);
    // Divide for floor. Ceiling is double that. Expand by one each direction.
    let mut lo = ONE_TRILLION / for_one - 1;
    debug_assert!(ore_for_fuel(cookbook, lo + 1) < ONE_TRILLION);
    let mut hi = (lo + 1) * 2 + 1;
    debug_assert!(ore_for_fuel(cookbook, hi) > ONE_TRILLION);
    // binary search!
    while lo < hi - 1 {
        let mid = (lo + hi) / 2;
        let ore = ore_for_fuel(cookbook, mid);
        if ore < ONE_TRILLION {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"#;

    const EXAMPLE_2: &str = r#"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"#;

    lazy_static! {
        static ref COOKBOOK_1: Cookbook = {
            let mut map = HashMap::new();
            for r in [
                Recipe::new(&[(10, "ORE")], 10, "A"),
                Recipe::new(&[(1, "ORE")], 1, "B"),
                Recipe::new(&[(7, "A"), (1, "B")], 1, "C"),
                Recipe::new(&[(7, "A"), (1, "C")], 1, "D"),
                Recipe::new(&[(7, "A"), (1, "D")], 1, "E"),
                Recipe::new(&[(7, "A"), (1, "E")], 1, "FUEL"),
            ] {
                map.insert(r.name, r);
            }
            map
        };
        static ref COOKBOOK_2: Cookbook = {
            let mut map = HashMap::new();
            for r in [
                Recipe::new(&[(9, "ORE")], 2, "A"),
                Recipe::new(&[(8, "ORE")], 3, "B"),
                Recipe::new(&[(7, "ORE")], 5, "C"),
                Recipe::new(&[(3, "A"), (4, "B")], 1, "AB"),
                Recipe::new(&[(5, "B"), (7, "C")], 1, "BC"),
                Recipe::new(&[(4, "C"), (1, "A")], 1, "CA"),
                Recipe::new(&[(2, "AB"), (3, "BC"), (4, "CA")], 1, "FUEL"),
            ] {
                map.insert(r.name, r);
            }
            map
        };
    }

    const EXAMPLE_3: &str = r#"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"#;

    const EXAMPLE_4: &str = r#"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"#;

    const EXAMPLE_5: &str = r#"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"#;

    #[test]
    fn parse_example_1() {
        assert_eq!(*COOKBOOK_1, parse(EXAMPLE_1))
    }

    #[test]
    fn parse_example_2() {
        assert_eq!(*COOKBOOK_2, parse(EXAMPLE_2))
    }

    #[test]
    fn example_1() {
        assert_eq!(r"31", part_one(&*COOKBOOK_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"165", part_one(&*COOKBOOK_2).to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(r"13312", part_one(&parse(EXAMPLE_3)).to_string());
        assert_eq!(r"82892753", part_two(&parse(EXAMPLE_3)).to_string());
    }

    #[test]
    fn example_4() {
        assert_eq!(r"180697", part_one(&parse(EXAMPLE_4)).to_string());
        assert_eq!(r"5586022", part_two(&parse(EXAMPLE_4)).to_string());
    }

    #[test]
    fn example_5() {
        assert_eq!(r"2210736", part_one(&parse(EXAMPLE_5)).to_string());
        assert_eq!(r"460664", part_two(&parse(EXAMPLE_5)).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2019, 14, do_solve).unwrap();
    }
}
