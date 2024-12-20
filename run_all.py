#!/usr/bin/env python
import json
import os
import subprocess
import sys
from os import environ
from time import perf_counter_ns

# noinspection PyUnresolvedReferences
from aocd.models import Puzzle

from lib import (
    BOLD,
    colored,
    Colors,
    compute_done,
    END,
    FAINT,
    GREEN,
    LIGHT_RED,
    puzzle_name,
    RED,
)

NANOS_PER_MICROSECOND = 1_000
NANOS_PER_MILLISECOND = NANOS_PER_MICROSECOND * 1_000
NANOS_PER_SEC = NANOS_PER_MILLISECOND * 1_000
NANOS_PER_MINUTE = NANOS_PER_SEC * 60


def format_ns(nanos):
    """I format the passed nanoseconds as a duration. The result will always be
    11 characters long, including the units.
    """
    if nanos > NANOS_PER_MINUTE:
        return f"{RED}{BOLD}{nanos / NANOS_PER_MINUTE :>7,.2f} min{END}"
    if nanos > NANOS_PER_SEC:
        return f"{LIGHT_RED}{nanos / NANOS_PER_SEC :>7,.2f} sec{END}"
    if nanos > NANOS_PER_MILLISECOND:
        return f"{nanos / NANOS_PER_MILLISECOND :>8,.2f} ms"
    if nanos > NANOS_PER_MICROSECOND:
        return f"{GREEN}{nanos / NANOS_PER_MICROSECOND :>8,.2f} µs{END}"
    return f"{nanos:8,d} ns"


year = day = None
if len(sys.argv) > 1:
    year = int(sys.argv[1])
    if len(sys.argv) > 2:
        day = int(sys.argv[2])
        if year < day:
            year, day = day, year
    elif year <= 25:
        day = year
        year = None

start_run = perf_counter_ns()
to_run = compute_done(include_working_copy=True)
if year is not None:
    if day is None:
        to_run = {(y, d) for (y, d) in to_run if y == year}
    elif (year, day) in to_run:
        to_run = {(year, day)}
    else:
        to_run = set()
elif day is not None:
    to_run = {(y, d) for (y, d) in to_run if d == day}

if not to_run:
    print("Nothing matched your filter?")
    exit(1)

TOKENS_FILE = environ["HOME"] + "/.config/aocd/tokens.json"
with open(TOKENS_FILE, "r", encoding="utf-8") as f:
    TOKENS = {p: t for p, t in json.load(f).items()}
N_ACCOUNTS = len(TOKENS)
MIN_WIDTH = 80  # this holds the longest title yet solved
W_ACCOUNT = max([len(p) for p in TOKENS])
W_TITLE = max([len(Puzzle(year=y, day=d).title) for y, d in to_run]) + 2
W_DIV = 3
WIDTH = W_TITLE + 5 + ((W_DIV + W_ACCOUNT) * N_ACCOUNTS)
if WIDTH < MIN_WIDTH:
    pad = MIN_WIDTH - WIDTH
    W_TITLE += pad
    WIDTH += pad

print(f"{'Building':.<10}", end="", flush=True)
subprocess.run(
    ["cargo", "build", "--tests", "--profile", "release", "--quiet"], check=True
)
print(f"{'Done!':.>{WIDTH-10}}{format_ns(perf_counter_ns() - start_run)}")
row = ""
for provider in TOKENS:
    row += f" | {provider:^{W_ACCOUNT}}"
print(f"{row:>{WIDTH}}")
prev = None
env = {}
env.update(os.environ)
env["BEB_SOLVE_NANOS"] = "1"
exit_code = 0
total_nanos = 0
for y, d in reversed(sorted(to_run)):
    if y != prev:
        prev = y
        print(f"{y} {'=' * (WIDTH-5)}")
    puzzle = Puzzle(year=y, day=d)
    print(f"{d:>4} {puzzle.title[0:W_TITLE]:{W_TITLE}}", end="", flush=True)
    start_puzzle = perf_counter_ns()
    solve_nanos = []
    abandon_puzzle = False
    for provider, tkn in TOKENS.items():
        print(f" {FAINT}|{END} ", end="", flush=True)
        env["AOC_SESSION"] = tkn
        res = subprocess.run(
            [
                "cargo",
                "test",
                "--profile",
                "release",
                "--quiet",
                "--lib",
                f"y{y}::{puzzle_name(puzzle)}_{d:02}::test::test_real_input",
                "--",
                "--show-output",
                "--exact",
            ],
            env=env,
            capture_output=True,
            text=True,
        )
        if res.returncode == 0:
            mark = colored(f"{'✔':^{W_ACCOUNT}}", Colors.GREEN)
        else:
            mark = colored(f"{'✖':^{W_ACCOUNT}}", Colors.RED)
        print(f"{mark}", end="", flush=True)
        if res.returncode == 0:
            for line in res.stdout.splitlines():
                if line.startswith("¡¡solve nanos ") and line.endswith("!!"):
                    solve_nanos.append(int(line[14:-2]))
                    break
        else:
            print(res.stdout)
            print(res.stderr, file=sys.stderr)
            print()
            print(colored("Failed!", RED, BOLD))
            print(f"  {y} / {d} : {puzzle.title} for '{provider}'")
            print()
            if not exit_code:
                exit_code = res.returncode
            abandon_puzzle = True
            break
    if abandon_puzzle:
        continue
    if len(solve_nanos) == N_ACCOUNTS:
        puzzle_nanos = sum(solve_nanos)
        print(format_ns(puzzle_nanos))
    else:
        puzzle_nanos = perf_counter_ns() - start_puzzle
        print(f"{format_ns(puzzle_nanos)}?!")
    total_nanos += puzzle_nanos
print("=" * WIDTH)
runtime = format_ns(perf_counter_ns() - start_run)
if exit_code == 0:
    fmt = GREEN
    ps = len(to_run)
    pl = "puzzle"
    if ps != 1:
        pl += "s"
    al = "account"
    if N_ACCOUNTS != 1:
        al += "s"
    stars = ps * 2 * N_ACCOUNTS
    per_star = total_nanos / stars
    print(f"{fmt}{'Success!':{WIDTH}}{END}{runtime}")
    print(
        f"{FAINT}{ps} {pl} x {N_ACCOUNTS} {al} = {stars} stars @{format_ns(per_star)}{FAINT} / star{END}"
    )
else:
    fmt = RED + BOLD
    print(f"{fmt}{'Failed!':{WIDTH}}{END}{runtime}")
exit(exit_code)
