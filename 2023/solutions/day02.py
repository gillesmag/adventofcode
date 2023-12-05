import math


def parse(input):
    games = []
    for line in input.strip().split("\n"):
        game_description, colors = line.split(":")
        game_number = int(game_description.split(" ")[1])
        draws = []
        for draw in colors.split(";"):
            colors = {}
            for balls in draw.strip().split(","):
                b = balls.strip().split(" ")
                number = int(b[0])
                color = b[1]
                colors[color] = number
            draws.append(colors)
        games.append((game_number, draws))
    return games


def solution_1(input):
    target = {"red": 12, "green": 13, "blue": 14}
    games = parse(input)
    count = 0
    for gid, game in games:
        game_worked = True
        for s in game:
            matched = {k: (s[k], target[k]) for k in s}
            if not all([v[0] <= v[1] for v in matched.values()]):
                game_worked = False
                break
        if game_worked:
            count += gid
    return count


def solution_2(input):
    games = parse(input)
    total = 0
    for gid, game in games:
        minimum = dict(game[0])
        for s in game[1:]:
            for k in s:
                if k not in minimum:
                    minimum[k] = s[k]
                else:
                    minimum[k] = max(minimum[k], s[k])
        total += math.prod(minimum.values())
    return total


def example_1(input):
    return solution_1(input)


def part_1(input):
    return solution_1(input)


def example_2(input):
    return solution_2(input)


def part_2(input):
    return solution_2(input)
