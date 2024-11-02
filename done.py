#!/usr/bin/env python
import subprocess

if subprocess.run(["git", "diff", "--exit-code"]).returncode != 0:
    print()
    print("Your working copy is dirty?!")
    print()
    exit(1)

subprocess.run(["cargo", "test"],
               check=True)
branch = subprocess.run(["git", "name-rev", "--name-only", "HEAD"],
                        capture_output=True,
                        text=True,
                        check=True
                        ).stdout.strip()
subprocess.run(["git", "checkout", "master"],
               check=True)
subprocess.run(["git", "merge", "--no-edit", branch],
               check=True)
