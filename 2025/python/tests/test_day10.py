import challenges.day10 as challenge


class TestChallenge:
    lines = [
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    ]

    def test_part1(self):
        result = challenge.part1(self.lines)

        assert result == 7

    def test_part2(self):
        result = challenge.part2(self.lines)

        assert result is None
