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
            start_index = last_battery_index + 1
            end_index = len(batteries) - end_buffer
            iteration_battery_index = 0
            highest_battery = "0"

            for i, battery in enumerate(batteries[start_index:end_index]):
                if battery > highest_battery:
                    highest_battery = battery
                    iteration_battery_index = start_index + i

            last_battery_index = iteration_battery_index
            battery_str += highest_battery

        total_output_joltage += int(battery_str)

    return total_output_joltage
