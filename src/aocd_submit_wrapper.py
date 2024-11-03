import sys
from aocd.models import Puzzle

[_, year, day, part, val] = sys.argv
if part != "a" and part != "b":
    raise TypeError(f"Unknown '{part}' part")
puzzle = Puzzle(year=int(year), day=int(day))
if not getattr(puzzle, f"answered_{part}"):
    setattr(puzzle, f"answer_{part}", val)
    if getattr(puzzle, f"answered_{part}"):
        exit(0)  # woo!
    else:
        exit(1)  # bummer
answer = getattr(puzzle, f"answer_{part}")
if val == answer:
    print(f"'{val}' is correct!")
    exit(0)
print(f"Expected '{answer}', actual '{val}'")
exit(2)
