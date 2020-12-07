function main()
    #filename = "test.txt"
    filename = "input.txt"

    lines = split(strip(read(open(filename), String)), "\n")

    bags = Dict()
    for line in lines
        key, value = split(strip(line, ['.']), " contain ")
        key = replace(key, "bags" => "bag")

        values = [
            (
                replace(string(v)[3:length(v)], "bags" => "bag"),
                parse(Int, string(v)[1])
            )
            for v in split(value, ", ") if isdigit(v[1])
        ]
        bags[key] = Dict(values)
    end

    # Part 1
    counter = 0
    for key in keys(bags)
        if contains_shiny_gold(key, bags)
            counter += 1
        end
    end

    println(counter)


    # Part 2
    println(total_bag_count("shiny gold bag", bags))
    # println(total_bag_count(
    #     "shiny gold bag", Dict(
    #         "shiny gold bag" => Dict("dark red bag" => 2),
    #         "dark red bag" => Dict("dark orange bag" => 2),
    #         "dark orange bag" => Dict("dark yellow bag" => 2),
    #         "dark yellow bag" => Dict("dark green bag" => 2),
    #         "dark green bag" => Dict("dark blue bag" => 2),
    #         "dark blue bag" => Dict("dark violet bag" => 2),
    #         "dark violet bag" => Dict(),
    #     ),
    # ))
end

function total_bag_count(key, bags)
    if length(bags[key]) == 0
        return 0
    end

    total = 0
    for (subbag, count) ∈ bags[key]
        total += count
        total += count * total_bag_count(subbag, bags)
    end

    return total
end

function contains_shiny_gold(key, bags)
    found_shiny_gold = false

    if length(keys(bags[key])) == 0
        found_shiny_gold = false
    elseif "shiny gold bag" ∈ keys(bags[key])
        found_shiny_gold = true
    else
        found_shiny_gold = false

        for subkey in keys(bags[key])
            if contains_shiny_gold(subkey, bags)
                found_shiny_gold = true
                break
            end
        end
    end

    return found_shiny_gold
end

main()
