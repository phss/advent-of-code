from collections import defaultdict
from heapq import heappop, heappush


def part1(lines: list[str]) -> int:
    tiles = __parse_input(lines)

    max_area = 0
    for i, a in enumerate(tiles[:-1]):
        for b in tiles[i + 1 :]:
            max_area = max(max_area, __area(a, b))

    return max_area


def part2(lines: list[str]) -> int:
    tiles = __parse_input(lines)
    vlines = __vertical_lines(tiles)

    max_area = 0
    for x, (low_y, high_y) in vlines:
        min_y = low_y
        max_y = high_y

        for curr_x, (low_curr_y, high_curr_y) in vlines:
            if curr_x < x:
                None
            elif curr_x > x:
                if low_y <= low_curr_y and low_curr_y <= high_y:
                    max_area = max(max_area, __area((x, low_y), (curr_x, low_curr_y)))
                if low_y <= high_curr_y and high_curr_y <= high_y:
                    max_area = max(max_area, __area((x, low_y), (curr_x, high_curr_y)))
                if high_y >= low_curr_y and low_curr_y >= low_y:
                    max_area = max(max_area, __area((x, high_y), (curr_x, low_curr_y)))
                if high_y >= high_curr_y and high_curr_y >= low_y:
                    max_area = max(max_area, __area((x, high_y), (curr_x, high_curr_y)))

    return max_area


def __vertical_lines(tiles: list[tuple[int, int]]):
    lines = defaultdict(list)
    for x, y in tiles:
        lines[x].append(y)
        lines[x].sort()

    return sorted(lines.items())


def __area(a: tuple[int, int], b: tuple[int, int]) -> int:
    vertical_side = abs(a[0] - b[0]) + 1
    horizontal_side = abs(a[1] - b[1]) + 1
    return vertical_side * horizontal_side


def __parse_input(lines: list[str]) -> list[tuple[int, int]]:
    tiles = []
    for line in lines:
        a, b = line.split(",")
        tiles.append((int(a), int(b)))
    return tiles
