#!/usr/bin/env python
import re
import subprocess
import sys
from string import Template

# noinspection PyUnresolvedReferences
from aocd.models import Puzzle

from lib import (
    AOC_NOW,
    compute_done,
    END,
    FAINT,
    GREEN,
    last_day_of_year,
    MAX_YEAR,
    MIN_YEAR,
    puzzle_name,
    RED,
    suggest_next,
)

done = compute_done()
if len(sys.argv) == 1:
    # use the current suggestion
    sugg = suggest_next(done)
    if not sugg:
        print("No day is suggested; specify one explicitly.")
        exit(4)
    (year, day) = sugg
else:
    # grab the params, interpret, and error-correct
    year = int(sys.argv[2]) if len(sys.argv) >= 3 else MAX_YEAR
    day = int(sys.argv[1]) if len(sys.argv) >= 2 else AOC_NOW.day
    if year < day:
        (year, day) = (day, year)
    if day > 25:
        print(f"There's no day {day}?!")
        exit(1)
    if year == MAX_YEAR and day > last_day_of_year(year):
        year -= 1
    while year > MIN_YEAR and (year, day) in done:
        year -= 1
zday = str(day) if day >= 10 else f"0{day}"
branch_name = f"{year}/{zday}"
branch_exists = subprocess.run(
    ["git", "branch", "--list", branch_name], capture_output=True, text=True, check=True
).stdout.strip()
puzzle = Puzzle(year=year, day=day)
if (year, day) in done:
    print(f"You've already done {year} day {day} ({puzzle.title})?!")
    print(
        f"To re-initialize, rename/delete the existing module{' (and branch!)' if branch_exists else ''} first."
    )
    exit(2)
yyear = f"y{year}"
if branch_exists:
    print(f"You already have a '{branch_name}' branch?!")
    exit(3)
input_data = puzzle.input_data  # do this early, to fail on a bad token
name = puzzle_name(puzzle)
print(f"{year} Day {day}: {puzzle.title}")

# first, ensure we're ready to start a new day...
subprocess.run(["cargo", "fmt"], check=True)
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
    ["git", "checkout", "-b", branch_name, "--no-track", start_ref], check=True
)
subprocess.run(["cargo", "test", "--profile", "release"], check=True)

print()
print(f"{year} Day {day}: {puzzle.title}")
print()


def prefix_lines(prefix, str):
    """
    Add a prefix to each line, at the same indent level as the least-indented
    line in the string. Blank lines are prefixed, but aren't considered when
    computing the minimum indent.

    I should really be three functions: strip_indent, prefix_lines, and indent.

    :param prefix: The prefix to add to each line of the string.
    :param str: The string to add the prefix to the lines of.
    :return: A string with prefixes added.
    """
    lines = str.splitlines(True)
    indent = min([len(l) - len(l.lstrip()) for l in lines if l.strip() != ""])
    return "".join(
        [
            (prefix + l) if l.strip() == "" else (l[0:indent] + prefix + l[indent:])
            for l in lines
        ]
    )


example_inputs = ""
example_tests = ""
for i, e in enumerate(puzzle.examples, start=1):
    has_part_a = False
    asserts = ""
    if e.extra:
        asserts = f"""
        /*
         {e.extra}
         */"""
    if e.answer_a:
        has_part_a = True
        asserts += f"""
        assert_eq!(r"{e.answer_a}", part_one(EXAMPLE_{i}).to_string());"""
    if e.answer_b:
        prefix = "// " if has_part_a else ""
        asserts += f"""
        {prefix}assert_eq!(r"{e.answer_b}", part_two(EXAMPLE_{i}).to_string());"""
    example_inputs += f"""
    const EXAMPLE_{i}: &str = r#"{e.input_data}"#;\n"""
    tst = f"""
    #[test]
    fn example_{i}() {{
        {asserts.strip()}
    }}\n"""
    example_tests += tst if has_part_a else prefix_lines("// ", tst)
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
print(f"{input_data}")
print("-" * 80)
lines = input_data.splitlines()
words = input_data.split()
chars = input_data
print(f"{len(lines):8} lines")
print(f"{len(words):8} words")
print(f"{len(chars):8} chars")
if len(lines) > 1:
    lo = min(map(len, lines))
    hi = max(map(len, lines))
    if lo == hi:
        print(f"{lo:8} line length")
    else:
        print(f"{lo:8} min line length")
        print(f"{hi:8} max line length")
if "-" in input_data:
    color = GREEN
    msg = "DOES"
else:
    color = RED
    msg = "does NOT"
print(f"{color}{msg:8}{END} contain dashes")
print("-" * 80)
print(f"{year} Day {day}: {puzzle.title} {FAINT}({puzzle.url}){END}")
print()
subprocess.run(["open", puzzle.url], check=True)

params = dict(
    year=year,
    yyear=yyear,
    day=day,
    zday=zday,
    name=name,
    example_tests=(example_inputs + example_tests).strip()
    or f"""#[test]
    fn test_part_one() {{
        assert_eq!(3, part_one("AoC"));
    }}""",
)

year_filename = f"./src/{yyear}.rs"
module_filename = f"./src/{yyear}/{name}_{zday}.rs"
binary_filename = f"./src/bin/{name}.rs"

with open(year_filename, "a", encoding="utf-8") as f:
    f.write(f"pub mod {name}_{zday};\n")

subprocess.run(["mkdir", "-p", f"./src/{yyear}"], check=True)
with open(module_filename, "w", encoding="utf-8") as f:
    f.write(
        re.sub(
            "// *\n",
            "\n",
            Template(
                """\
use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::Other(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

fn part_one(_input: &str) -> usize {
    99999
}

// fn part_two(input: &str) -> usize {
//     99999
// }

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
            ).substitute(params),
        )
    )

with open(binary_filename, "w", encoding="utf-8") as f:
    f.write(
        Template(
            """\
use aoc::$yyear::${name}_$zday::do_solve;
use std::io::Error;

fn main() -> Result<(), Error> {
    aoc::with_input($year, $day, do_solve)
}
"""
        ).substitute(params)
    )

subprocess.run(["cargo", "fmt"], check=True)
subprocess.run(["cargo", "run", "--bin", name], check=True)
subprocess.run(["git", "add", module_filename, binary_filename], check=True)
day_spec = f"day {day}" if year == AOC_NOW.year else f"{year} day {day}"
subprocess.run(
    ["git", "commit", "-am", f"skeleton for {day_spec}: {puzzle.title}"], check=True
)
subprocess.run(["idea", binary_filename])
subprocess.run(["idea", module_filename])
