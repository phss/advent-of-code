def part1(lines: list[str]) -> int:
    id_ranges = __parse_input(lines)
    sum_invalid_ids = 0

    for i in range(1, 100000):
        i_str = str(i)
        invalid_id = int(i_str * 2)

        for id_range in id_ranges:
            if invalid_id in id_range:
                sum_invalid_ids += invalid_id

    return sum_invalid_ids


def part2(lines: list[str]) -> int:
    id_ranges = __parse_input(lines)
    sum_invalid_ids = 0

    invalid_ids = set()
    for i in range(1, 100000):
        for j in range(2, 10):
            candidate = str(i) * j
            if len(candidate) > 10:
                break
            invalid_ids.add(int(candidate))

    for invalid_id in invalid_ids:
        for id_range in id_ranges:
            if invalid_id in id_range:
                sum_invalid_ids += invalid_id

    return sum_invalid_ids


def __parse_input(lines: list[str]) -> list[range[int]]:
    entries = lines[0].split(",")
    id_ranges = []

    for entry in entries:
        a, b = entry.split("-")
        id_range = range(int(a), int(b) + 1)
        id_ranges.append(id_range)

    return id_ranges
