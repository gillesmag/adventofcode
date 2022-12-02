def main():
    frequencies = [
        int(freq) for freq in open("input.txt").read().strip().split("\n")
    ]
    frequency = sum(frequencies)
    print(frequency)

    running_sum = 0
    # note using sets is way more efficient (both memory and time)
    # than using normal lists, why?
    running_frequency_sums = [0]
    running_frequency_sums = set([0])
    has_encountered_number_twice = False
    while not has_encountered_number_twice:
        for i, freq_var in enumerate(frequencies):
            running_sum += freq_var
            # print(running_sum)
            if running_sum in running_frequency_sums:
                print(running_sum)
                has_encountered_number_twice = True
                break
            # running_frequency_sums.append(running_sum)
            running_frequency_sums.add(running_sum)


if __name__ == "__main__":
    main()
