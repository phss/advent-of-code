from collections import deque


def part1(lines: list[str]) -> int:
    devices = __parse_input(lines)
    search = deque()
    search.appendleft("you")

    path_count = 0
    while search:
        device = search.pop()
        if device == "out":
            path_count += 1
            continue

        for output_devices in devices[device]:
            search.appendleft(output_devices)

    return path_count


def part2(lines: list[str]) -> int:
    None


def __parse_input(lines: list[str]) -> dict[str, list[str]]:
    devices = {}
    for line in lines:
        start, rest = line.split(":")
        dests = rest.strip().split(" ")
        devices[start] = dests

    return devices
