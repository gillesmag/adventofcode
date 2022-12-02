function print_grid(matrix)
    ymax, xmax = size(matrix)[1], size(matrix[1])[1]
    for x ∈ 1:1:xmax, y ∈ 1:1:ymax
        print(matrix[x][y])
        if y == ymax
            println()
        end
    end
end

function is_seat(matrix, position)
    return matrix[position[1]][position[2]] != '.'
end

function around(matrix, position)
    empty = fill('.', (3, 3))
    xmax, ymax = size(matrix)[1], size(matrix[1])[1]
    δ = -1:1:1
    for δx ∈ δ, δy ∈ δ
        x, y = position[1] + δx, position[2] + δy
        while 1 <= x <= xmax && 1 <= y <= ymax && (δx != 0 || δy != 0)
            #println(x, " ", y, " ", δx, " ", δy, " ", matrix[x][y])
            if matrix[x][y] != '.'
                empty[δx+2, δy+2] = matrix[x][y]
                break
            end

            x, y = (x, y) .+ (δx, δy)
        end
    end

    return empty
end

function adjacent(matrix, position)
    empty = fill('.', (3, 3))
    xmax, ymax = size(matrix)[1], size(matrix[1])[1]
    δ = -1:1:1
    for δx ∈ δ, δy ∈ δ
        x, y = position[1] + δx, position[2] + δy
        #println(x, " ", y)
        if 1 <= x <= xmax && 1 <= y <= ymax
            if x == position[1] && y == position[2]
                continue
            end
            empty[δx+2, δy+2] = matrix[x][y]
        end
    end
    return empty
end

function part1(seating_grid)
    ymax, xmax = size(seating_grid)[1], size(seating_grid[1])[1]
    previous_seating_grid = deepcopy(seating_grid)

    while true
        updates = []
        for x ∈ 1:1:ymax, y ∈ 1:1:xmax
            if !is_seat(seating_grid, (x, y))
                continue
            end

            position = adjacent(seating_grid, (x, y))
            occupied_adjacent_seats = count("#", join(position, ""))
            #adjacent_empty_seat_count = count("L", join(position, ""))
            if seating_grid[x][y] == 'L' && occupied_adjacent_seats == 0
                push!(updates, (x, y, '#'))
            elseif seating_grid[x][y] == '#' && occupied_adjacent_seats >= 4
                push!(updates, (x, y, 'L'))
            end
        end

        previous_seating_grid = deepcopy(seating_grid)
        # apply updates
        for (x, y, s) ∈ updates
            seating_grid[x][y] = s
        end

        #print_grid(seating_grid)

        if seating_grid == previous_seating_grid
            break
        end
    end

    return count("#", join(join.(seating_grid)))
end

function part2(seating_grid)
    ymax, xmax = size(seating_grid)[1], size(seating_grid[1])[1]
    previous_seating_grid = deepcopy(seating_grid)

    while true
        updates = []
        for x ∈ 1:1:ymax, y ∈ 1:1:xmax
            if !is_seat(seating_grid, (x, y))
                continue
            end

            position = around(seating_grid, (x, y))
            occupied_adjacent_seats = count("#", join(position, ""))
            #adjacent_empty_seat_count = count("L", join(position, ""))
            if seating_grid[x][y] == 'L' && occupied_adjacent_seats == 0
                push!(updates, (x, y, '#'))
            elseif seating_grid[x][y] == '#' && occupied_adjacent_seats >= 5
                push!(updates, (x, y, 'L'))
            end
        end

        previous_seating_grid = deepcopy(seating_grid)
        # apply updates
        for (x, y, s) ∈ updates
            seating_grid[x][y] = s
        end

        #print_grid(seating_grid)

        if seating_grid == previous_seating_grid
            break
        end
    end

    return count("#", join(join.(seating_grid)))
end


function main()
    filename = "input.txt"
    #filename = "test.txt"
    #filename = "distance.txt"
    #filename = "direct.txt"
    #filename = "none.txt"
    seating_grid = collect.(readlines(filename))
    #println(part1(seating_grid))

    # line_sight = around(seating_grid, (5, 4))
    # println(count("#", join(join.(line_sight))))

    # direct
    # line_sight = around(seating_grid, (2, 2))
    # println(count("L", join(join.(line_sight))))

    # none

    #print_grid(seating_grid)
    println(part2(seating_grid))
end

main()
