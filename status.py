#!/usr/bin/env python
import math
from collections import defaultdict

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
    RED,
    suggest_next,
)


def print_status(*, include_working_copy: bool = False):
    done = compute_done(include_working_copy=include_working_copy)
    suggestion = suggest_next(done)
    row = "       "
    for d in range(1, 26):
        row += f" {d:2}"
    print(row + f" {FAINT}│   #{END}")
    print(FAINT + "──────┬─" + "─" * 25 * 3 + f"┼─────{END}")
    total_count = 0
    day_hist = defaultdict(lambda: 0)
    for y in range(MIN_YEAR, MAX_YEAR + 1):
        row = f" {y} {FAINT}│{END}"
        end_day = last_day_of_year(y)
        count = 0
        sum = 0
        for d in range(1, 26):
            if d <= end_day and (y, d) in done:
                count += 1
                sum += d
        midpoint = 1 if count == 0 else math.floor(sum / count) + 1
        count = 0
        for d in range(1, 26):
            if d == midpoint:
                row += f"{RED}|{END}"
                midpoint = None
            else:
                row += " "
            if d > end_day:
                row += "  "
            elif (y, d) in done:
                count += 2  # two stars per day!
                day_hist[d] += 2
                row += f" {BOLD}*{END}"
            elif (y, d) == suggestion:
                if suggestion == current_yd():
                    row += f" {NEGATIVE}*{END}"
                else:
                    row += f" ?"
            else:
                row += f" {FAINT}.{END}"
        total_count += count
        if count == 0:
            count = "."
        print(f"{row} {FAINT}│ {count:>3}{END}")
    print(FAINT + "──────┼─" + "─" * 25 * 3 + f"┼─────{END}")
    count = 0
    sum = 0
    for d in range(1, 26):
        count += day_hist[d]
        sum += day_hist[d] * d
    midpoint = 1 if count == 0 else math.floor(sum / count) + 1
    row = f"{FAINT}{'│':>7}"
    for d in range(1, 26):
        if d == midpoint:
            row += f"{RED}|{END}{FAINT}"
            midpoint = 999999
        else:
            row += " "
        count = day_hist[d]
        if count == 0:
            count = "."
        row += f"{count:>2}"
    print(f"{row} │ {total_count:3}{END}")
    if suggestion:
        (y, d) = suggestion
        puzzle = Puzzle(year=y, day=d)
        sugg = f"{puzzle.title} {FAINT}({puzzle.url}){END}"
    else:
        sugg = ""
    print(f"  {FAINT}Next:{END} {sugg:^89}")


if __name__ == "__main__":
    print_status()
