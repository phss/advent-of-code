from collections import Counter


def part1(map: list[str]) -> int:
    split_count = 0
    beams = set()
    beams.add(__find_start(map))

    for row in map[1:]:
        new_beams = set()
        for x, y in beams:
            if row[x] == "^":
                split_count += 1
                new_beams.add((x - 1, y + 1))
                new_beams.add((x + 1, y + 1))
            else:
                new_beams.add((x, y + 1))

        beams = new_beams

    return split_count


def part2(map: list[str]) -> int:
    timelines = 1
    x, y = __find_start(map)
    beams = set()
    beams.add((x, y, timelines))

    for row in map[1:]:
        new_beams = Counter()
        for x, y, t in beams:
            if row[x] == "^":
                timelines += t
                new_beams[(x - 1, y + 1)] += t
                new_beams[(x + 1, y + 1)] += t
            else:
                new_beams[(x, y + 1)] += t

        beams = set()
        for (x, y), t in new_beams.items():
            beams.add((x, y, t))

    return timelines


def __find_start(map: list[str]) -> tuple[int, int]:
    y = 0
    x = map[y].index("S")
    return (x, y)
