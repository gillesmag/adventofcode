using Test

function part1(earliest_time, bus_ids)
    next_arrivals = []
    for bus_id ∈ bus_ids
        current_earliest_time = earliest_time
        next_arrival = current_earliest_time // bus_id
        while next_arrival.den != 1
            current_earliest_time += 1
            next_arrival = current_earliest_time // bus_id
        end

        push!(next_arrivals, current_earliest_time)
    end

    earliest_bus = bus_ids[argmin(next_arrivals)]
    earliest_departure = minimum(next_arrivals) - earliest_time
    return earliest_bus * earliest_departure
end

function part2(bus_ids, multiplier = 1)
    ids = []
    offsets = []

    for (idx, bus_id) ∈ enumerate(bus_ids)
        if bus_id != "x"
            push!(ids, parse(Int, bus_id))
            push!(offsets, idx-1)
        end
    end

    would_delete = 0
    for id ∈ ids
        if id ∈ offsets
            idx = findfirst(x -> x == id, offsets)
            would_delete += 1
            #ids[idx] = 7
            # deleteat!(offsets, idx)
            # deleteat!(ids, idx)
        end
    end

    println(would_delete)

    # println(ids)
    # println(offsets)

    while true
        t = ids[1] * multiplier
        found_solution = true
        #println(t, " ", ids[2:length(ids)])
        for (offset_idx, id) ∈ enumerate(ids[2:length(ids)])
            #println(t, " ", offsets[offset_idx+1], " ", id)
            #readline()
            if (t + offsets[offset_idx+1]) % id != 0
                found_solution = false
                break
            end
        end

        if found_solution
            return t
        end

        multiplier += 1
    end
end

function part2_tests()
    f = x -> part2(split(x, ","))
    @test f("17,x,13,19") == 3417
    @test f("67,7,59,61") == 754018
    @test f("67,x,7,59,61") == 779210
    @test f("67,7,x,59,61") == 1261476
    @test f("1789,37,47,1889") == 1202161486
end

function main()
    filename = "test.txt"
    filename = "input.txt"
    lines = readlines(filename)
    earliest_time = parse(Int, lines[1])
    bus_ids = parse.(Int, filter(x -> x != "x", split(lines[2], ",")))

    part2_tests()

    #println(part1(earliest_time, bus_ids))
    bus_ids = split(lines[2], ",")
    println(part2(bus_ids))
    #println(part2(bus_ids, 5882352941177))
end

main()
