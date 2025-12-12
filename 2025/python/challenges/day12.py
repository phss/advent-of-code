def part1(lines: list[str]) -> int:
    _, trees = __parse_input(lines)

    count = 0
    for dimensions, quantities in trees:
        area = (dimensions[0] * dimensions[1]) / 9
        total_quantities = sum(quantities)

        if area >= total_quantities:
            count += 1

    return count


def part2(lines: list[str]) -> int:
    None


def __parse_input(lines: list[str]):
    shapes = []
    for i in range(6):
        start = i * 5
        shapes.append(lines[start + 1 : start + 4])

    trees = []
    for line in lines[30:]:
        a, b = line.split(":")
        dimensions = tuple(map(int, a.split("x")))
        quantities = list(map(int, b.strip().split(" ")))
        trees.append((dimensions, quantities))

    return (shapes, trees)
