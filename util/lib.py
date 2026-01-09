import os
import re
import subprocess

YD = tuple[int, int]


def compute_done(*, include_working_copy: bool = False) -> set[YD]:
    cwd = os.path.join(os.path.dirname(__file__), "..")
    # always consider files committed to master
    rust_files = subprocess.run(
        ["git", "ls-tree", "master", "-r", "--name-only", "src"],
        cwd=cwd,
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    if include_working_copy:
        # add whatever is currently checked out
        rust_files += subprocess.run(
            ["find", "src", "-name", "*_*.rs"],
            cwd=cwd,
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


def puzzle_name(puzzle):
    name = puzzle.title.lower()
    name = re.sub("'([dst]|ll|re) ", "\\1 ", name)
    name = re.sub("[^a-z0-9]+", "_", name)
    name = name.strip("_")
    if not name[0].isalpha():
        name = "_" + name
    return name
