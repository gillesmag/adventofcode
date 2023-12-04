import re


def solution(input, skip_spelled=False):
    number_mapping = {
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
    }
    total = 0
    for line in input.strip().splitlines():
        digit_indices = [(idx, c) for idx, c in enumerate(line) if c.isdigit()]
        spelled_indices = (
            [
                (match.start(), str(number_mapping[num]))
                for num in number_mapping.keys()
                for match in re.finditer(num, line)
            ]
            if not skip_spelled
            else []
        )

        combined = sorted(digit_indices + spelled_indices)
        result = int(combined[0][1] + combined[-1][1])
        total += result
    return total


def part_1(input):
    return solution(input, skip_spelled=True)


def example_1(input):
    return solution(input, skip_spelled=True)


def example_2(input):
    return solution(input)


def part_2(input):
    return solution(input)
