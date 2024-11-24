#!/usr/bin/env python
import sys

from lib import Deps, load_deps, save_deps
from status import compute_done, suggest


def print_deps(known_deps: Deps):
    print("Current Dependencies:\n")
    keys = list(known_deps)
    keys.sort()
    for k in keys:
        (year, day) = k
        deps = list(known_deps[k])
        deps.sort()
        by_year = dict()
        for y, d in deps:
            if y in by_year:
                by_year[y].append(d)
            else:
                by_year[y] = [d]
        years = list(by_year)
        years.sort()
        first_year = years.pop(0)
        print(
            f"  {year} day {day:2} -> {'day' if year == first_year else f'{first_year} day'} {', '.join(f'{d}' for d in by_year[first_year])}"
        )
        for y in years:
            print(
                f"              -> {'     day' if year == y else f'{y} day'} {', '.join(f'{d}' for d in by_year[y])}"
            )
    print()


if __name__ == "__main__":
    done = compute_done()
    sugg = suggest(done)

    known_deps = load_deps()
    if len(sys.argv) == 1:
        print_deps(known_deps)
        if sugg:
            print(
                f"Add {'another' if sugg in known_deps else 'one'} for {sugg} with '{sys.argv[0]} <day> [ <year> ]'."
            )
        exit(0)

    if not sugg:
        print("No suggestion exists to add a dependency to?!")
        exit(4)

    year = int(sys.argv[2]) if len(sys.argv) >= 3 else sugg[0]
    day = int(sys.argv[1]) if len(sys.argv) >= 2 else sugg[1]
    if year < day:
        (year, day) = (day, year)
    new_dep = (year, day)
    if sugg == new_dep:
        print("A day can't depend on itself?!")
        exit(1)
    if sugg < new_dep:
        print("A day can't on a later day?!")
        exit(2)
    if sugg in known_deps:
        if new_dep in known_deps[sugg]:
            print("Dependency is already known?!")
            exit(3)
        known_deps[sugg].add(new_dep)
    else:
        known_deps[sugg] = {new_dep}
    save_deps(known_deps)
    print(f"Added dependency on {new_dep} to {sugg}")
    print_deps(known_deps)
