#!/usr/bin/env python

# noinspection PyUnresolvedReferences
from aocd.models import Puzzle

from lib import (
    BOLD,
    compute_done,
    current_yd,
    END,
    FAINT,
    last_day_of_year,
    MAX_YEAR,
    MIN_YEAR,
    NEGATIVE,
    suggest_next,
)


def print_status():
    done = compute_done()
    row = "       "
    for d in range(1, 26):
        row += f" {d:2}"
    print(row + f" {FAINT}│   #{END}")
    print(FAINT + "──────┬─" + "─" * 25 * 3 + f"┼─────{END}")
    suggestion = suggest_next(done)
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
                if suggestion == current_yd():
                    row += f"  {NEGATIVE}*{END}"
                else:
                    row += f"  ?"
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
