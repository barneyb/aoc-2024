use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

type Program = Vec<usize>;

fn part_one(input: &str) -> usize {
    let mut program = parse(input);
    program[1] = 12;
    program[2] = 2;
    run_for(program, 0)
}

fn parse(input: &str) -> Program {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

fn run_for(mut program: Program, idx: usize) -> usize {
    let mut ip = 0;
    loop {
        match program[ip] {
            op @ (1 | 2) => {
                let a = program[program[ip + 1]];
                let b = program[program[ip + 2]];
                let d = program[ip + 3];
                program[d] = match op {
                    1 => a + b,
                    2 => a * b,
                    op => panic!("Found code {op} at {ip}?!"),
                };
                ip += 4;
            }
            99 => return program[idx],
            op => panic!("Found code {op} at {ip}?!"),
        }
    }
}

fn part_two(input: &str) -> usize {
    let mut program = parse(input);
    for n in 0..100 {
        for v in 0..100 {
            program[1] = n;
            program[2] = v;
            if run_for(program.clone(), 0) == 19690720 {
                return n * 100 + v;
            }
        }
    }
    panic!("No params gave the right result?!")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(vec![1, 0, 0, 0, 99], parse("1,0,0,0,99"));
    }

    #[test]
    fn test_run_for() {
        assert_eq!(2, run_for(parse("1,0,0,0,99"), 0));
        assert_eq!(30, run_for(parse("1,1,1,4,99,5,6,0,99"), 0));
        assert_eq!(6, run_for(parse("2,3,0,3,99"), 3));
        assert_eq!(9801, run_for(parse("2,4,4,5,99,0"), 5));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2019, 2, do_solve).unwrap();
    }
}
