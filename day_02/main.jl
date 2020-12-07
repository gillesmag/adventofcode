function min_max_policy(password, character, positions)
    char_count = count(c -> c == character, collect(password))

    if positions[1] <= char_count <= positions[2]
        return true
    end

    false
end

function position_policy(password, character, positions)
    already_matched = false

    for pos in positions
        if password[pos] == character
            if already_matched
                return false
            end
            already_matched = true
        end
    end

    already_matched
end

function check_passwords(password_file, policy_function)
    counter = 0

    open(password_file) do file
        for line in eachline(file)
            policy, password = split(line, ": ")
            min_max_count, character = split(policy, " ")
            positions = [parse(Int64, x) for x in split(min_max_count, "-")]

            if policy_function(password, character[1], positions)
                counter += 1
            end
        end
    end

    counter
end

function main()
    check = f -> println(check_passwords("problem.txt", f))

    # First star
    check(min_max_policy)

    # Second star
    check(position_policy)
end

main()
