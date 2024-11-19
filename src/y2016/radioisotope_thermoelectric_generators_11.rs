use crate::Part;
use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Write;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let (elements, initial_state) = parse(input);
    tx.send(Part::A(
        Solver::new(&elements).solve(initial_state).to_string(),
    ))
    .unwrap();
    let mut elements = elements;
    elements.push('E');
    elements.push('D');
    tx.send(Part::B(
        Solver::new(&elements).solve(initial_state).to_string(),
    ))
    .unwrap();
}

struct Solver {
    elements: Vec<char>,
    item_count: usize,
    element_count: usize,
    goal: usize,
    bit_size: u8,
    elevator_inc: usize,
    step_inc: usize,
}

impl Solver {
    fn new(elements: &str) -> Solver {
        let element_count = elements.len();
        // two bits for the elevator, four per element
        let goal = {
            let mut goal: usize = 0b11;
            for _ in 0..element_count {
                goal = goal << 4 | 0b1111;
            }
            goal
        };
        let bit_size = 2 + (element_count * 4) as u8;
        Solver {
            elements: elements.chars().collect(),
            item_count: element_count * 2,
            element_count,
            goal,
            bit_size,
            elevator_inc: 1 << (bit_size - 2),
            step_inc: 1 << bit_size,
        }
    }

    fn solve(&self, initial_state: usize) -> usize {
        let mut queue = VecDeque::from([initial_state]);
        let mut visited = HashSet::new();
        let mut neighbors = Vec::new(); // to avoid re-allocating
        while let Some(s) = queue.pop_front() {
            if !visited.insert(s & self.goal) {
                continue;
            }
            if self.is_goal(s) {
                return self.step_count(s);
            }
            neighbors.clear();
            self.build_neighbors(s + self.step_inc, &mut neighbors);
            for &n in neighbors.iter() {
                queue.push_back(n);
            }
        }
        panic!("Failed to find a suitable series of moves.")
    }

    fn step_count(&self, state: usize) -> usize {
        // the step count is immediately left of the goal
        state >> self.bit_size
    }

    fn current_floor(&self, state: usize) -> usize {
        // elevator is the leftmost two bits of the goal.
        state >> (self.bit_size - 2) & 0b11
    }

    fn is_goal(&self, state: usize) -> bool {
        state & self.goal == self.goal
    }

    fn is_safe(&self, state: usize) -> bool {
        // For each element, record what floor its generator is on, and if
        // it's microchip is detached, what floor it's on.
        let mut val = state;
        let mut loose_chips = [false; 4];
        let mut generators = [false; 4];
        for _ in 0..self.element_count {
            let m = val & 0b11;
            val >>= 2;
            let g = val & 0b11;
            val >>= 2;
            if loose_chips[g] {
                // loose chip w/ this generator
                return false;
            }
            if m == g {
                // same floor; microchip is safe
                generators[g] = true;
            } else if generators[m] {
                // alone w/ another element's generator
                return false;
            } else {
                generators[g] = true;
                loose_chips[m] = true;
            }
        }
        true
    }

    fn build_neighbors(&self, state: usize, buffer: &mut Vec<usize>) {
        let floor = self.current_floor(state);
        let mut add_next = |next| {
            if self.is_safe(next) {
                buffer.push(next)
            }
        };
        let mut first = state;
        for i in 0..self.item_count {
            if first & 0b11 == floor {
                let mut up = None;
                let mut down = None;
                let delta = (1 << i * 2) + self.elevator_inc;
                if floor < 0b11 {
                    let n = state + delta;
                    add_next(n);
                    up = Some(n);
                }
                if floor > 0 {
                    let n = state - delta;
                    add_next(n);
                    down = Some(n);
                }
                let mut second = first;
                for j in (i + 1)..self.item_count {
                    second >>= 2;
                    if second & 0b11 == floor {
                        let delta = 1 << j * 2;
                        if let Some(n) = up {
                            add_next(n + delta);
                        }
                        if let Some(n) = down {
                            add_next(n - delta);
                        }
                    }
                }
            }
            first >>= 2;
        }
    }

