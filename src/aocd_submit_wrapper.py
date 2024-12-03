import sys

from aocd.models import Puzzle
from aocd.post import submit

[_, year, day, part, val] = sys.argv
if part != "a" and part != "b":
    raise TypeError(f"Unknown '{part}' part")
puzzle = Puzzle(year=int(year), day=int(day))
if not getattr(puzzle, f"answered_{part}"):
    # this is a little silly, but Puzzle itself doesn't offer a reopen flag
    submit(val, part=part, day=puzzle.day, year=puzzle.year, reopen=False)
    if getattr(puzzle, f"answered_{part}"):
        exit(0)  # woo!
    else:
        exit(1)  # bummer
answer = getattr(puzzle, f"answer_{part}")
if val == answer:
    print(f"'{val}' is correct!")
    exit(0)
if val.__contains__("\n"):
    print(f"Expected '{answer}', actual:\n{val}")
else:
    print(f"Expected '{answer}', actual '{val}'")
exit(2)
