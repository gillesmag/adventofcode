import math
import re

def solution(input, together=False):
    numbers = [[int(x) for x in re.sub(" +", " " if not together else "", line.split(":")[1]).strip().split(" ")] for line in input.splitlines()]
    time_distances = list(zip(numbers[0], numbers[1]))
    return math.prod([len([x for x in [(t-y)*y > d for y in range(t)] if x]) for t, d in time_distances])

def example_1(input):
    return solution(input)

def part_1(input):
    return solution(input)

def example_2(input):
    return solution(input, True)

def part_2(input):
    return solution(input, True)
