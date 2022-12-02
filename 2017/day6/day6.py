def spread(banks, block_pos):
    block = banks[block_pos]
    banks[block_pos] = 0

    for i in range(block):
        next_pos = (block_pos+1+i) % len(banks)
        banks[next_pos] += 1

    return tuple(banks)

def main():
    banks = [int(x) for x in open("input.txt").read().split("\t")]
    #banks = [0, 2, 7, 0]
    counter = 0
    encountered_banks = [tuple(banks.copy())]
    distance = 0

    while True:
        max_pos = banks.index(max(banks))
        current_bank = spread(banks, max_pos)
        counter += 1
        #print(encountered_banks)
        if len(set(tuple(encountered_banks))) != len(encountered_banks):
            last_encountered = encountered_banks[-1]
            distance = len(encountered_banks) - encountered_banks.index(last_encountered)
            break
        encountered_banks.append(current_bank)

    print(counter-1)
    print("Same bank seen:", distance-1)


if __name__ == '__main__':
    main()
