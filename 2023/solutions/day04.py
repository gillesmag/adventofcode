import re


def parse_cards(input):
    cards = {}
    for line in input.splitlines():
        card_number = int(line.split(":")[0].split(" ")[-1])
        winning_numbers, my_numbers = [
            set([int(n.strip()) for n in re.sub(" +", " ", arr).strip().split(" ")])
            for arr in line.split(":")[1].split("|")
        ]
        cards[card_number] = (winning_numbers, my_numbers)
    return cards


def solution(input):
    total = 0
    cards = parse_cards(input)
    for winning_numbers, my_numbers in cards.values():
        r = len(my_numbers.intersection(winning_numbers))
        if r == 0:
            continue
        total += 2 ** (r - 1)

    return total


def solution_2(input):
    cards = list(parse_cards(input).items())
    stack = [v for v in cards]
    card_count = {}
    while len(stack) > 0:
        card_number, (winning_numbers, my_numbers) = stack.pop()
        wins = len(my_numbers.intersection(winning_numbers))
        if card_number not in card_count:
            card_count[card_number] = 0
        card_count[card_number] += 1
        stack.extend([cards[card] for card in range(card_number, card_number + wins)])

    return sum(card_count.values())


def part_1(input):
    return solution(input)


def part_2(input):
    return solution_2(input)


def example_1(input):
    return solution(input)


def example_2(input):
    return solution_2(input)
