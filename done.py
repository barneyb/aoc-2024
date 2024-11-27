#!/usr/bin/env python
import io
import re
import subprocess
from contextlib import redirect_stdout

from lib import current_branch
from status import print_status

if subprocess.run(["git", "diff", "--exit-code"]).returncode != 0:
    print()
    print("Your working copy is dirty?!")
    print()
    exit(1)

branch = current_branch()
if branch == "master":
    print("You're already on master?!")
    exit(2)
subprocess.run(["cargo", "test", "--profile", "release"], check=True)
subprocess.run(["git", "checkout", "master"], check=True)
subprocess.run(["git", "pull"], check=True)
subprocess.run(["git", "merge", "--no-commit", branch], check=True)

f = io.StringIO()
with redirect_stdout(f):
    print_status(include_working_copy=True)
status = f.getvalue()

with open("README.md", "r", encoding="utf-8") as f:
    lines = f.read().splitlines(keepends=True)

out = ""
in_block = False
for l in lines:
    if in_block and l == "</pre>\n":
        # strip color codes and trailing spaces
        out += re.sub("\s+\n", "\n", re.sub("\033[^m]*m", "", status))
        in_block = False
    if not in_block:
        out += l
    if not in_block and l == '<pre id="current-status">\n':
        in_block = True

with open("README.md", "w", encoding="utf-8") as f:
    f.write(out)

subprocess.run(["git", "add", "README.md"], check=True)
subprocess.run(["git", "commit", "--no-edit"], check=True)
subprocess.run(["cargo", "test", "--profile", "release"], check=True)
subprocess.run(["git", "push"], check=True)
print(status)
