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
    (fresh_ingredients, _) = __parse_input(lines)
    merged_ingredients = __merge_ingredients_range(fresh_ingredients)

    fresh_ingredients_count = 0
    for fresh_ingredient_range in merged_ingredients:
        fresh_ingredients_count += len(fresh_ingredient_range)

    return fresh_ingredients_count


def __merge_ingredients_range(ingredients_range):
    ingredients_range = sorted(ingredients_range, key=lambda x: x.start)
    merged = [ingredients_range[0]]

    for current in ingredients_range[1:]:
        last = merged[-1]
        if current.start <= last.stop:
            merged[-1] = range(last.start, max(last.stop, current.stop))
        else:
            merged.append(current)

    return merged


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
