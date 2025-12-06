import challenges.day6 as challenge


class TestChallenge:
    lines = [
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "    98  215 314",
        "*   +   *   +  ",
    ]

    def test_part1(self):
        result = challenge.part1(self.lines)

        assert result is None

    def test_part2(self):
        result = challenge.part2(self.lines)

        assert result is None
