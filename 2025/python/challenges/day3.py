def part1(lines: list[str]) -> int:
    return __calculate_total_output_joltage(lines, 2)


def part2(lines: list[str]) -> int:
    return __calculate_total_output_joltage(lines, 12)


def __calculate_total_output_joltage(
    battery_banks: list[str], num_batteries: int
) -> int:
    total_output_joltage = 0

    for batteries in battery_banks:
        battery_str = ""
        last_battery_index = -1

        for end_buffer in reversed(range(num_batteries)):
            highest_battery = "0"
            blah = 0
            for i, battery in enumerate(
                batteries[(last_battery_index + 1) : (len(batteries) - end_buffer)]
            ):
                if battery > highest_battery:
                    highest_battery = battery
                    blah = last_battery_index + 1 + i
            last_battery_index = blah
            battery_str += highest_battery

        total_output_joltage += int(battery_str)

    return total_output_joltage


def __parse_input(lines: list[str]) -> list[list[int]]:
    battery_banks = []

    for line in lines:
        batteries = list(map(int, line))
        battery_banks.append(batteries)

    return battery_banks
