use crate::Part;
use std::sync::mpsc::Sender;

#[derive(Debug, Eq, PartialEq)]
enum Val {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    Imm(isize),
}

impl From<&str> for Val {
    fn from(value: &str) -> Self {
        match value {
            "a" => Val::A,
            "b" => Val::B,
            "c" => Val::C,
            "d" => Val::D,
            "e" => Val::E,
            "f" => Val::F,
            "g" => Val::G,
            "h" => Val::H,
            n => Val::Imm(
                n.parse()
                    .expect(&format!("'{value}' should have been a signed integer")),
            ),
        }
    }
}

impl From<Option<&str>> for Val {
    fn from(value: Option<&str>) -> Self {
        value.unwrap().into()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ISA {
    /// set X Y sets register X to the value of Y.
    Set(Val, Val),
    /// sub X Y decreases register X by the value of Y.
    Sub(Val, Val),
    /// mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
    Mult(Val, Val),
    /// jnz X Y jumps with an offset of the value of Y, but only if the value of X is not zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
    JumpNonZero(Val, Val),
}

impl From<&str> for ISA {
    fn from(input: &str) -> Self {
        use ISA::*;
        let mut tokens = input.split_whitespace();
        match tokens.next() {
            Some("set") => Set(tokens.next().into(), tokens.next().into()),
            Some("sub") => Sub(tokens.next().into(), tokens.next().into()),
            Some("mul") => Mult(tokens.next().into(), tokens.next().into()),
            Some("jnz") => JumpNonZero(tokens.next().into(), tokens.next().into()),
            t => panic!("Unrecognized token: '{t:?}'"),
        }
    }
}

#[derive(Debug, Default)]
struct Cpu {
    ip: usize,
    registers: [isize; 8],
    mult_count: usize,
}

impl Cpu {
    fn get(&self, v: &Val) -> isize {
        use Val::*;
        match v {
            &A => self.registers[0],
            &B => self.registers[1],
            &C => self.registers[2],
            &D => self.registers[3],
            &E => self.registers[4],
            &F => self.registers[5],
            &G => self.registers[6],
            &H => self.registers[7],
            &Imm(i) => i,
        }
    }

    fn set(&mut self, tgt: &Val, v: isize) {
        use Val::*;
        match tgt {
            &A => self.registers[0] = v,
            &B => self.registers[1] = v,
            &C => self.registers[2] = v,
            &D => self.registers[3] = v,
            &E => self.registers[4] = v,
            &F => self.registers[5] = v,
            &G => self.registers[6] = v,
            &H => self.registers[7] = v,
            _ => panic!("Cannot set {tgt:?}"),
        }
    }

    fn execute(&mut self, program: &Vec<ISA>) {
        while self.ip < program.len() {
            match &program[self.ip] {
                ISA::Set(r, v) => self.set(r, self.get(v)),
                ISA::Sub(r, v) => self.set(r, self.get(r) - self.get(v)),
                ISA::Mult(r, v) => {
                    self.mult_count += 1;
                    self.set(r, self.get(r) * self.get(v))
                }
                ISA::JumpNonZero(v, d) => {
                    if self.get(v) != 0 {
                        let d = self.get(d);
                        if d < 0 {
                            self.ip -= d.abs() as usize;
                        } else {
                            self.ip += d as usize;
                        }
                        continue;
                    }
                }
            }
            self.ip += 1;
        }
    }
}

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap()
}

fn part_one(input: &str) -> usize {
    let program = parse(input);
    let mut cpu = Cpu::default();
    cpu.execute(&program);
    cpu.mult_count
}

fn parse(input: &str) -> Vec<ISA> {
    input.trim().lines().map(ISA::from).collect()
}

// fn part_two(input: &str) -> usize {
//     input.len()
// }

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = r#"set a 1
set b 2
set c b
sub c a
mul a b"#;

    #[test]
    fn test_part_one() {
        assert_eq!(1, part_one(TEST));
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(12, part_two("adventofcode"));
    // }

    #[test]
    fn test_real_input() {
        crate::with_input(2017, 23, do_solve).unwrap();
    }
}
