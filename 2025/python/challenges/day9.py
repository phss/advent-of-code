from collections import defaultdict
from itertools import pairwise

X = 0
Y = 1


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
            if __is_valid(vlines, a, b, X, Y) and __is_valid(hlines, a, b, Y, X):
                max_area = max(max_area, __area(a, b))

    return max_area


def __is_valid(
    lines: list[tuple[int, int]],
    a: tuple[int, int],
    b: tuple[int, int],
    main_axis: int,
    other_axis: int,
):
    min_main_axis = min(a[main_axis], b[main_axis])
    max_main_axis = max(a[main_axis], b[main_axis])
    min_other_axis = min(a[other_axis], b[other_axis])
    max_other_axis = max(a[other_axis], b[other_axis])

    for p1, p2 in lines:
        line_main_axis = p1[main_axis]
        line_min_other_axis = p1[other_axis]
        line_max_other_axis = p2[other_axis]

        intersects_on_main_axis = (
            line_main_axis > min_main_axis and line_main_axis < max_main_axis
        )
        intersects_on_other_axis = not (
            line_min_other_axis >= max_other_axis
            or line_max_other_axis <= min_other_axis
        )

        if intersects_on_main_axis and intersects_on_other_axis:
            return False

    return True


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
