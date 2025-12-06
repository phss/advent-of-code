import re


def part1(lines: list[str]) -> int:
    (all_numbers, operations) = __parse_human_input(lines)
    return __calculate_worsheet_answers(all_numbers, operations)


def part2(lines: list[str]) -> int:
    (all_numbers, operations) = __parse_cephalopod_input(lines)
    return __calculate_worsheet_answers(all_numbers, operations)


def __calculate_worsheet_answers(
    all_numbers: list[list[int]], operations: list[str]
) -> int:
    sum_of_answers = 0

    for i, operation in enumerate(operations):
        answer = all_numbers[i][0]
        for number in all_numbers[i][1:]:
            match operation:
                case "+":
                    answer += number
                case "*":
                    answer *= number

        sum_of_answers += answer

    return sum_of_answers


def __parse_human_input(lines: list[str]) -> list:
    all_numbers = [list(map(int, __list_from(line))) for line in lines[:-1]]
    all_numbers = [[row[i] for row in all_numbers] for i in range(len(all_numbers[0]))]
    operations = __list_from(lines[-1])
    return (all_numbers, operations)


def __parse_cephalopod_input(lines: list[str]) -> list:
    all_numbers = []
    numbers = []
    for i in range(len(lines[0])):
        number_str = [line[i] for line in lines[:-1]]
        number_str = "".join(number_str).strip()
        if number_str == "":
            all_numbers.append(numbers)
            numbers = []
        else:
            numbers.append(int(number_str))
    all_numbers.append(numbers)

    operations = __list_from(lines[-1])
    return (all_numbers, operations)


def __list_from(line: str) -> list:
    sanitised_string = re.sub(r"\s+", " ", line.strip())
    return sanitised_string.split(" ")
