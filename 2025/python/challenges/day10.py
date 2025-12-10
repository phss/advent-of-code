import re


def part1(lines: list[str]) -> int:
    machines = __parse_input(lines)

    for machine in machines:
        print(machine)


def part2(lines: list[str]) -> int:
    None


def __parse_input(lines: list[str]) -> list:
    lights_pattern = re.compile(r"\[(.*)\]")
    buttons_pattern = re.compile(r"\((.*?)\)+")
    joltage_pattern = re.compile(r"\{(.*)\}")

    machines = []
    for line in lines:
        lights = lights_pattern.findall(line)[0]
        buttons = [
            list(map(int, button.split(",")))
            for button in buttons_pattern.findall(line)
        ]
        joltage = list(map(int, joltage_pattern.findall(line)[0].split(",")))
        machines.append((lights, buttons, joltage))

    return machines
