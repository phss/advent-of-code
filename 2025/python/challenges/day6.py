import re


def part1(lines: list[str]) -> int:
    (all_numbers, operations) = __parse_input(lines)
    sum_of_answers = 0

    for i, operation in enumerate(operations):
        answer = all_numbers[0][i]
        for numbers in all_numbers[1:]:
            match operation:
                case "+":
                    answer += numbers[i]
                case "*":
                    answer *= numbers[i]

        sum_of_answers += answer

    return sum_of_answers


def part2(lines: list[str]) -> int:
    None


def __parse_input(lines: list[str]) -> list:
    all_numbers = [list(map(int, __list_from(line))) for line in lines[:-1]]
    operations = __list_from(lines[-1])
    return (all_numbers, operations)


def __list_from(line: str) -> list:
    sanitised_string = re.sub(r"\s+", " ", line.strip())
    return sanitised_string.split(" ")
