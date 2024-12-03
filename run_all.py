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


def format_ns(nanos):
    """I format the passed nanoseconds as a duration. The result will always be
    11 characters long, including the units.
    """
    if nanos > NANOS_PER_SEC:
        return f"{nanos / NANOS_PER_SEC :>7,.2f} sec"
    if nanos > NANOS_PER_MILLISECOND:
        return f"{nanos / NANOS_PER_MILLISECOND :>8,.2f} ms"
    if nanos > NANOS_PER_MICROSECOND:
        return f"{nanos / NANOS_PER_MICROSECOND :>8,.2f} µs"
    return f"{nanos:8,d} ns"


TOKENS_FILE = environ["HOME"] + "/.config/aocd/tokens.json"
with open(TOKENS_FILE, "r", encoding="utf-8") as f:
    TOKENS = {p: t for p, t in json.load(f).items()}
N_ACCOUNTS = len(TOKENS)
W_ACCOUNT = max([len(p) for p in TOKENS])
W_TITLE = 50
W_DIV = 3
WIDTH = W_TITLE + 5 + ((W_DIV + W_ACCOUNT) * N_ACCOUNTS)

start_run = perf_counter_ns()
print(f"{'Building':.<{WIDTH-30}}", end="", flush=True)
subprocess.run(
    ["cargo", "build", "--tests", "--profile", "release", "--quiet"], check=True
)
print(f"{'Done!':.>30}{format_ns(perf_counter_ns() - start_run)}")
row = ""
for provider in TOKENS:
    row += f" | {provider:^{W_ACCOUNT}}"
print(f"{row:>{WIDTH}}")
prev = None
env = {}
env.update(os.environ)
exit_code = 0
for y, d in sorted(compute_done()):
    if y != prev:
        print(f"{y} {'=' * (WIDTH-5)}")
        prev = y
    puzzle = Puzzle(year=y, day=d)
    print(f"{d:>4} {puzzle.title[0:W_TITLE]:{W_TITLE}}", end="", flush=True)
    start_puzzle = perf_counter_ns()
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
                f"y{y}::{puzzle_name(puzzle)}_{d:02}::test",
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
        if res.returncode != 0:
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
    print(format_ns(perf_counter_ns() - start_puzzle))
print("=" * WIDTH)
if exit_code == 0:
    fmt = GREEN
    status = "Success!"
else:
    fmt = RED + BOLD
    status = "Failed!"
print(f"{fmt}{status:{WIDTH}}{END}{format_ns(perf_counter_ns() - start_run)}")
exit(exit_code)
