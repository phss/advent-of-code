import challenges.day1 as challenge


class TestTemplate:
    lines = [
        "L68",
        "L30",
        "R48",
        "L5",
        "R60",
        "L55",
        "L1",
        "L99",
        "R14",
        "L82",
    ]

    def test_part1(self):
        result = challenge.part1(self.lines)

        assert result == 3

    def test_part2(self):
        result = challenge.part2(self.lines)

        assert result == 6

    def test_part2_mega_rotation(self):
        lines = [
            "L68",
            "L30",
            "R48",
            "L5",
            "R60",
            "R1100",
            "L55",
            "L1",
            "L99",
            "L1000",
            "R14",
            "L82",
        ]

        result = challenge.part2(lines)

        assert result == 27
