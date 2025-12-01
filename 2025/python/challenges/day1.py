def part1(lines: list[str]) -> int:
    password = 0

    dial_location = 50
    for line in lines:
        direction = line[0]
        clicks = int(line[1:])

        match direction:
            case "R":
                dial_location += clicks
            case "L":
                dial_location -= clicks

        if (dial_location % 100) == 0:
            password += 1

    return password


def part2() -> int:
    None
