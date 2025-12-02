def part1(lines: list[str]) -> int:
    id_ranges = __parse_input(lines)
    sum_invalid_ids = 0

    for i in range(1, 100000):
        i_str = str(i)
        invalid_id = int(i_str + i_str)

        for id_range in id_ranges:
            if invalid_id in id_range:
                sum_invalid_ids += invalid_id

    return sum_invalid_ids


def part2(lines: list[str]) -> int:
    None


def __parse_input(lines: list[str]) -> list[range[int]]:
    entries = lines[0].split(",")
    id_ranges = []

    for entry in entries:
        a, b = entry.split("-")
        id_range = range(int(a), int(b) + 1)
        id_ranges.append(id_range)

    return id_ranges
