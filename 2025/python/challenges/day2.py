def part1(lines: list[str]) -> int:
    id_ranges = __parse_input(lines)
    invalid_ids = __generate_possible_invalid_ids(2)
    return __sum_of_invalid_ids_found(id_ranges, invalid_ids)


def part2(lines: list[str]) -> int:
    id_ranges = __parse_input(lines)
    invalid_ids = __generate_possible_invalid_ids(9)
    return __sum_of_invalid_ids_found(id_ranges, invalid_ids)


def __parse_input(lines: list[str]) -> list[range[int]]:
    entries = lines[0].split(",")
    id_ranges = []

    for entry in entries:
        a, b = entry.split("-")
        id_range = range(int(a), int(b) + 1)
        id_ranges.append(id_range)

    return id_ranges


def __generate_possible_invalid_ids(max_repetitions: int) -> set[int]:
    invalid_ids = set()

    for i in range(1, 100000):
        for j in range(2, max_repetitions + 1):
            candidate = str(i) * j
            if len(candidate) > 10:
                break
            invalid_ids.add(int(candidate))

    return invalid_ids


def __sum_of_invalid_ids_found(
    id_ranges: list[range[int]], invalid_ids: set[int]
) -> int:
    sum_invalid_ids = 0

    for invalid_id in invalid_ids:
        for id_range in id_ranges:
            if invalid_id in id_range:
                sum_invalid_ids += invalid_id

    return sum_invalid_ids
