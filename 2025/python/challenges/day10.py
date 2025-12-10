from collections import deque
import re

import z3


def part1(lines: list[str]) -> int:
    machines = __parse_input(lines)

    total_button_presses = 0
    for machine in machines:
        lights, buttons, _ = machine
        total_button_presses += __min_button_presses_for_lights(lights, buttons)

    return total_button_presses


def part2(lines: list[str]) -> int:
    machines = __parse_input(lines)

    total_button_presses = 0
    for machine in machines:
        _, buttons, joltage = machine
        total_button_presses += __min_button_presses_for_joltage(joltage, buttons)

    return total_button_presses


def __min_button_presses_for_lights(end_lights: str, buttons: list[list[int]]) -> int:
    search = deque()
    search.append((0, len(end_lights) * "."))

    while True:
        presses, lights = search.popleft()
        if lights == end_lights:
            return presses

        for button in buttons:
            lights_after_button = list(lights)
            for i in button:
                if lights_after_button[i] == ".":
                    lights_after_button[i] = "#"
                else:
                    lights_after_button[i] = "."
            search.append((presses + 1, "".join(lights_after_button)))


def __min_button_presses_for_joltage(
    end_joltage: list[int], buttons: list[list[int]]
) -> int:
    solver = z3.Optimize()
    variables = [z3.Int(f"x{i}") for i, _ in enumerate(buttons)]

    for variable in variables:
        solver.add(variable >= 0)

    for joltage_index, joltage in enumerate(end_joltage):
        relevant_variables = [
            variable
            for variable, button_indexes in zip(variables, buttons)
            if joltage_index in button_indexes
        ]

        solver.add(joltage == z3.Sum(relevant_variables))

    total_presses = solver.minimize(sum(variables))
    if solver.check() == z3.sat:
        return total_presses.value().as_long()
    else:
        raise "couldn't solve it"


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
