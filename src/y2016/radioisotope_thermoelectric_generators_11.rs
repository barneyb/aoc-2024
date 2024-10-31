use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Write;

pub fn part_one(input: &str) -> usize {
    let (elements, initial_state) = parse(input);
    Solver::new(&elements).solve(initial_state)
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
        if let Ok(s) = self.draw(initial_state) {
            println!("{s}");
        }
        let mut queue = VecDeque::from([initial_state]);
        let mut visited = HashSet::new();
        let mut distinct: usize = 0;
        let mut enqueued: usize = 0;
        while let Some(s) = queue.pop_front() {
            enqueued += 1;
            if !visited.insert(s & self.goal) {
                continue;
            }
            distinct += 1;
            if self.is_goal(s) {
                if let Ok(s) = self.draw(s) {
                    println!("{s}");
                }
                println!("Enqueued {enqueued} states");
                println!("Checked {distinct} distinct states");
                return self.step_count(s);
            }
            for n in self.get_neighbors(s) {
                if !visited.contains(&(n & self.goal)) {
                    queue.push_back(n)
                }
            }
        }
        panic!("Failed to find a suitable series of moves.")
    }

    fn step_count(&self, state: usize) -> usize {
        // the goal is all ones, and the step count is immediately left of it.
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
        // For each floor, compute the generators and microchips present. If any
        // generators are present, every microchip must have its generator.
        for f in 0..4 {
            let mut val = state;
            let mut ms: u32 = 0;
            let mut gs: u32 = 0;
            for _ in 0..self.element_count {
                ms <<= 1;
                if val & 0b11 == f {
                    ms += 1;
                }
                val >>= 2;
                gs <<= 1;
                if val & 0b11 == f {
                    gs += 1;
                }
                val >>= 2;
            }
            if gs > 0 && ms & gs != ms {
                return false;
            }
        }
        true
    }

    fn get_neighbors(&self, state: usize) -> Vec<usize> {
        // An iterator, perhaps...?
        let mut result = Vec::new();
        let floor = self.current_floor(state);
        let move_item = |state, i, sign: i8| {
            let delta = 1_usize << i * 2;
            if sign > 0 {
                state + delta
            } else {
                state - delta
            } // move item
        };
        let mut add_next = |next, sign: i8| {
            if self.is_safe(next) {
                let mut next = if sign > 0 {
                    next + self.elevator_inc
                } else {
                    next - self.elevator_inc
                }; // move elevator
                next += self.step_inc; // take step
                result.push(next)
            }
        };
        for i in 0..self.item_count {
            if state >> i * 2 & 0b11 == floor {
                let mut up = None;
                let mut down = None;
                if floor < 0b11 {
                    let n = move_item(state, i, 1);
                    add_next(n, 1);
                    up = Some(n);
                }
                if floor > 0 {
                    let n = move_item(state, i, -1);
                    add_next(n, -1);
                    down = Some(n);
                }
                for j in (i + 1)..self.item_count {
                    if state >> j * 2 & 0b11 == floor {
                        if let Some(n) = up {
                            add_next(move_item(n, j, 1), 1);
                        }
                        if let Some(n) = down {
                            add_next(move_item(n, j, -1), -1);
                        }
                    }
                }
            }
        }
        result
    }

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
    // four bits per element, right-aligned, microchip first
    // two bits for the elevator
    // rest is steps
    // //    steps eE LgLm HgHm
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
            println!(
                "F{floor}: {element} {}",
                if offset == 0 {
                    "microchip"
                } else {
                    "generator"
                }
            );
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

// pub fn part_two(input: &str) -> usize {
//     let (mut elements, initial_state) = parse(input);
//     elements.push('E');
//     elements.push('D');
//     Solver::new(&elements).solve(initial_state)
// }

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
    fn get_neighbors_initial() {
        let s = Solver::new("HL");
        assert_eq!(
            vec![0b00001_01_1000_0101], // only hydrogen microchip is safe to move
            s.get_neighbors(0b00_1000_0100)
        );
    }

    #[test]
    fn get_neighbors_second_step() {
        let s = Solver::new("HL");
        println!("{:020b}", 0b00001_01_1000_0101);
        println!("{:020b}", 2698);
        assert_eq!(
            vec![
                // 0b01_01_1000_0101
                0b00010_00_1000_0100, // move the hydrogen microchip down
                0b00010_10_1000_1010, // move both hydrogen items up
                0b00010_10_1000_1001, // move the hydrogen generator up
            ],
            s.get_neighbors(0b00001_01_1000_0101)
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
        use crate::{with_input, Part};
        with_input(2016, 11, |input, tx| {
            tx.send(Part::A(Box::new(part_one(input)))).unwrap();
            // tx.send(Part::B(Box::new(part_two(input)))).unwrap();
        })
        .unwrap();
    }
}
