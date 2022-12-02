function two_sum(numbers, target)
    for i in range(length(numbers)-1, 1, step=-1)
        for j in 1:length(numbers)
            if numbers[i] + numbers[j] > target
                break
            end

            if numbers[i] + numbers[j] == target
                return numbers[i], numbers[j]
            end
        end
    end

    return -1, -1
end

function find_invalid_number(nums, preamble_size)
    numbers = nums[preamble_size+1:length(nums)]
    start_idx, end_idx = 1, preamble_size

    for num_idx in range(preamble_size+1, length(nums), step=1)
        previous = sort(nums[start_idx:end_idx])

        if two_sum(previous, nums[num_idx]) == (-1, -1)
            return num_idx
            break
        end

        start_idx += 1
        end_idx += 1
    end

    return -1
end

function contiguous_sum(numbers, target)
    previous_start = 1
    curr_idx = 1
    total = 0

    while true
        total += numbers[curr_idx]

        if total > target
            previous_start += 1
            curr_idx = previous_start
            total = 0
            continue
        elseif total == target
            break
        end

        curr_idx += 1
    end

    contiguous_range = numbers[previous_start:curr_idx]

    return minimum(contiguous_range)+maximum(contiguous_range)
end

function main()
    #filename = "test.txt"
    #preamble_size = 5

    filename = "input.txt"
    preamble_size = 25

    numbers = [parse(Int, line) for line ∈ split(strip(read(open(filename), String)), "\n")]

    invalid_idx = find_invalid_number(numbers, preamble_size)
    invalid_num = numbers[invalid_idx]

    println(contiguous_sum(numbers, invalid_num))
end

main()
