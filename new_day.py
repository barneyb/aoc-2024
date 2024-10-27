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

params = dict(
    year = year,
    yyear = f"y{year}",
    day = day,
    zday = str(day) if day >= 10 else f"0{day}",
    name = re.sub("[^a-z0-9]+", "_", puzzle.title.lower()),
)

print(params)
with open(f"./src/{params['yyear']}.rs", "a", encoding="utf-8") as f:
    f.write(f"mod {params['name']}_{params['zday']};\n")
module_file_name=f"./src/y{year}/{params['name']}_{params['day']}.rs"
with open(module_file_name, "w", encoding="utf-8") as f:
    f.write(Template("""
pub fn part_one(input: &str) -> usize {
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
subprocess.run(["git", "add", module_file_name])
