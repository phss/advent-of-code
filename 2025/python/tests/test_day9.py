import challenges.day9 as challenge


class TestChallenge:
    lines = [
        "7,1",
        "11,1",
        "11,7",
        "9,7",
        "9,5",
        "2,5",
        "2,3",
        "7,3",
    ]

    def test_part1(self):
        result = challenge.part1(self.lines)

        assert result == 50

    def test_part2(self):
        result = challenge.part2(self.lines)

        assert result is None
