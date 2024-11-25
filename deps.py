#!/usr/bin/env python
import sys

from lib import Deps, END, FAINT, load_deps, save_deps, suggest_next, YD

ByYear = dict[int, list[(int, bool)]]


def gather_deps(yd: YD, known_deps: Deps) -> ByYear:
    queue = [(yd, False)]  # tee-hee
    by_year: ByYear = dict()
    visited = set()
    while queue:
        (yd, transitive) = queue.pop(0)
        if yd not in known_deps:
            continue
        deps = list(known_deps[yd])
        deps.sort()
        for k in deps:
            if k in visited:
                continue
            visited.add(k)
            (y, d) = k
            if y in by_year:
                by_year[y].append((d, transitive))
            else:
                by_year[y] = [(d, transitive)]
            queue.append(((y, d), True))
    return by_year


def draw_dep_list(this_year, deps_by_year, dep_year):
    deps = deps_by_year[dep_year]
    return (
        (FAINT if deps[0][1] else "")
        + f"{'     day' if this_year == dep_year else f'{dep_year} day'}{END} "
        + ", ".join(f"{FAINT}{d:2}" if t else f"{d:2}" for d, t in deps)
        + END
    )


def print_deps(known_deps: Deps):
    print("Current Dependencies:\n")
    keys = list(known_deps)
    keys.sort()
    for k in keys:
        (year, day) = k
        by_year = gather_deps(k, known_deps)
        years = list(by_year)
        years.sort(key=lambda n: -n)
        first_year = years.pop(0)
        print(f"  {year} day {day:2} -> {draw_dep_list(year, by_year, first_year)}")
        for y in years:
            print(f"              -> {draw_dep_list(year, by_year, y)}")
    print()


if __name__ == "__main__":
    sugg = suggest_next()

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
