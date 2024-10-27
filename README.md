# Advent of Code 2024

The yearly repo, seeded with a few solvers for random problems from prior years.
If you don't know what [Advent of Code](https://adventofcode.com) is, you should
go see! It's both lovely, and will help make sense of this repo. :)

https://github.com/barneyb/aoc2017 has an index, if you want my details.

## Architecture

The main solvers are implenented as Rust modules, one per day. They leverage
[advent-of-code-data](https://github.com/wimglenn/advent-of-code-data), which
is a python library for interacting with the [adventofcode.com](https://adventofcode.com)
API. Retrieving input comes out of the box; there's a small wrapper script `src`
that helps with and submitting answers. This alleviates a lot of manual steps,
as well as keeps all of my "private" info out of the source code.

## Running

You need a current-ish Rust, a current-ish Python, and `aocd` both pip-installed
and configured with your AoC token. While there are some binary solvers, a given
solver module's tests are where to start.
