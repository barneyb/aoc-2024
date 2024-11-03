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
year = int(sys.argv[2]) if len(sys.argv) >= 3 else aoc_now.year
day = int(sys.argv[1]) if len(sys.argv) >= 2 else aoc_now.day
if year < day:
    (year, day) = (day, year)
puzzle = Puzzle(year=year, day=day)
print()
print(f"{year} Day {day}: {puzzle.title}")
print()

example_tests = ""
has_part_two = False
for i, e in enumerate(puzzle.examples, start=1):
    example_tests += f"""
    const EXAMPLE_{i}: &str = r#"{e.input_data}"#;\n"""
for i, e in enumerate(puzzle.examples, start=1):
    asserts = ""
    if e.extra:
        asserts = f"""
        /*
         {e.extra}
         */"""
    if e.answer_a:
        asserts += f"""
        assert_eq!(r"{e.answer_a}", part_one(EXAMPLE_{i}).to_string());"""
    if e.answer_b:
        has_part_two = True
        asserts += f"""
        assert_eq!(r"{e.answer_b}", part_two(EXAMPLE_{i}).to_string());"""
    example_tests += f"""
    #[test]
    fn example_{i}() {{
        {asserts.strip()}
    }}"""
    print(f"Example {i}")
    print("-" * 80)
    print(e.input_data)
    print("-" * 80)
    print(f"Part A: {e.answer_a or '-'}")
    print(f"Part B: {e.answer_b or '-'}")
    print("-" * 80)
    print()

print("Today's Input:")
print("-" * 80)
print(f"{puzzle.input_data}")
print("-" * 80)
print()

yyear = f"y{year}"
zday = str(day) if day >= 10 else f"0{day}"
name = re.sub("[^a-z0-9]+", "_", puzzle.title.lower())
p2p = "" if has_part_two else "// "
params = dict(
    year=year,
    yyear=yyear,
    day=day,
    zday=zday,
    name=name,
    example_tests=example_tests.strip()
    or f"""#[test]
    fn test_part_one() {{
        assert_eq!(3, part_one("AoC"));
    }}

    {p2p}#[test]
    {p2p}fn test_part_two() {{
    {p2p}    assert_eq!(12, part_two("adventofcode"));
    {p2p}}}""",
    p2p=p2p,
)

print(params)

if subprocess.run(["git", "diff", "--exit-code"]).returncode != 0:
    subprocess.run(["git", "commit", "-am", "WIP"], check=True)

subprocess.run(["git", "fetch"], check=True)
# if master is at/after origin/master, use master, otherwise use origin/master
start_ref = (
    "master"
    if subprocess.run(
        ["git", "merge-base", "--is-ancestor", "origin/master", "master"]
    ).returncode
    == 0
    else "origin/master"
)
subprocess.run(
    ["git", "checkout", "-b", f"{year}/{zday}", "--no-track", start_ref], check=True
)

year_filename = f"./src/{yyear}.rs"
module_filename = f"./src/{yyear}/{name}_{zday}.rs"
binary_filename = f"./src/bin/{name}.rs"

with open(year_filename, "a", encoding="utf-8") as f:
    f.write(f"pub mod {name}_{zday};\n")

subprocess.run(["mkdir", "-p", f"./src/{yyear}"], check=True)
with open(module_filename, "w", encoding="utf-8") as f:
    f.write(
        Template(
            """use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::Other(part_one(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    input.len()
}

${p2p}fn part_two(input: &str) -> usize {
${p2p}    input.len()
${p2p}}

#[cfg(test)]
mod test {
    use super::*;

    $example_tests

    // #[test]
    // fn test_real_input() {
    //     crate::with_input($year, $day, do_solve).unwrap();
    // }
}
"""
        ).substitute(params)
    )

with open(binary_filename, "w", encoding="utf-8") as f:
    f.write(
        Template(
            """use aoc::$yyear::${name}_$zday::do_solve;
use std::io::Error;

fn main() -> Result<(), Error> {
    aoc::with_input($year, $day, do_solve)
}
"""
        ).substitute(params)
    )

subprocess.run(["cargo", "run", "--bin", name], check=True)
subprocess.run(["git", "add", module_filename, binary_filename], check=True)
day_spec = f"day {day}" if year == aoc_now.year else f"{year} day {day}"
subprocess.run(
    ["git", "commit", "-am", f"skeleton for {day_spec}: {puzzle.title}"], check=True
)
subprocess.run(["idea", binary_filename])
subprocess.run(["idea", module_filename])
