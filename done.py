#!/usr/bin/env python
import subprocess

from status import print_status

if subprocess.run(["git", "diff", "--exit-code"]).returncode != 0:
    print()
    print("Your working copy is dirty?!")
    print()
    exit(1)

subprocess.run(["cargo", "test", "--profile", "release"], check=True)
branch = subprocess.run(
    ["git", "name-rev", "--name-only", "HEAD"],
    capture_output=True,
    text=True,
    check=True,
).stdout.strip()
subprocess.run(["git", "checkout", "master"], check=True)
subprocess.run(["git", "pull"], check=True)
subprocess.run(["git", "merge", "--no-edit", branch], check=True)
subprocess.run(["cargo", "test", "--profile", "release"], check=True)
subprocess.run(["git", "push"], check=True)
print_status(True)
