#!/usr/bin/env python
import json
import os
import subprocess
import sys
from os import environ
from time import perf_counter_ns

# noinspection PyUnresolvedReferences
from aocd.models import Puzzle

from lib import BOLD, compute_done, END, FAINT, GREEN, puzzle_name, RED

NANOS_PER_MICROSECOND = 1_000
NANOS_PER_MILLISECOND = NANOS_PER_MICROSECOND * 1_000
NANOS_PER_SEC = NANOS_PER_MILLISECOND * 1_000
NANOS_PER_MINUTE = NANOS_PER_SEC * 60


def format_ns(nanos):
    """I format the passed nanoseconds as a duration. The result will always be
    11 characters long, including the units.
    """
    if nanos > NANOS_PER_MINUTE:
        return f"{nanos / NANOS_PER_MINUTE :>7,.2f} min"
    if nanos > NANOS_PER_SEC:
        return f"{nanos / NANOS_PER_SEC :>7,.2f} sec"
    if nanos > NANOS_PER_MILLISECOND:
        return f"{nanos / NANOS_PER_MILLISECOND :>8,.2f} ms"
    if nanos > NANOS_PER_MICROSECOND:
        return f"{nanos / NANOS_PER_MICROSECOND :>8,.2f} µs"
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
to_run = compute_done()
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
MIN_WIDTH = 72
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
for y, d in reversed(sorted(to_run)):
    if y != prev:
        print(f"{y} {'=' * (WIDTH-5)}")
        prev = y
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
            mark = f"{GREEN}{'✔':^{W_ACCOUNT}}{END}"
        else:
            mark = f"{RED}{'✖':^{W_ACCOUNT}}{END}"
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
            print(f"{RED}{BOLD}Failed!{END}")
            print(f"  {y} / {d} : {puzzle.title} for '{provider}'")
            print()
            if not exit_code:
                exit_code = res.returncode
            abandon_puzzle = True
            break
    if abandon_puzzle:
        continue
    if len(solve_nanos) == N_ACCOUNTS:
        print(format_ns(sum(solve_nanos)))
    else:
        print(format_ns(perf_counter_ns() - start_puzzle))
print("=" * WIDTH)
if exit_code == 0:
    fmt = GREEN
    ds = len(to_run)
    dl = "day" if ds == 1 else "days"
    al = "account" if N_ACCOUNTS == 1 else "accounts"
    status = f"Success!  {ds} {dl} x {N_ACCOUNTS} {al} = {ds * 2 * N_ACCOUNTS} stars!"
else:
    fmt = RED + BOLD
    status = "Failed!"
print(f"{fmt}{status:{WIDTH}}{END}{format_ns(perf_counter_ns() - start_run)}")
exit(exit_code)
