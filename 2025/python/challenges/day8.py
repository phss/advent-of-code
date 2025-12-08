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
    for _ in range(num_connects):
        print(heappop(connections))


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
