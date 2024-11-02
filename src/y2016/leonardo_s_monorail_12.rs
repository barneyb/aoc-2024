use crate::y2016::leonardo_s_monorail_12::Val::Imm;

#[derive(Debug, Eq, PartialEq)]
enum Val {
    A,
    B,
    C,
    D,
    Imm(isize),
}

impl From<&str> for Val {
    fn from(value: &str) -> Self {
        match value {
            "a" => Val::A,
            "b" => Val::B,
            "c" => Val::C,
            "d" => Val::D,
            n => Imm(n
                .parse()
                .expect(&format!("'{value}' should have been a signed integer"))),
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
    Copy(Val, Val),
    Inc(Val),
    Dec(Val),
    JumpNonZero(Val, Val),
}

impl From<&str> for ISA {
    fn from(input: &str) -> Self {
        use ISA::*;
        let mut tokens = input.split_whitespace();
        match tokens.next() {
            Some("cpy") => Copy(tokens.next().into(), tokens.next().into()),
            Some("inc") => Inc(tokens.next().into()),
            Some("dec") => Dec(tokens.next().into()),
            Some("jnz") => JumpNonZero(tokens.next().into(), tokens.next().into()),
            t => panic!("Unrecognized token: '{t:?}'"),
        }
    }
}

#[derive(Debug, Default)]
struct Cpu {
    ip: usize,
    registers: [isize; 4],
}

impl Cpu {
    fn get(&self, v: &Val) -> isize {
        use Val::*;
        match v {
            &A => self.registers[0],
            &B => self.registers[1],
            &C => self.registers[2],
            &D => self.registers[3],
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
            _ => panic!("Cannot set {tgt:?}"),
        }
    }

    fn execute(&mut self, program: &Vec<ISA>) {
        while self.ip < program.len() {
            match &program[self.ip] {
                ISA::Copy(v, r) => self.set(r, self.get(v)),
                ISA::Inc(r) => self.set(r, self.get(r) + 1),
                ISA::Dec(r) => self.set(r, self.get(r) - 1),
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

pub fn part_one(input: &str) -> isize {
    let program = parse(input);
    let mut cpu = Cpu::default();
    cpu.execute(&program);
    cpu.get(&Val::A)
}

fn parse(input: &str) -> Vec<ISA> {
    input.trim().lines().map(ISA::from).collect()
}

pub fn part_two(input: &str) -> isize {
    let program = parse(input);
    let mut cpu = Cpu::default();
    cpu.set(&Val::C, 1);
    cpu.execute(&program);
    cpu.get(&Val::A)
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    const EXAMPLE_1: &str = r#"cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a"#;

    lazy_static! {
        static ref MODEL_1: Vec<ISA> = vec![
            ISA::Copy(Val::Imm(41), Val::A),
            ISA::Inc(Val::A),
            ISA::Inc(Val::A),
            ISA::Dec(Val::A),
            ISA::JumpNonZero(Val::A, Val::Imm(2)),
            ISA::Dec(Val::A),
        ];
    }

    #[test]
    fn parse_example_1() {
        assert_eq!(*MODEL_1, parse(EXAMPLE_1));
    }

    #[test]
    fn execute_model_1() {
        let mut cpu = Cpu::default();
        cpu.execute(&*MODEL_1);
        assert_eq!(42, cpu.get(&Val::A));
    }

    #[test]
    fn example_1() {
        assert_eq!(42, part_one(EXAMPLE_1));
    }

    #[test]
    fn test_real_input() {
        use crate::{with_input, Part};
        with_input(2016, 12, |input, tx| {
            tx.send(Part::A(Box::new(part_one(input)))).unwrap();
            tx.send(Part::B(Box::new(part_two(input)))).unwrap();
        })
        .unwrap();
    }
}
