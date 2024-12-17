#!/usr/bin/env python3
"""Create a tarball of intermediate output for inspection if tests fail.

This is useful for seeing what exactly `ctest` is running.
"""

import os
import subprocess as sp
import sys
from datetime import datetime, timezone
from glob import glob, iglob
from pathlib import Path


def main():
    """The main function"""
    # Find the most recently touched file named "main.c" in the target
    # directory. This will be libc-tests's `OUT_DIR`
    build_dir = min(
        (Path(p) for p in iglob("target/**/main.c", recursive=True)),
        key=lambda path: path.stat().st_mtime,
    ).parent
    print(f"Located build directory '{build_dir}'")

    # Collect all relevant Rust and C files
    add_files = glob("**/*.rs", recursive=True, root_dir=build_dir)
    add_files += glob("**/*.c", recursive=True, root_dir=build_dir)
    file_list = "\n".join(add_files).encode()

    now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H%MZ")
    archive_name = f"archive-{now}"
    if len(sys.argv) > 1:
        archive_name += f"-{sys.argv[1]}"
    archive_path = f"{archive_name}.tar.gz"

    sp.run(
        ["tar", "czvf", archive_path, "-C", build_dir, "-T-"],
        input=file_list,
        check=False,
    )

    # If we are in GHA, set these env vars for future use
    gh_env = os.getenv("GITHUB_ENV")
    if gh_env is not None:
        print(f"""Updating CI environment with
ARCHIVE_NAME={archive_name}
ARCHIVE_PATH={archive_path}""")

        with open(gh_env, "w+", encoding="utf-8") as ghenv:
            ghenv.write(f"ARCHIVE_NAME={archive_name}\n")
            ghenv.write(f"ARCHIVE_PATH={archive_path}\n")


if __name__ == "__main__":
    print(' '.join(sys.argv))
    # FIXME(ci): remove after the bump to windows-2025 GHA images
    # Python <= 3.9 does not support the very helpful `root_dir` argument,
    # and that is the version used by the Windows GHA images. Rather than
    # using setup-python or doing something in the CI script, just find
    # the newer version and relaunch if this happens to be run with an old
    # version.
    try:
        glob("", root_dir="")
    except TypeError:
        if os.environ.get("CI") is None:
            sys.exit(1)

        # Find the next 3.1x Python version
        dirs = sorted(Path(r"C:\hostedtoolcache\windows\Python").iterdir())
        usepy = next(x for x in dirs if r"\3.1" in str(x))
        py = usepy.joinpath(r"x64\python.exe")
        print(f"relaunching with {py}")
        os.execvp(py, [__file__] + sys.argv)

    main()
