#!/usr/bin/env python3
import argparse
import pathlib
import shutil
import subprocess
import sys


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build the bare-metal AArch64 Rust kernel")
    parser.add_argument("--rustc", required=True)
    parser.add_argument("--source", required=True)
    parser.add_argument("--linker", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument("--cfg", action="append", default=[])
    parser.add_argument("--debug-symbols", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()

    source = pathlib.Path(args.source).resolve()
    linker = pathlib.Path(args.linker).resolve()
    output = pathlib.Path(args.output).resolve()
    output.parent.mkdir(parents=True, exist_ok=True)

    rustc = shutil.which(args.rustc)
    if rustc is None:
        print(
            "error: rustc was not found. Install Rust and add the aarch64-unknown-none target:\n"
            "  rustup target add aarch64-unknown-none",
            file=sys.stderr,
        )
        return 1

    command = [
        rustc,
        "--edition=2021",
        "--crate-name",
        "kernel",
        "--crate-type",
        "bin",
        "-C",
        "opt-level=z",
        "-C",
        "panic=abort",
        "-C",
        "relocation-model=static",
        "-C",
        f"link-arg=-T{linker}",
        "--target",
        "aarch64-unknown-none",
        "-C",
        "target-feature=+vhe",
    ]

    if args.debug_symbols:
        command.extend([
            "-C",
            "debuginfo=2",
            "-C",
            "force-frame-pointers=yes",
        ])

    for cfg in args.cfg:
        command.extend(["--cfg", cfg])

    command.extend([
        str(source),
        "-o",
        str(output),
    ])

    print(" ".join(command))
    subprocess.run(command, check=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
