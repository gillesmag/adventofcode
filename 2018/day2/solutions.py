def main():
    ids = open("input.txt").read().strip().split("\n")
    # ids = [
    #     "abcdef",
    #     "bababc",
    #     "abbcde",
    #     "abcccd",
    #     "aabcdd",
    #     "abcdee",
    #     "ababab"
    # ]

    # ids = [
    #     "abcde",
    #     "fghij",
    #     "klmno",
    #     "pqrst",
    #     "fguij",
    #     "axcye",
    #     "wvxyz"
    # ]

    unique_letter_ids = [set(i) for i in ids]
    letter_count = {}

    twos_count = 0
    threes_count = 0

    for i, id in enumerate(ids):
        letter_count[id] = {}
        twos_counted = False
        threes_counted = False
        for letter in unique_letter_ids[i]:
            count = id.count(letter)
            if count == 2 and not twos_counted:
                twos_count += 1
                twos_counted = True
            elif count == 3 and not threes_counted:
                threes_count += 1
                threes_counted = True
            letter_count[id][letter] = count

    print("{} * {} = {}".format(
        twos_count, threes_count, twos_count*threes_count
    ))

    one_distance_pair = None
    for i, first_id in enumerate(ids):
        for second_id in ids[i+1:]:
            distance = 0
            edit_point = 0
            for j, c in enumerate(first_id):
                if c != second_id[j]:
                    distance += 1
                    edit_point = j
            if distance == 1:
                one_distance_pair = (first_id, second_id, edit_point)
                break
        else:
            continue
        break

    first, second, edit_point = one_distance_pair
    print(first[:edit_point] + first[edit_point+1:])


if __name__ == "__main__":
    main()
