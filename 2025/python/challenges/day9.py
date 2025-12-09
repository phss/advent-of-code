from collections import defaultdict
from itertools import pairwise


def part1(lines: list[str]) -> int:
    tiles = __parse_input(lines)

    max_area = 0
    for i, a in enumerate(tiles[:-1]):
        for b in tiles[i + 1 :]:
            rectangle = (a, b)
            max_area = max(max_area, __area(rectangle))

    return max_area


def part2(input_lines: list[str]) -> int:
    tiles = __parse_input(input_lines)
    lines = __to_lines(tiles)

    max_area = 0
    for i, a in enumerate(tiles[:-1]):
        for b in tiles[i + 1 :]:
            rectangle = (a, b)

            valid_rectangle = True
            for line in lines:
                if __intersects(line, rectangle):
                    valid_rectangle = False
                    break

            if valid_rectangle:
                max_area = max(max_area, __area(rectangle))

    return max_area


def __intersects(
    line: tuple[tuple[int, int], tuple[int, int]],
    rectangle: tuple[tuple[int, int], tuple[int, int]],
):
    x, y = 0, 1

    def intersects_on(axis):
        line_min = min(line[0][axis], line[1][axis])
        line_max = max(line[0][axis], line[1][axis])
        rectangle_min = min(rectangle[0][axis], rectangle[1][axis])
        rectangle_max = max(rectangle[0][axis], rectangle[1][axis])

        return line_max > rectangle_min and line_min < rectangle_max

    return intersects_on(x) and intersects_on(y)


def __to_lines(
    tiles: list[tuple[int, int]],
) -> list[tuple[tuple[int, int], tuple[int, int]]]:
    lines = list(pairwise(tiles))
    lines.append((tiles[-1], tiles[0]))
    return lines


def __area(rectangle: tuple[tuple[int, int], tuple[int, int]]) -> int:
    a, b = rectangle
    vertical_side = abs(a[0] - b[0]) + 1
    horizontal_side = abs(a[1] - b[1]) + 1
    return vertical_side * horizontal_side


def __parse_input(lines: list[str]) -> list[tuple[int, int]]:
    tiles = []
    for line in lines:
        a, b = line.split(",")
        tiles.append((int(a), int(b)))
    return tiles
