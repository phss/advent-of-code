import challenges.day5 as challenge


class TestChallenge:
    lines = [
        "3-5",
        "10-14",
        "16-20",
        "12-18",
        "",
        "1",
        "5",
        "8",
        "11",
        "17",
        "32",
    ]

    def test_part1(self):
        result = challenge.part1(self.lines)

        assert result == 3

    def test_part2(self):
        result = challenge.part2(self.lines)

        assert result is None
