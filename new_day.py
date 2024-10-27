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
print(f"\nInput for {year} Day {day}:\n{puzzle.input_data}\n")

params = dict(
    year = year,
    yyear = f"y{year}",
    day = day,
    zday = str(day) if day >= 10 else f"0{day}",
    name = re.sub("[^a-z0-9]+", "_", puzzle.title.lower()),
)

print(params)

if subprocess.run(["git", "diff", "--exit-code"]).returncode != 0:
    subprocess.run(["git", "commit", "-am", "WIP"], check=True)

subprocess.run(["git", "fetch"], check=True)
subprocess.run(["git", "checkout", "-b", f"{year}/{day}", "--no-track", "origin/master"], check=True)

year_filename = f"./src/{params['yyear']}.rs"
with open(year_filename, "a", encoding="utf-8") as f:
    f.write(f"mod {params['name']}_{params['zday']};\n")

subprocess.run(["mkdir", "-p", f"./src/{params['yyear']}"], check=True)
module_file_name = f"./src/{params['yyear']}/{params['name']}_{params['zday']}.rs"
with open(module_file_name, "w", encoding="utf-8") as f:
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
    // //         tx.send(Part::B(Box::new(part_two(input)))).unwrap();
    //     })
    //     .unwrap();
    // }
}
""").substitute(params))

subprocess.run(["git", "add", year_filename, module_file_name], check=True)
subprocess.run(["idea", module_file_name])
