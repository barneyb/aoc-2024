use crate::Part;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    tx.send(Part::B(part_two(input).to_string())).unwrap();
}

type Passport = HashMap<Field, String>;

//noinspection RsEnumVariantNaming
#[derive(Debug, Eq, PartialEq, Hash)]
enum Field {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

impl From<&str> for Field {
    fn from(value: &str) -> Self {
        use Field::*;
        match value {
            "byr" => Byr,
            "iyr" => Iyr,
            "eyr" => Eyr,
            "hgt" => Hgt,
            "hcl" => Hcl,
            "ecl" => Ecl,
            "pid" => Pid,
            "cid" => Cid,
            s => panic!("Unknown '{s}' field?!"),
        }
    }
}

#[derive(Debug)]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

impl FromStr for EyeColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EyeColor::*;
        match s {
            "amb" => Ok(Amb),
            "blu" => Ok(Blu),
            "brn" => Ok(Brn),
            "gry" => Ok(Gry),
            "grn" => Ok(Grn),
            "hzl" => Ok(Hzl),
            "oth" => Ok(Oth),
            _ => Err(()),
        }
    }
}

fn part_one(input: &str) -> usize {
    either_part(input, is_valid)
}

fn either_part<F>(input: &str, is_valid: F) -> usize
where
    F: Fn(&Passport) -> bool,
{
    let mut valid = 0;
    let mut passport = HashMap::new();
    for line in input.lines() {
        if line == "" {
            if is_valid(&passport) {
                valid += 1;
            }
            passport.clear();
        }
        for pair in line.split_ascii_whitespace() {
            let (k, v) = pair.split_once(':').unwrap();
            passport.insert(k.into(), v.to_string());
        }
    }
    if is_valid(&passport) {
        valid += 1;
    }
    valid
}

fn is_valid(passport: &Passport) -> bool {
    use Field::*;
    [Byr, Iyr, Eyr, Hgt, Hcl, Ecl, Pid]
        .iter()
        .all(|k| passport.contains_key(k))
}

fn part_two(input: &str) -> usize {
    use Field::*;
    either_part(input, |p| {
        if !is_valid(p) {
            return false;
        }
        for (k, v) in p {
            if !match k {
                Byr => int_between(v, 1920, 2002),
                Iyr => int_between(v, 2010, 2020),
                Eyr => int_between(v, 2020, 2030),
                Hgt => {
                    if v.ends_with("cm") {
                        int_between(&v[..v.len() - 2], 150, 193)
                    } else if v.ends_with("in") {
                        int_between(&v[..v.len() - 2], 59, 76)
                    } else {
                        false
                    }
                }
                Hcl => {
                    v.len() == 7
                        && v.starts_with("#")
                        && v[1..].chars().all(|c| "0123456789abcdef".contains(c))
                }
                Ecl => v.parse::<EyeColor>().is_ok(),
                Pid => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
                _ => true,
            } {
                return false;
            }
        }
        return true;
    })
}

/// both bounds are inclusive!
fn int_between(s: &str, lo: i32, hi: i32) -> bool {
    if let Ok(n) = s.parse::<i32>() {
        n >= lo && n <= hi
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

    const EXAMPLE_2: &str = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;

    const EXAMPLE_3: &str = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;

    #[test]
    fn example_1() {
        assert_eq!(r"2", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(r"0", part_two(EXAMPLE_2).to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(r"4", part_two(EXAMPLE_3).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2020, 4, do_solve).unwrap();
    }
}
