function binary_space_partitioning(bsp_range, path)
    min_row::Int = minimum(bsp_range)
    max_row::Int = maximum(bsp_range)

    for r in path
        difference = (max_row+1 - min_row) / 2

        front = min_row, min_row+difference
        back = min_row+difference, max_row

        if r == 'U'
            min_row, max_row = front
            max_row -= 1
        elseif r == 'D'
            min_row, max_row = back
        end
    end

    return min_row
end

function compute_seat_id(bsp)
    char_map = Dict(
        "F" => "U",
        "B" => "D",
        "L" => "U",
        "R" => "D",
    )

    for (key, value) ∈ char_map
        bsp = replace(bsp, key => value)
    end

    rows, columns = bsp[1:7], bsp[8:length(bsp)]
    row = binary_space_partitioning(0:127, rows)
    column = binary_space_partitioning(0:7, columns)

    return row * 8 + column
end

function find_remaining_seat(seats)
    for index ∈ range(2, length(seats)-1, step=1)
        seat = seats[index]
        if seats[index-1] != seat-1 || seats[index+1] != seat+1
            return seat+1
        end
    end

    return -1
end

function main()
    #bsp = "FBFBBFFRLR"
    #bsp = "BFFFBBFRRR"
    #bsp = "FFFBBBFRRR"
    #bsp = "BBFFBBFRLL"

    filename = "input.txt"
    seat_positions = split(strip(read(open(filename), String)), "\n")


    seat_ids = [compute_seat_id(pos) for pos ∈ seat_positions]
    sort!(seat_ids)

    # First star: 908
    println(maximum(seat_ids))

    # Second star: 619
    println(find_remaining_seat(seat_ids))
end

main()
