import challenges.day11 as challenge


class TestChallenge:
    lines = [
        "aaa: you hhh",
        "you: bbb ccc",
        "bbb: ddd eee",
        "ccc: ddd eee fff",
        "ddd: ggg",
        "eee: out",
        "fff: out",
        "ggg: out",
        "hhh: ccc fff iii",
        "iii: out",
    ]

    def test_part1(self):
        result = challenge.part1(self.lines)

        assert result is None

    def test_part2(self):
        result = challenge.part2(self.lines)

        assert result is None
