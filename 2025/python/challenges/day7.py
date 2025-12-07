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
    None


def __find_start(map: list[str]) -> tuple[int, int]:
    y = 0
    x = map[y].index("S")
    return (x, y)
