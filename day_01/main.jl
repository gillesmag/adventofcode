function parse_lines(filename)
    numbers = []

    open(filename) do file
        for l in eachline(file)
            x = parse(Int, l)
            append!(numbers, x)
        end
    end

    sort(numbers)
end

function two_element_sum(numbers, target)
    stop = false
    terms = []

    for i in range(length(numbers)-1, 1, step=-1)
        for j in 1:length(numbers)
            if numbers[i] + numbers[j] > target
                break
            end

            if numbers[i] + numbers[j] == target
                append!(terms, numbers[i])
                append!(terms, numbers[j])
                stop = true
            end
        end

        if stop
            break
        end
    end

    terms
end

function three_element_sum(numbers, target)
    stop = false
    terms = []

    for i in range(length(numbers)-1, 1, step=-1)
        for j in 1:length(numbers)
            for k in 2:i
                if i == j == k
                    continue
                end

                if numbers[i] + numbers[j] + numbers[k] > target
                    break
                end

                if numbers[i] + numbers[j] + numbers[k] == target
                    append!(terms, numbers[i])
                    append!(terms, numbers[j])
                    append!(terms, numbers[k])
                    global stop = true
                end
            end

            if stop
                break
            end
        end

        if stop
            break
        end
    end

    terms
end

function main()
    problem_file = "problem.txt"

    solve = f -> println(prod(f(parse_lines(problem_file), 2020)))

    # First star
    solve(two_element_sum)
    #@assert result == 145875, "First star result is wrong"

    # Second star
    solve(three_element_sum)
    #@assert result == 69596112, "Second star result is wrong"
end

main()
