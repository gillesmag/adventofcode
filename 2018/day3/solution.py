import itertools
import re


def parse_input(filename):
    f = open(filename).read().strip().split("\n")
    p = re.compile(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")
    results = []

    for line in f:
        result = p.match(line)
        results.append(tuple(int(r) for r in result.groups()))

    return results


def max_claim(claims):
    max_left, max_top = 0, 0
    max_width, max_height = 0, 0

    for claim in claims:
        _, left, top, width, height = claim
        if left > max_left:
            max_left = left

        if top > max_top:
            max_top = top

        if width > max_width:
            max_width = width

        if height > max_height:
            max_height = height

    return max_left, max_top, max_width, max_height


def print_max_fabric(fab):
    for i, x in enumerate(fab):
        for j, _ in enumerate(x):
            print(fab[i][j], end="")
        print()

def main():
    claims = parse_input("input.txt")
    # claims = parse_input("test.in")
    max_left, max_top, max_width, max_height = max_claim(claims)
    width, height = max_left + max_width, max_top + max_height
    max_fabric = [["." for _ in range(width)] for _ in range(height)]
    print(width, height)
    # print_max_fabric(max_fabric)
    # print(claims)
    square_inches_overlapping = 0
    for claim in claims:
        id, left, top, width, height = claim
        for i, row in enumerate(max_fabric[top:top+height]):
            new_row = []
            for j, column in enumerate(row):
                char = column
                if j in range(left, left+width):
                    if column == ".":
                        char = id
                    elif column != "X":
                        square_inches_overlapping += 1
                        char = "X"
                new_row.append(char)
            max_fabric[i+top] = new_row
    # print_max_fabric(max_fabric)
    print(square_inches_overlapping)

    non_overlapping_claim_id = None
    special_chars = ["X", "."]
    for claim in claims:
        id, left, top, width, height = claim
        claimed = []
        for row in max_fabric[top:top+height]:
            claimed.append(row[left:left+width])
        claimed_chars = set(itertools.chain.from_iterable(claimed))
        # print_max_fabric(claimed)
        # print()

        if len(claimed_chars) == 1:
            if claimed_chars.pop() not in special_chars:
                non_overlapping_claim_id = id
                break
    print(non_overlapping_claim_id)


if __name__ == "__main__":
    main()
