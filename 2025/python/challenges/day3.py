def part1(lines: list[str]) -> int:
    battery_banks = __parse_input(lines)
    total_output_joltage = 0

    for batteries in battery_banks:
        left_battery = 0
        for i, battery in enumerate(batteries[:-1]):
            if battery > left_battery:
                left_battery = battery
                left_battery_index = i

        right_battery = 0
        for battery in batteries[(left_battery_index + 1) :]:
            if battery > right_battery:
                right_battery = battery

        total_output_joltage += left_battery * 10 + right_battery

    return total_output_joltage


def part2(lines: list[str]) -> int:
    None


def __parse_input(lines: list[str]) -> list[list[int]]:
    battery_banks = []

    for line in lines:
        batteries = list(map(int, line))
        battery_banks.append(batteries)

    return battery_banks
