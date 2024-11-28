use crate::timing::Timing;
use crate::Part;
use regex::{Regex, RegexSet};
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // On average, 3.432 diff digits, 4.22 total digits per line
    // With these tiny REs, the set is slower than probing all ten each line.
    tx.send(Part::B(part_two_set(input).to_string())).unwrap();
    tx.send(Part::B(part_two_separate(input).to_string()))
        .unwrap();
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|text| {
            let digits: Vec<_> = text
                .chars()
                .filter(char::is_ascii_digit)
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();
            digits[0] * 10 + digits.last().unwrap()
        })
        .sum()
}

fn part_two_set(input: &str) -> usize {
    let literals = [
        "0", // no "zero"!
        "1|one", "2|two", "3|three", "4|four", "5|five", "6|six", "7|seven", "8|eight", "9|nine",
    ];
    // one pass to say _which_ matched
    let re_set = RegexSet::new(literals).unwrap();
    // run those to find _where_ the matches are
    let arr = literals.map(Regex::new).map(Result::unwrap);
    Timing::ad_hoc("set", || {
        input
            .lines()
            .map(|text| {
                let mut matches: Vec<_> = re_set
                    .matches(text)
                    .iter()
                    .map(|i| {
                        let mut ms = arr[i].find_iter(text);
                        let first = ms.next().unwrap();
                        let last = ms.last().unwrap_or(first);
                        (i, first.start(), last.start() + last.len())
                    })
                    .collect();
                matches.sort_by_key(|m| m.1);
                let first = matches[0].0;
                matches.sort_by_key(|m| m.2);
                let last = matches.last().unwrap().0;
                first * 10 + last
            })
            .sum()
    })
}

fn part_two_separate(input: &str) -> usize {
    let literals = [
        "0", // no "zero"!
        "1|one", "2|two", "3|three", "4|four", "5|five", "6|six", "7|seven", "8|eight", "9|nine",
    ];
    let arr = literals.map(Regex::new).map(Result::unwrap);
    Timing::ad_hoc("separate", || {
        input
            .lines()
            .map(|text| {
                let mut matches: Vec<_> = arr
                    .iter()
                    .enumerate()
                    .map(|(i, re)| {
                        let mut ms = re.find_iter(text);
                        if let Some(first) = ms.next() {
                            let last = ms.last().unwrap_or(first);
                            Some((i, first.start(), last.start() + last.len()))
                        } else {
                            None
                        }
                    })
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .collect();
                matches.sort_by_key(|m| m.1);
                let first = matches[0].0;
                matches.sort_by_key(|m| m.2);
                let last = matches.last().unwrap().0;
                first * 10 + last
            })
            .sum()
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    const EXAMPLE_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn example_1() {
        assert_eq!(r"142", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"281", part_two_set(EXAMPLE_2).to_string());
        assert_eq!(r"281", part_two_separate(EXAMPLE_2).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2023, 1, do_solve).unwrap();
    }
}
