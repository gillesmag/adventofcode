def main():
    jumps = [int(x) for x in open("input.txt").read().strip().split("\n")]
    #jumps = [0, 3, 0, 1, -3]

    step_counter = 0
    prev_pos, next_pos = 0, 0
    while True:
        try:
            # jump with offset
            next_pos += jumps[prev_pos]
            if (next_pos - prev_pos) >= 3:
                jumps[prev_pos] -= 1
            else:
                jumps[prev_pos] += 1

            prev_pos = next_pos
            #print(jumps, next_pos+1)
            step_counter += 1
        except IndexError:
            break

    print("Number of steps required:", step_counter)

    # Output solution: 375042



if __name__ == '__main__':
    main()
