#!/usr/bin/env python
import datetime
import re
import subprocess
import sys
from string import Template
from zoneinfo import ZoneInfo
from aocd.models import Puzzle
AOC_TZ = ZoneInfo("America/New_York")
aoc_now = datetime.datetime.now(tz=AOC_TZ)
day = int(sys.argv[2]) if len(sys.argv) >= 3 else aoc_now.day
year = int(sys.argv[1]) if len(sys.argv) >= 2 else aoc_now.year
puzzle = Puzzle(year=year, day=day)
print(f"\nInput for {year} Day {day}:\n---------------------\n{puzzle.input_data}\n")

yyear = f"y{year}"
zday = str(day) if day >= 10 else f"0{day}"
name = re.sub("[^a-z0-9]+", "_", puzzle.title.lower())
params = dict(
    year = year,
    yyear = yyear,
    day = day,
    zday = zday,
    name = name,
)

print(params)

if subprocess.run(["git", "diff", "--exit-code"]).returncode != 0:
    subprocess.run(["git", "commit", "-am", "WIP"], check=True)

subprocess.run(["git", "fetch"], check=True)
# if master is at/after origin/master, use master, otherwise use origin/master
start_ref ="master" if subprocess.run(["git", "merge-base", "--is-ancestor", "origin/master", "master"]).returncode == 0 else "origin/master"
subprocess.run(["git", "checkout", "-b", f"{year}/{zday}", "--no-track", start_ref], check=True)

year_filename = f"./src/{yyear}.rs"
module_filename = f"./src/{yyear}/{name}_{zday}.rs"
binary_filename = f"./src/bin/{name}.rs"

with open(year_filename, "a", encoding="utf-8") as f:
    f.write(f"pub mod {name}_{zday};\n")

subprocess.run(["mkdir", "-p", f"./src/{yyear}"], check=True)
with open(module_filename, "w", encoding="utf-8") as f:
    f.write(Template("""pub fn part_one(input: &str) -> usize {
    input.len()
}

// pub fn part_two(input: &str) -> usize {
//     input.len()
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(3, part_one("AoC"));
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(12, part_two("adventofcode"));
    // }

    // #[test]
    // fn test_real_input() {
    //     use crate::{with_input, Part};
    //     with_input($year, $day, |input, tx| {
    //         tx.send(Part::A(Box::new(part_one(input)))).unwrap();
    //         // tx.send(Part::B(Box::new(part_two(input)))).unwrap();
    //     })
    //     .unwrap();
    // }
}
""").substitute(params))

with open(binary_filename, "w", encoding="utf-8") as f:
    f.write(Template("""use aoc::$yyear::${name}_$zday::part_one;
use aoc::{with_input, Part};
use std::io::Error;

fn main() -> Result<(), Error> {
    with_input($year, $day, |input, tx| {
        tx.send(Part::Other(Box::new(part_one(input)))).unwrap();
        // tx.send(Part::A(Box::new(part_one(input)))).unwrap();
        // tx.send(Part::B(Box::new(part_two(input)))).unwrap();
    })
}
""").substitute(params))

subprocess.run(["cargo", "run", "--bin", name], check=True)
subprocess.run(["git", "add", year_filename, module_filename, binary_filename], check=True)
subprocess.run(["idea", binary_filename])
subprocess.run(["idea", module_filename])
