# Advent of Code 2024

The yearly repo, seeded with a few solvers for random problems from prior years.
If you don't know what [Advent of Code](https://adventofcode.com) is, you should
go see! It's both lovely, and will help make sense of this repo. :)

<pre id="current-status">
         1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 │   #
──────┬────────────────────────────────────────────────────────────────────────────┼─────
 2015 │  *  .  .  .  .  *  .  .  .  .  .  .  .  .  .  *  .  .  .  .  .  .  .  .  . │   6
 2016 │  *  .  .  .  .  .  .  .  .  .  *  *  *  .  .  .  .  .  *  .  .  .  .  .  . │  10
 2017 │  *  *  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  *  .  . │   6
 2018 │  .  .  .  .  *  .  .  .  .  .  .  .  .  .  .  .  *  .  .  .  .  .  .  .  . │   4
 2019 │  .  *  .  .  .  .  .  *  .  .  .  .  .  *  .  .  .  .  .  .  .  .  .  .  . │   6
 2020 │  *  .  .  .  .  .  .  .  .  .  *  .  .  .  .  .  .  *  .  .  .  *  .  .  . │   8
 2021 │  .  *  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  . │   2
 2022 │  .  .  .  .  .  .  .  .  *  .  .  .  *  .  .  .  .  .  .  *  .  .  .  ?  . │   6
 2023 │  .  .  *  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  . │   2
──────┼────────────────────────────────────────────────────────────────────────────┼─────
      │  8  6  2  .  2  2  .  2  2  .  4  2  4  2  .  2  2  2  2  2  .  2  2  .  . │  50
                     Blizzard Basin  /  adventofcode.com/2022/day/24
</pre>

https://github.com/barneyb/aoc2017 has an index of all years' repositories.

## Architecture

The main solvers are implemented as Rust modules, one per day. They leverage
[advent-of-code-data](https://github.com/wimglenn/advent-of-code-data), which
is a python library for interacting with the [adventofcode.com](https://adventofcode.com)
API. Retrieving input comes out of the box; there's a small wrapper script `src`
that helps with and submitting answers. This alleviates a lot of manual steps,
as well as keeps all of my "private" info out of the source code.

## Running

You need a current-ish Rust, a current-ish Python, and `aocd` both pip-installed
and configured with your AoC token. While there are some binary solvers, a given
solver module's tests are where to start.

## Visualization

Some of the binaries do visualization, rather than spit out answers. One example
is `probably_a_fire_hazard` ([2015 day 6](https://adventofcode.com/2015/day/6)),
which displays the final winning light pattern:

![Probably a Fire Hazard](viz/probably_a_fire_hazard.png)
