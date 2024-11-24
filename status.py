#!/usr/bin/env python
import datetime
import re
import subprocess
import sys
from zoneinfo import ZoneInfo

AOC_TZ = ZoneInfo("America/New_York")
aoc_now = datetime.datetime.now(tz=AOC_TZ)
MIN_YEAR = 2015
MAX_YEAR = aoc_now.year if aoc_now.month == 12 else aoc_now.year - 1


def last_day_of_year(year):
    return min(25, aoc_now.day) if year == aoc_now.year else 25


def suggest(done):
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
            last_reached = list(curr - prev)
            y_factor = 25 / (MAX_YEAR - MIN_YEAR + 1)
            last_reached.sort(key=lambda yd: (yd[0] - MIN_YEAR) * y_factor + yd[1])
            print(last_reached)
            return last_reached[0]
        prev = curr


def print_status(color):
    rust_files = subprocess.run(
        ["find", "src", "-name", "*_*.rs"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    done = set()
    pat = re.compile(r".*/y(\d{4})/.*_(\d{2})\.rs")
    for file in rust_files.strip().splitlines():
        m = re.fullmatch(pat, file)
        if m:
            done.add((int(m.group(1)), int(m.group(2))))
    BOLD = "\033[1m" if color else ""
    FAINT = "\033[2m" if color else ""
    END = "\033[0m" if color else ""
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
                row += f"  {BOLD}?{END}"
            else:
                row += f"  {FAINT}.{END}"
        total_count += count
        print(f"{row} {FAINT}│ {count:3}{END}")
    print(FAINT + "──────┴─" + "─" * 25 * 3 + f"┼─────{END}")
    if suggestion:
        (y, d) = suggestion
        sugg = f"Maybe {y} day {d} next?"
        print(f"      {sugg:77}{FAINT}│ {total_count:3}{END}")


if __name__ == "__main__":
    print_status(len(sys.argv) <= 1 or sys.argv[1] != "--no-color")
