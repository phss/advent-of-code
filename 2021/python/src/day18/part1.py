# String operations
def fromString(numberStr):
    return eval(numberStr)

def toString(number):
    if type(number) == int:
        return str(number)
    else:
        left, right = number
        return "[%s,%s]" % (toString(left), toString(right))

# Tree operations
def addToLeftmost(number, toAdd):
    if type(number) == int:
        return number + toAdd
    else:
        left, right = number
        return [addToLeftmost(left, toAdd), right]

def addToRightmost(number, toAdd):
    if type(number) == int:
        return number + toAdd
    else:
        left, right = number
        return [left, addToRightmost(right, toAdd)]


def explode(number, level=0):
    if type(number) == int:
        return (number, (False, [None, None]))
    elif level == 4:
        return (0, (True, number))

    left, right = number

    left, (leftExploded, (toLeft, toRight)) = explode(left, level + 1)
    if leftExploded:
        return ([left, addToLeftmost(right, toRight)], (True, [toLeft, 0]))

    right, (rightExploded, (toLeft, toRight)) = explode(right, level + 1)
    if rightExploded:
        return ([addToRightmost(left, toLeft), right], (True, [0, toRight]))

    return ([left, right], (False, [None, None]))


def split(number):
    if type(number) == int:
        if number <= 9:
            return (number, (False, None))
        else:
            return ([number//2, number//2 + number%2], (True, number))

    left, right = number
    left, (splited, old) = split(left)
    if splited:
        return ([left, right], (True, old))

    right, result = split(right)
    return ([left, right], result)


def reduce(number):
    reducing = True
    while reducing:
        number, (exploded, _) = explode(number)
        if exploded:
            continue
        number, (splited, _) = split(number)
        reducing = exploded or splited
    return number

def add(a, b):
    return reduce([a, b])

def magnitude(number):
    if type(number) == int:
        return number
    left, right = number
    return 3*magnitude(left) + 2*magnitude(right)

# Core
def parse(filename: str):
    with open(filename) as file:
        fileLines = file.read().split('\n')
        return [fromString(line) for line in fileLines]

def solve(input) -> int:
    result = input[0]
    for number in input[1:]:
        result = add(result, number)
    print(toString(result))
    return magnitude(result)

def main():
    input = parse("data/day18.txt")
    result = solve(input)
    print(result)

if __name__ == '__main__':
    main()