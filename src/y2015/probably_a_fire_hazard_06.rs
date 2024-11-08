use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

struct XY(usize, usize);

impl From<&str> for XY {
    fn from(value: &str) -> Self {
        let parts = value
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();
        XY(parts[0], parts[1])
    }
}

impl From<Option<&str>> for XY {
    fn from(value: Option<&str>) -> Self {
        value.expect("Can't parse an XY from None").into()
    }
}

enum Ins {
    On(XY, XY),
    Off(XY, XY),
    Toggle(XY, XY),
}

impl From<&str> for Ins {
    fn from(value: &str) -> Self {
        let mut words = value.split_ascii_whitespace();
        match words.next() {
            Some("toggle") => {
                let a = words.next().into();
                words.next();
                Ins::Toggle(a, words.next().into())
            }
            Some("turn") => match words.next() {
                Some("on") => {
                    let a = words.next().into();
                    words.next();
                    Ins::On(a, words.next().into())
                }
                Some("off") => {
                    let a = words.next().into();
                    words.next();
                    Ins::Off(a, words.next().into())
                }
                os => panic!("Unexpected {os:?}"),
            },
            os => panic!("Unexpected {os:?}"),
        }
    }
}

fn part_one(input: &str) -> usize {
    part_one_array(input).iter().filter(|&&v| v).count()
}

pub fn part_one_array(input: &str) -> Vec<bool> {
    let mut array = vec![false; 1_000_000];
    for ins in input.lines().map(Ins::from) {
        match ins {
            Ins::On(a, b) => {
                for y in a.1..=b.1 {
                    let dy = y * 1000;
                    for x in a.0..=b.0 {
                        array[dy + x] = true;
                    }
                }
            }
            Ins::Off(a, b) => {
                for y in a.1..=b.1 {
                    let dy = y * 1000;
                    for x in a.0..=b.0 {
                        array[dy + x] = false;
                    }
                }
            }
            Ins::Toggle(a, b) => {
                for y in a.1..=b.1 {
                    let dy = y * 1000;
                    for x in a.0..=b.0 {
                        let i = dy + x;
                        array[i] = !array[i];
                    }
                }
            }
        }
    }
    array
}

fn part_two(input: &str) -> u32 {
    part_two_array(input).into_iter().sum()
}

pub fn part_two_array(input: &str) -> Vec<u32> {
    let mut array = vec![0; 1_000_000];
    for ins in input.lines().map(Ins::from) {
        match ins {
            Ins::On(a, b) => {
                for y in a.1..=b.1 {
                    let dy = y * 1000;
                    for x in a.0..=b.0 {
                        array[dy + x] += 1;
                    }
                }
            }
            Ins::Off(a, b) => {
                for y in a.1..=b.1 {
                    let dy = y * 1000;
                    for x in a.0..=b.0 {
                        let idx = dy + x;
                        if array[idx] > 0 {
                            array[idx] -= 1;
                        }
                    }
                }
            }
            Ins::Toggle(a, b) => {
                for y in a.1..=b.1 {
                    let dy = y * 1000;
                    for x in a.0..=b.0 {
                        array[dy + x] += 2;
                    }
                }
            }
        }
    }
    array
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"turn on 0,0 through 999,999"#;

    const EXAMPLE_2: &str = r#"toggle 0,0 through 999,0"#;

    const EXAMPLE_3: &str = r#"turn off 499,499 through 500,500"#;

    const EXAMPLE_4: &str = r#"turn on 0,0 through 0,0"#;

    const EXAMPLE_5: &str = r#"toggle 0,0 through 999,999"#;

    #[test]
    fn example_1() {
        assert_eq!(r"1000000", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"1000", part_one(EXAMPLE_2).to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(r"0", part_one(EXAMPLE_3).to_string());
    }

    #[test]
    fn example_4() {
        assert_eq!(r"1", part_two(EXAMPLE_4).to_string());
    }

    #[test]
    fn example_5() {
        assert_eq!(r"2000000", part_two(EXAMPLE_5).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2015, 6, do_solve).unwrap();
    }
}
