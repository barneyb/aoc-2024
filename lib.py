import datetime
import json
import os
import re
import subprocess
from doctest import master
from zoneinfo import ZoneInfo

YD = (int, int)
Deps = dict[YD, set[YD]]

AOC_TZ = ZoneInfo("America/New_York")
AOC_NOW = datetime.datetime.now(tz=AOC_TZ)
MIN_YEAR = 2015
MAX_YEAR = AOC_NOW.year if AOC_NOW.month == 12 else AOC_NOW.year - 1
DEPS_FILE = ".deps.json"


class Colors:
    """ANSI color codes"""

    BLACK = "\033[0;30m"
    RED = "\033[0;31m"
    GREEN = "\033[0;32m"
    BROWN = "\033[0;33m"
    BLUE = "\033[0;34m"
    PURPLE = "\033[0;35m"
    CYAN = "\033[0;36m"
    LIGHT_GRAY = "\033[0;37m"
    DARK_GRAY = "\033[1;30m"
    LIGHT_RED = "\033[1;31m"
    LIGHT_GREEN = "\033[1;32m"
    YELLOW = "\033[1;33m"
    LIGHT_BLUE = "\033[1;34m"
    LIGHT_PURPLE = "\033[1;35m"
    LIGHT_CYAN = "\033[1;36m"
    LIGHT_WHITE = "\033[1;37m"
    BOLD = "\033[1m"
    FAINT = "\033[2m"
    ITALIC = "\033[3m"
    UNDERLINE = "\033[4m"
    BLINK = "\033[5m"
    NEGATIVE = "\033[7m"
    CROSSED = "\033[9m"
    END = "\033[0m"
    # cancel SGR codes if we don't write to a terminal
    if not __import__("sys").stdout.isatty():
        for _ in dir():
            if isinstance(_, str) and _[0] != "_":
                locals()[_] = ""
    else:
        # set Windows console in VT mode
        if __import__("platform").system() == "Windows":
            kernel32 = __import__("ctypes").windll.kernel32
            kernel32.SetConsoleMode(kernel32.GetStdHandle(-11), 7)
            del kernel32


BLUE = Colors.BLUE
GREEN = Colors.GREEN
LIGHT_RED = Colors.LIGHT_RED
RED = Colors.RED

BOLD = Colors.BOLD
FAINT = Colors.FAINT
NEGATIVE = Colors.NEGATIVE
END = Colors.END


def colored(text, *colors):
    return f"{''.join(colors)}{text}{Colors.END}"


def compute_done(*, include_working_copy: bool = False) -> set[YD]:
    # always consider files committed to master
    rust_files = subprocess.run(
        ["git", "ls-tree", "master", "-r", "--name-only", "src"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    if include_working_copy:
        # add whatever is currently checked out
        rust_files += subprocess.run(
            ["find", "src", "-name", "*_*.rs"],
            capture_output=True,
            text=True,
            check=True,
        ).stdout
    pat = re.compile(r".*/y(\d{4})/.*_(\d{2})\.rs")
    return {
        (int(m.group(1)), int(m.group(2)))
        for m in [re.fullmatch(pat, file) for file in rust_files.strip().splitlines()]
        if m
    }


def compute_in_progress(done: set[YD] = None) -> set[YD]:
    if done is None:
        done = compute_done()
    branches = subprocess.run(
        ["git", "branch", "--list", "*/*"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    pat = re.compile(r"(\d{4})/(\d{1,2})")
    return {
        (int(m.group(1)), int(m.group(2)))
        for m in [
            re.fullmatch(pat, branch[2:]) for branch in branches.strip().splitlines()
        ]
        if m
    } - done


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
    return min(25, AOC_NOW.day) if year == AOC_NOW.year else 25


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


def puzzle_name(puzzle):
    name = puzzle.title.lower()
    name = re.sub("'([dst]|ll|re) ", "\\1 ", name)
    name = re.sub("[^a-z0-9]+", "_", name)
    name = name.strip("_")
    if not name[0].isalpha():
        name = "_" + name
    return name


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


def add_day_25(deps: Deps) -> None:
    for y in range(MIN_YEAR, MAX_YEAR + 1):
        deps[(y, 25)] = {(y, d) for d in range(1, 25)}


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


def suggest_next(done: set[YD] = None, in_progress: set[YD] = None) -> YD:
    if done is None:
        done = compute_done()
    if in_progress is None:
        in_progress = compute_in_progress(done)
    # Not Quite Lisp is ALWAYS first!
    if not done:
        return MIN_YEAR, 1
    # this year next!
    if AOC_NOW.year == MAX_YEAR:
        for d in range(last_day_of_year(MAX_YEAR), 0, -1):
            day = MAX_YEAR, d
            if day not in done:
                return day
    total = sum([last_day_of_year(y) for y in range(MIN_YEAR, MAX_YEAR + 1)])
    prev = set(done)
    prev.update(in_progress)
    if len(prev) == total:
        return None
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
            # it's flooded; search backward for an appropriate day
            all_deps = load_deps()
            add_day_25(all_deps)
            satisfied_deps = lambda yd: yd not in all_deps or all(
                [d in done for d in all_deps[yd]]
            )
            rounds.reverse()
            for candidates in rounds:
                for c in filter(satisfied_deps, candidates):
                    return c
            return None
        prev = curr
