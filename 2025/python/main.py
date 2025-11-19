import argparse


def run_challenge(challenge: str, part: int):
    challenge = __import__(f"challenges.{challenge}", fromlist=["part1", "part2"])

    match part:
        case 1:
            challenge.part1()
        case 2:
            challenge.part2()
        case _:
            print("No such part")


def main():
    parser = argparse.ArgumentParser(prog="Advent of Code 2025")
    parser.add_argument("challenge")
    parser.add_argument("--part", type=int)
    args = parser.parse_args()

    run_challenge(args.challenge, args.part)


if __name__ == "__main__":
    main()
