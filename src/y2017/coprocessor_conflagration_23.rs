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
        self.ip = 0;
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
    // This assumes register f is always used as the 'is composite' flag, and is
    // initialized immediately after setting up b and c (based on a's value).
    let prologue = parse(input)
        .into_iter()
        .take_while(|ins| {
            if let ISA::Set(Val::F, _) = ins {
                false
            } else {
                true
            }
        })
        .collect();
    tx.send(Part::A(part_one(&prologue).to_string())).unwrap();
    tx.send(Part::B(part_two(&prologue).to_string())).unwrap();
}

// b minus two, squared
fn part_one(prologue: &Vec<ISA>) -> isize {
    let mut cpu = Cpu::default();
    cpu.execute(&prologue);
    cpu.execute(&parse(
        r"
        sub b 2
        mul b b",
    ));
    cpu.get(&Val::B)
}

// Of every seventeenth number between b and c, inclusive, how many are composite?
fn part_two(prologue: &Vec<ISA>) -> usize {
    let mut cpu = Cpu::default();
    cpu.set(&Val::A, 1);
    cpu.execute(&prologue);
    let b = cpu.get(&Val::B);
    let c = cpu.get(&Val::C);
    (b..=c)
        .step_by(17)
        .filter(|&num| {
            for factor in 2..((num as f64).sqrt() as isize) {
                if num % factor == 0 {
                    return true;
                }
            }
            false
        })
        .count()
}

fn parse(input: &str) -> Vec<ISA> {
    input.trim().lines().map(ISA::from).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    // ¡¡hard-coded from my github input!!
    #[allow(dead_code)]
    fn execute(a: isize) -> (isize, isize) {
        let b: isize = 57;
        let mut h = 0;
        let (b, c) = if a == 0 {
            (b, b)
        } else {
            let b = b * 100 + 100_000;
            (b, b + 17_000)
        };
        for b in (b..=c).step_by(17) {
            for f in 2..((b as f64).sqrt() as isize) {
                if b % f == 0 {
                    h += 1;
                    break;
                }
            }
        }
        ((b - 2).pow(2), h)
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2017, 23, do_solve).unwrap();
    }
}
