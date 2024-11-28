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


def crab_midpoint(vals):
    """So named for 2021 Day 7: The Treachery of Whales"""
    if not vals:
        return 0
    lo = vals[0]
    hi = vals[-1]
    if lo == hi:
        return lo
    agg_dist = lambda pos: sum(map(lambda v: abs(pos - v), vals))
    while lo < hi:
        mid = int((lo + hi) / 2)
        a = agg_dist(mid)
        b = agg_dist(mid + 1)
        if a < b:
            hi = mid
        elif a > b:
            lo = mid + 1
        else:
            # no curvature
            return mid
    mid = math.floor(lo) + 1
    return mid


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
        row = f" {y} {FAINT}│{END} "
        end_day = last_day_of_year(y)
        stars = []
        for d in range(1, end_day + 1):
            if (y, d) in done:
                stars.append(d)
        midpoint = crab_midpoint(stars)
        count = 0
        for d in range(1, 26):
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
            if d == midpoint:
                row += f"{RED}|{END}"
                midpoint = None
            else:
                row += " "
        total_count += count
        if count == 0:
            count = "."
        print(f"{row}{FAINT}│ {count:>3}{END}")
    print(FAINT + "──────┼─" + "─" * 25 * 3 + f"┼─────{END}")
    stars = []
    for d in range(1, 26):
        stars.extend([d] * day_hist[d])
    midpoint = crab_midpoint(stars)
    row = f"{FAINT}{'│':>7} "
    for d in range(1, 26):
        count = day_hist[d]
        if count == 0:
            count = "."
        row += f"{count:>2}"
        if d == midpoint:
            row += f"{RED}|{END}{FAINT}"
            midpoint = 999999
        else:
            row += " "
    print(f"{row}│ {total_count:3}{END}")
    if suggestion:
        (y, d) = suggestion
        puzzle = Puzzle(year=y, day=d)
        sugg = f"{puzzle.title} {FAINT}({puzzle.url}){END}"
    else:
        sugg = ""
    print(f"  {FAINT}Next:{END} {sugg:^89}")


if __name__ == "__main__":
    print_status()
