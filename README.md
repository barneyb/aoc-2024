# Advent of Code 2024

The yearly repo, seeded with a few solvers for random problems from prior years.
If you don't know what [Advent of Code](https://adventofcode.com) is, you should
go see! It's both lovely, and will help make sense of this repo. :)

This repo's current solve status:

<pre id="current-status">
         1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 │   #
──────┬────────────────────────────────────────────────────────────────────────────┼─────
 2015 │  *  .  .  .  .  *  .| .  .  .  .  .  .  .  .  *  .  .  .  .  .  .  .  .  . │   6
 2016 │  *  .  .  .  .  .  .  .  .  .  *  *  *| .  .  .  .  .  *  .  .  .  .  .  . │  10
 2017 │  *  *  .| .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  *  .  . │   6
 2018 │  .  .  .  .  *  .  .  .  .  !  .| .  .  .  .  .  *  .  .  .  .  .  .  .  . │   4
 2019 │  .  *  .  .  .  .  .  *  .| .  .  .  .  *  .  .  .  .  .  ?  .  .  .  .  . │   6
 2020 │  *  .  .  *  .  .  .  .  .  .  *  .| .  .  .  .  .  *  .  .  .  *  .  .  . │  10
 2021 │  .  *  .  .| .  .  *  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  . │   4
 2022 │  .  .  .  .  .  .  .  .  *  .  .  .  *  .| .  .  .  .  .  *  .  .  .  .  . │   6
 2023 │  *  .  *  .| .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  *  .  . │   6
 2024 │  *| *                                                                      │   4
──────┼────────────────────────────────────────────────────────────────────────────┼─────
      │ 12  8  2  2  2  2  2  2  2| .  4  2  4  2  .  2  2  2  2  2  .  2  4  .  . │  62
  Prog! The Stars Align (https://adventofcode.com/2018/day/10)
  Next? Donut Maze (https://adventofcode.com/2019/day/20)
</pre>

https://github.com/barneyb/aoc2017 has an index of all years' repositories.

## Architecture

The main solvers are implemented as Rust modules, one per day. They leverage
[advent-of-code-data](https://github.com/wimglenn/advent-of-code-data), a Python
library for interacting with the [adventofcode.com](https://adventofcode.com)
API. Retrieving input comes out of the box; there's a small wrapper script `src`
that helps with and submitting answers. This alleviates a lot of manual steps,
as well as keeps all of my "private" info out of the source code.

The general idea is to run `./new_day.py`, which sets up a day to work on, get
it solved, then run `./done.py`. The generated skeleton will take care of both
submission and re-validation of your input, along with whatever examples are
available.

The `./new_day.py` script accepts year and day params if you want to set up a
specific puzzle. With no options, it'll select the latest unsolved day from this
year, if one exists. Otherwise, one that is "far away" from what you've already
solved will be chosen. 2019, in particular, has dependencies between puzzles;
you won't get later puzzles unless you've solved the earlier ones.

## Running

You need a current-ish Rust, a current-ish Python, and `aocd` both pip-installed
and [configured with your AoC token](https://github.com/wimglenn/advent-of-code-data#quickstart).
While there are binary solvers, a given solver module's tests are where to start.

When your token expires in ~30 days, you'll get completely non-handled 400 HTTP
errors. Go update your token.

## Visualization

Some of the binaries do visualization, rather than spit out answers. One example
is `probably_a_fire_hazard` ([2015 day 6](https://adventofcode.com/2015/day/6)),
which displays the final winning light pattern. Here's mine:

![Probably a Fire Hazard](viz/probably_a_fire_hazard.png)
