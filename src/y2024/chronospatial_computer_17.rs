use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

#[derive(Debug, Default)]
struct VM {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    ip: usize,
    program: Vec<usize>,
}

impl VM {
    #[allow(dead_code)]
    fn new(reg_a: usize, reg_b: usize, reg_c: usize, program: Vec<usize>) -> VM {
        VM {
            reg_a,
            reg_b,
            reg_c,
            ip: 0,
            program,
        }
    }

    fn run_for(&mut self, a: usize) -> Vec<usize> {
        self.reg_a = a;
        self.reg_b = 0;
        self.reg_c = 0;
        self.ip = 0;
        self.execute()
    }

    fn execute(&mut self) -> Vec<usize> {
        let mut stdout = vec![];
        while let Some(op) = self.next() {
            match op {
                0 => self.reg_a = self._dv(),
                1 => self.reg_b ^= self.literal(),
                2 => self.reg_b = self.combo() % 8,
                3 => {
                    let tgt = self.literal();
                    if self.reg_a != 0 {
                        self.ip = tgt;
                    }
                }
                4 => {
                    let _ = self.literal();
                    self.reg_b ^= self.reg_c;
                }
                5 => stdout.push(self.combo() % 8),
                6 => self.reg_b = self._dv(),
                7 => self.reg_c = self._dv(),
                _ => panic!("Unexpected {op} opcode?!"),
            }
        }
        stdout
    }

    fn _dv(&mut self) -> usize {
        let num = self.reg_a;
        let denom = 2_usize.pow(self.combo() as u32);
        num / denom
    }

    fn literal(&mut self) -> usize {
        self.next().unwrap()
    }

    fn combo(&mut self) -> usize {
        let v = self.literal();
        match v {
            0 | 1 | 2 | 3 => v,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            // 7 is reserved
            _ => panic!("Unexpected {v} combo operand?!"),
        }
    }

    fn next(&mut self) -> Option<usize> {
        if self.ip < self.program.len() {
            let v = self.program[self.ip];
            self.ip += 1;
            Some(v)
        } else {
            None
        }
    }
}

fn initialize(input: &str) -> VM {
    let mut vm = VM::default();
    for (i, line) in input.lines().enumerate() {
        if let Some(idx) = line.chars().position(|c| c == ':') {
            match i {
                0 => vm.reg_a = line[idx + 2..].parse().unwrap(),
                1 => vm.reg_b = line[idx + 2..].parse().unwrap(),
                2 => vm.reg_c = line[idx + 2..].parse().unwrap(),
                4 => vm.program.extend(
                    line[idx + 2..]
                        .split(',')
                        .map(|c| c.parse::<usize>().unwrap()),
                ),
                _ => {}
            }
        }
    }
    vm
}

fn part_one(input: &str) -> String {
    initialize(input)
        .execute()
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part_two(input: &str) -> usize {
    let mut vm = initialize(input);
    let prog = vm.program.clone();
    let mut this_generation = vec![0];
    // Deliberately go one extra - it'll never get there if correct, but can
    // provide debugging info if it does.
    for g in 0..=prog.len() {
        let mut next_gen = Vec::new();
        for &prev in this_generation.iter() {
            for offset in 0..8 {
                let a = prev * 8 + offset;
                let out = vm.run_for(a);
                if prog == out {
                    return a;
                }
                if prog.ends_with(&out) && out.len() > g {
                    next_gen.push(a);
                }
            }
        }
        this_generation = next_gen;
    }
    panic!("didn't find answer?!")
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    const EXAMPLE_2: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"#;

    #[test]
    fn example_1() {
        assert_eq!(r"4,6,3,5,6,3,5,2,1,0", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_0_1() {
        let mut vm = VM::new(0, 0, 9, vec![2, 6]);
        vm.execute();
        assert_eq!(1, vm.reg_b);
    }

    #[test]
    fn example_0_2() {
        let mut vm = VM::new(10, 0, 0, vec![5, 0, 5, 1, 5, 4]);
        assert_eq!(vec![0, 1, 2], vm.execute());
    }

    #[test]
    fn example_0_3() {
        let mut vm = VM::new(2024, 0, 0, vec![0, 1, 5, 4, 3, 0]);
        assert_eq!(vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0], vm.execute());
        assert_eq!(0, vm.reg_a);
    }

    #[test]
    fn example_0_4() {
        let mut vm = VM::new(0, 29, 0, vec![1, 7]);
        vm.execute();
        assert_eq!(26, vm.reg_b);
    }

    #[test]
    fn example_0_5() {
        let mut vm = VM::new(0, 2024, 43690, vec![4, 0]);
        vm.execute();
        assert_eq!(44354, vm.reg_b);
    }

    #[test]
    fn example_2a() {
        let mut vm = VM::new(117440, 0, 0, vec![0, 3, 5, 4, 3, 0]);
        assert_eq!(vec![0, 3, 5, 4, 3, 0], vm.execute());
    }

    #[test]
    fn example_2b() {
        assert_eq!(117440, part_two(EXAMPLE_2));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 17, do_solve).unwrap();
    }
}
