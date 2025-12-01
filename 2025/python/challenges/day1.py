def part1(lines: list[str]) -> int:
    rotations = __parse_input(lines)
    password = 0

    dial_location = 50
    for direction, clicks in rotations:
        match direction:
            case "R":
                dial_location += clicks
            case "L":
                dial_location -= clicks

        if (dial_location % 100) == 0:
            password += 1

    return password


def part2(lines: list[str]) -> int:
    rotations = __parse_input(lines)
    password = 0

    dial_location = 50
    for direction, clicks in rotations:
        password += clicks // 100
        clicks = clicks % 100

        match direction:
            case "R":
                next_dial_location = dial_location + clicks
            case "L":
                next_dial_location = dial_location - clicks

        crossed_zero = next_dial_location <= 0 or next_dial_location >= 100
        if dial_location != 0 and crossed_zero:
            password += 1

        dial_location = next_dial_location % 100

    return password


def __parse_input(lines: list[str]) -> list[(str, int)]:
    rotations = []
    for line in lines:
        direction = line[0]
        clicks = int(line[1:])
        rotations.append((direction, clicks))

    return rotations
