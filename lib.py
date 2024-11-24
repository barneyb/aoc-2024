import datetime
import json
import os
import subprocess
from doctest import master
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


def current_branch():
    return subprocess.run(
        ["git", "name-rev", "--name-only", "HEAD"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout.strip()


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


def last_day_of_year(year):
    return min(25, aoc_now.day) if year == aoc_now.year else 25
