import argparse
from os import path
from pathlib import Path
import shutil


def run_challenge(challenge: str, part: int):
    print(f"Running {challenge}, part {part}")

    with open(f"data/{challenge}.txt", "r") as file:
        lines = [line.rstrip() for line in file]

    challenge = __import__(f"challenges.{challenge}", fromlist=["part1", "part2"])
    match part:
        case 1:
            print(challenge.part1(lines))
        case 2:
            print(challenge.part2(lines))
        case _:
            print("No such part")


def setup_challenge(challenge: str):
    setup_files = {
        "challenges/template.py": f"challenges/{challenge}.py",
        "tests/test_template.py": f"tests/test_{challenge}.py",
        "data/template.txt": f"data/{challenge}.txt",
    }

    for src, dest in setup_files.items():
        if not path.isfile(dest):
            print(f"Copying {src} to {dest}")
            shutil.copyfile(src, dest)
            file = Path(dest)
            file.write_text(file.read_text().replace("template", challenge))


def main():
    parser = argparse.ArgumentParser(prog="Advent of Code 2025")
    parser.add_argument("challenge")
    parser.add_argument("--mode", default="run")
    parser.add_argument("--part", type=int)
    args = parser.parse_args()

    match args.mode:
        case "run":
            run_challenge(args.challenge, args.part)
        case "setup":
            setup_challenge(args.challenge)
        case _:
            print(f"Unsupported mode: {args.mode}")


if __name__ == "__main__":
    main()
