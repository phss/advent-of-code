import challenges.day7 as challenge


class TestChallenge:
    lines = [
        ".......S.......",
        "...............",
        ".......^.......",
        "...............",
        "......^.^......",
        "...............",
        ".....^.^.^.....",
        "...............",
        "....^.^...^....",
        "...............",
        "...^.^...^.^...",
        "...............",
        "..^...^.....^..",
        "...............",
        ".^.^.^.^.^...^.",
        "...............",
    ]

    def test_part1(self):
        result = challenge.part1(self.lines)

        assert result == 21

    def test_part2(self):
        result = challenge.part2(self.lines)

        assert result is None
