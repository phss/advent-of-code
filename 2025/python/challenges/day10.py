from heapq import heappop, heappush
import re


def part1(lines: list[str]) -> int:
    machines = __parse_input(lines)

    total_button_presses = 0
    for machine in machines:
        lights, buttons, _ = machine
        total_button_presses += __min_button_presses(lights, buttons)
        print(machine, total_button_presses)

    return total_button_presses


def part2(lines: list[str]) -> int:
    None


def __min_button_presses(end_lights: str, buttons: list[list[int]]) -> int:
    search = []
    heappush(search, (0, len(end_lights) * "."))

    while True:
        presses, lights = heappop(search)  # noqa: F821
        if lights == end_lights:
            return presses

        for button in buttons:
            lights_after_button = list(lights)
            for i in button:
                if lights_after_button[i] == ".":
                    lights_after_button[i] = "#"
                else:
                    lights_after_button[i] = "."
            heappush(search, (presses + 1, "".join(lights_after_button)))


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
