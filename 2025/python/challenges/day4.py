def part1(map: list[str]) -> int:
    rolls = __rolls_to_remove(map)
    return len(rolls)


def part2(lines: list[str]) -> int:
    None


def __rolls_to_remove(map: list[str]) -> list[(int, int)]:
    max_x = len(map[0])
    max_y = len(map)
    rolls_to_remove = []

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
            if map[y][x] == ".":
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
                    and map[check_y][check_x] == "@"
                ):
                    count += 1

            if count < 4:
                rolls_to_remove.append((x, y))

    return rolls_to_remove
