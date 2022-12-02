from collections import Counter


def read_input(filename):
    return list(open(filename).read().strip())


def can_react(a, b):
    if a.lower() != b.lower():
        return False

    if a.islower():
        if b.isupper():
            return True
    else:
        if b.islower():
            return True
    return False


def faster_remaining_units(polymer):
    # TODO: implement substitution with incremental backoff
    i = 0
    while i < len(polymer)-1:
        first, second = polymer[i:i+2]
        if can_react(first, second):
            polymer.pop(i+1)
            polymer.pop(i)
            i = i-2
            if i < 0:
                i = 0
        i += 1

    return len(polymer)


def remaining_units(polymer):
    i = 0
    while i < len(polymer)-1:
        first, second = polymer[i:i+2]
        if can_react(first, second):
            polymer.pop(i+1)
            polymer.pop(i)
            i = 0
        i += 1

    return len(polymer)


def best_polymer(polymer):
    characters = set("".join(polymer).lower())
    counter = Counter()

    for char in characters:
        edited_polymer = "".join(polymer).replace(char, "").replace(char.upper(), "")
        # counter[char] = remaining_units(list(edited_polymer))
        counter[char] = faster_remaining_units(list(edited_polymer))

    print(counter.most_common()[-1][1])



def main():
    # polymer = read_input("test.in")
    polymer = read_input("input.txt")

    # print(remaining_units(polymer))
    print(faster_remaining_units(polymer))
    best_polymer(polymer)



if __name__ == "__main__":
    main()
