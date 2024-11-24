import datetime
import json
import os
from zoneinfo import ZoneInfo

YD = (int, int)
Deps = dict[YD, set[YD]]

AOC_TZ = ZoneInfo("America/New_York")
aoc_now = datetime.datetime.now(tz=AOC_TZ)
MIN_YEAR = 2015
MAX_YEAR = aoc_now.year if aoc_now.month == 12 else aoc_now.year - 1
DEPS_FILE = ".deps.json"


def load_deps() -> Deps:
    if not os.path.isfile(DEPS_FILE):
        return dict()
    with open(DEPS_FILE, "r", encoding="utf-8") as f:
        return dict(
            [(tuple(d), {tuple(d) for d in ds}) for [d, ds] in json.load(f) if ds]
        )


def save_deps(deps: Deps):
    with open(DEPS_FILE, "w", encoding="utf-8") as f:
        json.dump([(d, sorted(list(ds))) for d, ds in sorted(deps.items()) if ds], f)


def last_day_of_year(year):
    return min(25, aoc_now.day) if year == aoc_now.year else 25
