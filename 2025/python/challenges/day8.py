from collections import Counter
from heapq import heappop, heappush
from math import sqrt


def part1(lines: list[str]) -> int:
    boxes = parse_input(lines)
    return mult_three_largest_circuits(boxes, 1000)


def part2(lines: list[str]) -> int:
    None


def mult_three_largest_circuits(
    boxes: list[tuple[int, int, int]], num_connects: int
) -> int:
    connections = list_shortest_connections(boxes)

    circuits_map = {}
    for i, box in enumerate(boxes):
        circuits_map[box] = i

    for _ in range(num_connects):
        _, (a, b) = heappop(connections)
        a_circuit = circuits_map[a]
        b_circuit = circuits_map[b]
        for box, circuit in circuits_map.items():
            if circuit == b_circuit:
                circuits_map[box] = a_circuit

    counter = Counter(circuits_map.values())
    result = 1
    for _, size in counter.most_common(3):
        result *= size
    return result


def list_shortest_connections(boxes: list[tuple[int, int, int]]) -> list:
    connections = []
    for i, a in enumerate(boxes[:-1]):
        for b in boxes[i + 1 :]:
            distance = sqrt(
                (a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2 + (a[2] - b[2]) ** 2
            )
            heappush(connections, (distance, (a, b)))
    return connections


def parse_input(lines: list[str]) -> list[tuple[int, int, int]]:
    boxes = []
    for line in lines:
        x, y, z = map(int, line.split(","))
        boxes.append((x, y, z))
    return boxes