    #[allow(dead_code)]
    fn draw(&self, state: usize) -> Result<String> {
        let mut s = Vec::new();
        let mut chips = Vec::new();
        let mut gens = Vec::new();
        let mut val = state;
        for _ in 0..self.element_count {
            chips.push(val & 0b11);
            val >>= 2;
            gens.push(val & 0b11);
            val >>= 2;
        }
        let el = val & 0b11;
        for f in (0..4).rev() {
            write!(s, "F{} {}  ", f + 1, if el == f { 'E' } else { '.' })?;
            for ((e, &g), &mc) in self.elements.iter().zip(gens.iter()).zip(chips.iter()) {
                if g == f {
                    write!(s, "{e}G ")?;
                } else {
                    write!(s, ".  ")?;
                }
                if mc == f {
                    write!(s, "{e}M ")?;
                } else {
                    write!(s, ".  ")?;
                }
            }
            writeln!(s)?;
        }
        let s = String::from_utf8(s)?;
        Ok(s)
    }
}

fn parse(input: &str) -> (String, usize) {
    // four bits per element, right-aligned, microchip on right
    // two bits for the elevator
    // rest is steps
    //       steps eE LgLm HgHm
    // (2, 0b00000_00_1000_0100)
    let mut offsets = HashMap::new();
    let mut elements = String::new();
    let mut state = 0;
    for (floor, line) in input.lines().enumerate() {
        let words = line.split(&[' ', '-', '.', ','][..]).collect::<Vec<_>>();
        for (i, w) in words.iter().enumerate() {
            let (element, offset) = match w {
                &"microchip" => (words[i - 2], 0),
                &"generator" => (words[i - 1], 2),
                _ => continue,
            };
            // the actual value, just-inserted or preexisting
            let i = if let Some(&i) = offsets.get(element) {
                i
            } else {
                // since we're inserting, also add it to the elements string
                let initial = element.chars().next().unwrap().to_ascii_uppercase();
                elements.push(if elements.contains(initial) {
                    initial.to_ascii_lowercase()
                } else {
                    initial
                });
                let next = offsets.len();
                offsets.insert(element, next);
                next
            };
            state += floor << (i * 4 + offset);
        }
    }
    (elements, state)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &'static str = r"The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";
    const MODEL: (&str, usize) = ("HL", 0b00_1000_0100);

    #[test]
    fn test_solve() {
        let (elements, start) = MODEL;
        assert_eq!(11, Solver::new(elements).solve(start));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            //                  eE LgLm HgHm
            ("HL".to_owned(), 0b00_1000_0100),
            parse(EXAMPLE)
        );
    }

    #[test]
    fn test_is_safe() {
        let s = Solver::new("HL");
        assert!(s.is_safe(0b00_0000_0000)); // both chips have gens
        assert!(s.is_safe(0b00_0000_0001)); // chip is alone
        assert!(s.is_safe(0b00_0000_0101)); // both elements are isolated
        assert!(!s.is_safe(0b00_1010_0110)); // HM is w/ LG, w/out HG
    }

    fn get_neighbors(s: &Solver, state: usize) -> Vec<usize> {
        let mut result = Vec::new();
        s.build_neighbors(state, &mut result);
        result
    }

    #[test]
    fn get_neighbors_initial() {
        let s = Solver::new("HL");
        assert_eq!(
            vec![0b01_1000_0101], // only hydrogen microchip is safe to move
            get_neighbors(&s, 0b00_1000_0100)
        );
    }

    #[test]
    fn get_neighbors_second_step() {
        let s = Solver::new("HL");
        assert_eq!(
            vec![
                // 0b01_01_1000_0101
                0b00_1000_0100, // move the hydrogen microchip down
                0b10_1000_1010, // move both hydrogen items up
                0b10_1000_1001, // move the hydrogen generator up
            ],
            get_neighbors(&s, 0b01_1000_0101)
        );
    }

    #[test]
    fn draw() {
        let s = Solver::new("HL");
        assert_eq!(
            "\
F4 .  .  .  .  .  
F3 .  .  .  LG .  
F2 .  HG .  .  .  
F1 E  .  HM .  LM 
",
            s.draw(0b00_1000_0100).unwrap()
        )
    }

    #[test]
    fn test_parse_real_input() {
        use crate::with_input;
        with_input(2016, 11, |input, _| {
            let (el, st) = parse(input);
            println!(
                "{}",
                Solver::new("TPSpR")
                    .draw(0b00_1010_1010_0001_0001_0000)
                    .unwrap()
            );
            println!("0b0010101010000100010000");
            println!("{st:#024b}");
            println!("{}", Solver::new(&el).draw(st).unwrap());
            assert_eq!("TPSpR".to_owned(), el);
            assert_eq!(0b00_1010_1010_0001_0001_0000, st);
        })
        .unwrap();
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2016, 11, do_solve).unwrap();
    }
}
