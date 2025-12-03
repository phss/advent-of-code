import challenges.day3 as challenge


class TestChallenge:
    lines = ["987654321111111", "811111111111119", "234234234234278", "818181911112111"]

    def test_part1(self):
        result = challenge.part1(self.lines)

        assert result == 357

    def test_part2(self):
        result = challenge.part2(self.lines)

        assert result == 3121910778619
