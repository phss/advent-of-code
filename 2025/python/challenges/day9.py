from collections import defaultdict
from itertools import pairwise


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
    hlines = __horizontal_lines(tiles)

    max_area = 0
    for i, a in enumerate(tiles[:-1]):
        for b in tiles[i + 1 :]:
            min_x = min(a[0], b[0])
            max_x = max(a[0], b[0])
            min_y = min(a[1], b[1])
            max_y = max(a[1], b[1])
            valid = True

            for p1, p2 in vlines:
                line_x = p1[0]
                line_min_y = p1[1]
                line_max_y = p2[1]

                intersects_on_x = line_x > min_x and line_x < max_x
                out_of_bounds_on_y = line_min_y >= max_y or line_max_y <= min_y

                if intersects_on_x:
                    valid = valid and out_of_bounds_on_y

            for p1, p2 in hlines:
                line_y = p1[1]
                line_min_x = p1[0]
                line_max_x = p2[0]

                intersects_on_y = line_y > min_y and line_y < max_y
                out_of_bounds_on_x = line_min_x >= max_x or line_max_x <= min_x

                if intersects_on_y:
                    valid = valid and out_of_bounds_on_x

            if valid:
                max_area = max(max_area, __area(a, b))

    return max_area


def __vertical_lines(tiles: list[tuple[int, int]]):
    line_group = defaultdict(list)
    for x, y in tiles:
        line_group[x].append(y)
        line_group[x].sort()

    lines = []
    for x, ys in line_group.items():
        for y1, y2 in pairwise(ys):
            lines.append(((x, y1), (x, y2)))

    return lines


def __horizontal_lines(tiles: list[tuple[int, int]]):
    line_group = defaultdict(list)
    for x, y in tiles:
        line_group[y].append(x)
        line_group[y].sort()

    lines = []
    for y, xs in line_group.items():
        for x1, x2 in pairwise(xs):
            lines.append(((x1, y), (x2, y)))

    return lines


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
