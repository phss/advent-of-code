import re


def part1(lines: list[str]) -> int:
    (all_numbers, operations) = __parse_human_input(lines)
    return __calculate_worsheet_answers(all_numbers, operations)


def part2(lines: list[str]) -> int:
    None


def __calculate_worsheet_answers(
    all_numbers: list[list[int]], operations: list[str]
) -> int:
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


def __parse_human_input(lines: list[str]) -> list:
    def __list_from(line: str) -> list:
        sanitised_string = re.sub(r"\s+", " ", line.strip())
        return sanitised_string.split(" ")

    all_numbers = [list(map(int, __list_from(line))) for line in lines[:-1]]
    operations = __list_from(lines[-1])
    return (all_numbers, operations)
