def part1(lines: list[str]) -> int:
    max_x = len(lines[0])
    max_y = len(lines)
    accessible_rolls = 0

    directions = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    for y in range(max_y):
        for x in range(max_x):
            if lines[y][x] == ".":
                continue

            count = 0
            for dir_x, dir_y in directions:
                check_x = x + dir_x
                check_y = y + dir_y

                if (
                    check_x >= 0
                    and check_x < max_x
                    and check_y >= 0
                    and check_y < max_y
                    and lines[check_y][check_x] == "@"
                ):
                    count += 1

            if count < 4:
                accessible_rolls += 1

    return accessible_rolls


def part2(lines: list[str]) -> int:
    None
