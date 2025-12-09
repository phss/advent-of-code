def part1(lines: list[str]) -> int:
    tiles = __parse_input(lines)

    max_area = 0
    for i, a in enumerate(tiles[:-1]):
        for b in tiles[i + 1 :]:
            side_x = abs(a[0] - b[0]) + 1
            side_y = abs(a[1] - b[1]) + 1
            area = side_x * side_y
            max_area = max(max_area, area)

    return max_area


def part2(lines: list[str]) -> int:
    None


def __parse_input(lines: list[str]) -> list[tuple[int, int]]:
    tiles = []
    for line in lines:
        a, b = line.split(",")
        tiles.append((int(a), int(b)))
    return tiles
