import datetime
import json
import os
import re
import subprocess
from doctest import master
from random import Random
from zoneinfo import ZoneInfo

YD = (int, int)
Deps = dict[YD, set[YD]]

AOC_TZ = ZoneInfo("America/New_York")
aoc_now = datetime.datetime.now(tz=AOC_TZ)
MIN_YEAR = 2015
MAX_YEAR = aoc_now.year if aoc_now.month == 12 else aoc_now.year - 1
BOLD = "\033[1m"
FAINT = "\033[2m"
NEGATIVE = "\033[7m"
END = "\033[0m"
DEPS_FILE = ".deps.json"


def compute_done() -> frozenset[YD]:
    # only consider files on master, not whatever is checked out
    rust_files = subprocess.run(
        ["git", "ls-tree", "master", "-r", "--name-only", "src"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    pat = re.compile(r".*/y(\d{4})/.*_(\d{2})\.rs")
    return frozenset(
        [
            (int(m.group(1)), int(m.group(2)))
            for m in [
                re.fullmatch(pat, file) for file in rust_files.strip().splitlines()
            ]
            if m
        ]
    )


def current_branch():
    return subprocess.run(
        ["git", "name-rev", "--name-only", "HEAD"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout.strip()


def current_yd():
    branch = current_branch()
    if re.match("\d{4}/\d{2}", branch):
        return tuple(map(int, branch.split("/")))
    else:
        return None


def last_day_of_year(year):
    return min(25, aoc_now.day) if year == aoc_now.year else 25


def load_deps() -> Deps:
    if not os.path.isfile(DEPS_FILE):
        return dict()
    s = subprocess.run(
        ["git", "cat-file", "--textconv", f"master:{DEPS_FILE}"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout.strip()
    return dict([(tuple(d), {tuple(d) for d in ds}) for [d, ds] in json.loads(s) if ds])


def save_deps(deps: Deps):
    wc_dirty = subprocess.run(["git", "diff", "--exit-code"]).returncode != 0
    if wc_dirty:
        subprocess.run(["git", "stash"], check=True)
    branch = current_branch()
    if branch != master:
        subprocess.run(["git", "checkout", "master"])
    with open(DEPS_FILE, "w", encoding="utf-8") as f:
        # Unclear why 'f' (a text-mode file) isn't considered 'SupportsWrite[str]'
        # noinspection PyTypeChecker
        json.dump(
            [(d, sorted(list(ds))) for d, ds in sorted(deps.items()) if ds],
            f,
            indent=4,
            sort_keys=True,
        )
    subprocess.run(
        ["git", "commit", "-m", f"update dependency graph", "--", DEPS_FILE], check=True
    )
    if branch != master:
        subprocess.run(["git", "checkout", branch])
    if wc_dirty:
        subprocess.run(["git", "stash", "pop"], check=True)


def no_day_25_unless_complete(yd, done):
    (y, d) = yd
    if d == 25:
        for d in range(0, 25):
            if (y, d) not in done:
                return False
    return True


def find_dependency_free(yd: YD, done: frozenset[YD]) -> YD:
    known_deps = load_deps()
    queue = [yd]  # tee-hee
    while queue:
        yd = queue.pop(0)
        if yd not in known_deps:
            return yd
        unsatisfied = [d for d in known_deps[yd] if d not in done]
        if unsatisfied:
            queue.extend(unsatisfied)
            continue
        return yd


def suggest_next(done: frozenset[YD] = None) -> YD:
    if done is None:
        done = compute_done()
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
    rounds = []
    # flood the grid
    while True:
        curr = set()
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
        rounds.append(curr - prev)
        curr = curr.union(prev)
        if len(curr) == total:
            # it's flooded; find one of the last to be reached
            y_factor = 25 / (MAX_YEAR - MIN_YEAR + 1)
            no_day_25 = lambda yd: no_day_25_unless_complete(yd, done)
            random = Random(hash(done))
            rounds.reverse()
            for candidates in rounds:
                candidates = list(candidates)
                random.shuffle(candidates)
                for c in filter(no_day_25, candidates):
                    c = find_dependency_free(c, done)
                    if c is not None:
                        return c
            return None
        prev = curr
