#!/usr/bin/env python
import datetime
import re
import subprocess
from zoneinfo import ZoneInfo

AOC_TZ = ZoneInfo("America/New_York")
aoc_now = datetime.datetime.now(tz=AOC_TZ)


def print_status():
    rust_files = subprocess.run(
        ["find", "src", "-name", "*_*.rs"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    end_year = aoc_now.year - (1 if aoc_now.month < 12 else 0)
    done = set()
    pat = re.compile(r".*/y(\d{4})/.*_(\d{2})\.rs")
    for file in rust_files.strip().splitlines():
        m = re.fullmatch(pat, file)
        if m:
            done.add((int(m.group(1)), int(m.group(2))))
    BOLD = "\033[1m"
    FAINT = "\033[2m"
    END = "\033[0m"
    row = "       "
    for d in range(1, 26):
        row += f" {d:2}"
    print(row + f"  {FAINT}#{END}")
    print("------+-" + "-" * 25 * 3 + f"{FAINT}---{END}")
    for y in range(2015, end_year + 1):
        row = f" {y} |"
        end_day = min(25, aoc_now.day if y == aoc_now.year else 25)
        count = 8
        for d in range(1, end_day + 1):
            if (y, d) in done:
                count += 1
                row += f"  {BOLD}*{END}"
            else:
                row += f"  {FAINT}.{END}"
        print(f"{row} {FAINT}{count:2}{END}")
    print("------+-" + "-" * 25 * 3 + f"{FAINT}---{END}")


if __name__ == "__main__":
    print_status()
