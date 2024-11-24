#!/usr/bin/env python
import re
import subprocess
from random import Random

# noinspection PyUnresolvedReferences
from aocd.models import Puzzle

from lib import (
    aoc_now,
    BOLD,
    END,
    FAINT,
    last_day_of_year,
    load_deps,
    MAX_YEAR,
    MIN_YEAR,
    NEGATIVE,
    YD,
)


def no_day_25_unless_complete(yd, done):
    (y, d) = yd
    if d == 25:
        for d in range(0, 25):
            if (y, d) not in done:
                return False
    return True


def find_dependency_free(yd: YD, done: frozenset[YD]) -> YD:
    known_deps = load_deps()
    queue = [yd]  # tee-hee
    while queue:
        yd = queue.pop(0)
        if yd not in known_deps:
            return yd
        unsatisfied = filter(lambda d: d not in done, known_deps[yd])
        if unsatisfied:
            queue.extend(unsatisfied)
            continue
        return yd


def suggest(done: frozenset[YD]) -> YD:
    # this year first!
    if aoc_now.year == MAX_YEAR:
        for d in range(last_day_of_year(MAX_YEAR), 0, -1):
            day = MAX_YEAR, d
            if day not in done:
                return day
    total = sum([last_day_of_year(y) for y in range(MIN_YEAR, MAX_YEAR + 1)])
    if len(done) == total:
        return None
    prev = done
    # flood the grid
    while True:
        curr = set(prev)
        for y, d in prev:
            if y == MIN_YEAR:
                if d <= last_day_of_year(MAX_YEAR):
                    curr.add((MAX_YEAR, d))
                else:
                    curr.add((MAX_YEAR - 1, d))
            else:
                curr.add((y - 1, d))
            if y == MAX_YEAR or d > last_day_of_year(y + 1):
                curr.add((MIN_YEAR, d))
            else:
                curr.add((y + 1, d))
            if d == 1:
                curr.add((y, last_day_of_year(y)))
            else:
                curr.add((y, d - 1))
            if d == last_day_of_year(y):
                curr.add((y, 1))
            else:
                curr.add((y, d + 1))
        if len(curr) == total:
            # it's flooded; find one of the last to be reached
            candidates = list(curr - prev)
            y_factor = 25 / (MAX_YEAR - MIN_YEAR + 1)
            candidates.sort(key=lambda yd: (yd[0] - MIN_YEAR) * y_factor + yd[1])
            sugg = next(
                filter(lambda yd: no_day_25_unless_complete(yd, done), candidates),
                None,
            )
            if sugg is None:
                candidates = list(prev - done)
                Random(hash(done)).shuffle(candidates)
                sugg = next(
                    filter(lambda yd: no_day_25_unless_complete(yd, done), candidates)
                )
            return find_dependency_free(sugg, done)
        prev = curr


def compute_done() -> frozenset[YD]:
    rust_files = subprocess.run(
        ["find", "src", "-name", "*_*.rs"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    pat = re.compile(r".*/y(\d{4})/.*_(\d{2})\.rs")
    return frozenset(
        [
            (int(m.group(1)), int(m.group(2)))
            for m in [
                re.fullmatch(pat, file) for file in rust_files.strip().splitlines()
            ]
            if m
        ]
    )


def print_status():
    done = compute_done()
    row = "       "
    for d in range(1, 26):
        row += f" {d:2}"
    print(row + f" {FAINT}│   #{END}")
    print(FAINT + "──────┬─" + "─" * 25 * 3 + f"┼─────{END}")
    suggestion = suggest(done)
    total_count = 0
    for y in range(MIN_YEAR, MAX_YEAR + 1):
        row = f" {y} {FAINT}│{END}"
        end_day = last_day_of_year(y)
        count = 0
        for d in range(1, 26):
            if d > end_day:
                row += "   "
            elif (y, d) in done:
                count += 2  # two stars per day!
                row += f"  {BOLD}*{END}"
            elif (y, d) == suggestion:
                row += f"  {NEGATIVE}*{END}"
            else:
                row += f"  {FAINT}.{END}"
        total_count += count
        print(f"{row} {FAINT}│ {count:3}{END}")
    print(FAINT + "──────┴─" + "─" * 25 * 3 + f"┼─────{END}")
    if suggestion:
        (y, d) = suggestion
        puzzle = Puzzle(year=y, day=d)
        sugg = f"{puzzle.title}  |  adventofcode.com/{y}/day/{d}"
    else:
        sugg = ""
    print(f"{sugg:^83}{FAINT}│ {total_count:3}{END}")


if __name__ == "__main__":
    print_status()
