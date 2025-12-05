def part1(lines: list[str]) -> int:
    (fresh_ingredients, ingredients) = __parse_input(lines)
    fresh_ingredients_count = 0

    for ingredient in ingredients:
        for fresh_ingredient_range in fresh_ingredients:
            if ingredient in fresh_ingredient_range:
                fresh_ingredients_count += 1
                break

    return fresh_ingredients_count


def part2(lines: list[str]) -> int:
    None


def __parse_input(lines: list[str]) -> tuple[list[range[int]], list[int]]:
    separator_index = lines.index("")
    fresh_ingredients = []
    for line in lines[:separator_index]:
        start, end = line.split("-")
        ingredient_range = range(int(start), int(end) + 1)
        fresh_ingredients.append(ingredient_range)

    ingredients = []
    for line in lines[separator_index + 1 :]:
        ingredients.append(int(line))

    return (fresh_ingredients, ingredients)
