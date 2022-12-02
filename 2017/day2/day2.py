def checksum(f):
    diff_sum = 0
    for line in f:
        nums = [int(i) for i in line.split("\t")]
        diff_sum += max(nums) - min(nums)

    return diff_sum

def checksum2(f):
    chksum = 0
    for line in f:
        nums = sorted([int(i) for i in line.split("\t")])

        for pos_divisor, curr_divisor in enumerate(nums):
            for curr_pos, num in enumerate(nums[pos_divisor+1:]):
                if (num % curr_divisor == 0) and pos_divisor != curr_pos:
                    chksum += num // curr_divisor
    return chksum


def main():
    test_input = open("input.txt")
    print(checksum(test_input))
    test_input.seek(0)
    print(checksum2(test_input))

if __name__ == '__main__':
    main()
