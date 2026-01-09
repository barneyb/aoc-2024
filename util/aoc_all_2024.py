import os
import re
import subprocess

# noinspection PyUnresolvedReferences
from aocd.models import Puzzle

from .lib import (
    compute_done,
    puzzle_name,
)


def support():
    return compute_done()


def solve(year, day, data):
    puzzle = Puzzle(year=year, day=day)
    env = {}
    env.update(os.environ)
    env["BEB_EXTERNAL_RUN"] = "1"
    proc = subprocess.run(
        [
            "cargo",
            "test",
            "--profile",
            "release",
            "--quiet",
            "--lib",
            f"y{year}::{puzzle_name(puzzle)}_{day:02}::test::test_real_input",
            "--",
            "--show-output",
            "--exact",
        ],
        cwd=os.path.join(os.path.dirname(__file__), ".."),
        env=env,
        capture_output=True,
        text=True,
    )
    part_a = None
    part_b = None
    part_t = None
    for p, ans in re.findall(r"\[__AOCD_VERIFY_([ABT])__\[([^]]+)]]", str(proc.stdout)):
        if p == "A":
            if part_a is None:
                part_a = ans
            elif part_a != ans:
                raise Exception(f"Multiple answers for part A?! '{part_a}' and {ans}")
        elif p == "B":
            if part_b is None:
                part_b = ans
            elif part_b != ans:
                raise Exception(f"Multiple answers for part B?! '{part_b}' and {ans}")
        elif part_t is None:
            part_t = int(ans)
        else:
            raise Exception(f"Multiple reported times?! '{part_t}' and {ans}")
    return part_a, part_b, part_t
