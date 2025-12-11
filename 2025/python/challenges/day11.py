from collections import Counter, defaultdict, deque


def part1(lines: list[str]) -> int:
    devices = __parse_input(lines)
    return __paths_between(devices, "you", "out")


def part2(lines: list[str]) -> int:
    devices = __parse_input(lines)
    path_counts = 1
    path_counts *= __paths_between(devices, "svr", "fft")
    path_counts *= __paths_between(devices, "fft", "dac")
    path_counts *= __paths_between(devices, "dac", "out")
    return path_counts


def __paths_between(devices: dict[str, list[str]], start: str, end: str):
    search = {start: 1}
    visits = Counter()

    while len(search) > 0:
        new_search = Counter()
        for device, count in search.items():
            if device not in devices:
                continue

            for output_device in devices[device]:
                new_search[output_device] += count
                visits[output_device] += count
        search = new_search

    return visits[end]


def __parse_input(lines: list[str]) -> dict[str, list[str]]:
    devices = {}
    for line in lines:
        start, rest = line.split(":")
        dests = rest.strip().split(" ")
        devices[start] = dests

    return devices
