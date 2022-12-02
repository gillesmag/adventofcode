function main()
    #filename = "test.txt"
    filename = "input.txt"

    groups = split(strip(read(open(filename), String)), "\n\n")

    total_count = 0

    for group ∈ groups
        answered_questions = Set(collect(join(split(group, "\n"), "")))

        total_count += length(answered_questions)
        # println(length(answered_questions))
        # println(answered_questions)
    end

    println(total_count)

    total_count = 0

    for group ∈ groups
        answers = intersect(split(group, "\n")...)
        total_count += length(answers)
    end

    println(total_count)
end

main()
