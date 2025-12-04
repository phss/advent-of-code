def part1(lines: list[str]) -> int:
    rolls = __rolls_to_remove(lines)
    return len(rolls)


def part2(lines: list[str]) -> int:
    total_rolls_removed = 0

    while True:
        rolls = __rolls_to_remove(lines)
        removed = len(rolls)
        total_rolls_removed += removed

        if removed == 0:
            break

        for x, y in rolls:
            lines[y] = lines[y][:x] + "x" + lines[y][x + 1 :]

    return total_rolls_removed


def __rolls_to_remove(lines: list[str]) -> list[(int, int)]:
    max_x = len(lines[0])
    max_y = len(lines)
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
            if lines[y][x] != "@":
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
                rolls_to_remove.append((x, y))

    return rolls_to_remove
