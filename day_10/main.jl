function read_adapter_joltages(filename)
    return [parse(Int, x) for x in split(strip(read(open(filename), String)), "\n")]
end

function count_differences(adapters)
    diffs = Dict()

    diffs[adapters[1] - 0] = 1
    diffs[3] = 1

    for i in range(1, length(adapters)-1, step=1)
        diff = adapters[i+1] - adapters[i]
        if diff ∈ keys(diffs)
            diffs[diff] += 1
        else
            diffs[diff] = 1
        end
    end

    return diffs
end

function binomial_coefficient_sum(target)
    return sum([binomial(target, n) for n in 0:target])
end

function count_useless_adapters(adapters)
    adapters = copy(adapters)
    current_adapter = 0
    max_jump_dist = 3

    insert!(adapters, 1, 0)
    push!(adapters, adapters[length(adapters)]+3)

    #println(adapters)

    junk = []

    idx = 1
    while idx < length(adapters)
        for jump_offset ∈ 3:-1:1
            candidate = current_adapter+jump_offset
            #println(candidate)

            if candidate ∈ adapters
                #println("found ", candidate)
                next_idx = findfirst(x -> x == candidate, adapters)
                #println("next pos ", next_idx)
                for i ∈ idx+1:next_idx-1
                    #println("push ", adapters[i])
                    push!(junk, adapters[i])
                end
                idx = next_idx
                current_adapter = adapters[idx]
                break
            end
        end
    end

    return junk
end

function main()
    #filename = "test1.txt"
    #filename = "test2.txt"
    filename = "input.txt"

    adapters = read_adapter_joltages(filename)
    sort!(adapters)

    differences = count_differences(adapters)
    println(differences[1] * differences[3])

    junk_count = length(count_useless_adapters(adapters))

    #println("junk ", count_useless_adapters(adapters))

    println("junk count ", junk_count)
    println("junk ", count_useless_adapters(adapters))

    println([binomial(junk_count, n) for n in 0:junk_count])

    println(binomial_coefficient_sum(junk_count))
end

main()
