
function tree_count(grid, step_size)
    x, y = step_size .+ 1

    counter = 0

    while y < length(grid)
        if grid[y][x] == '#'
            counter += 1
        end

        if x + step_size[1] == length(grid[1])
            x = length(grid[1])
        else
            x = (x + step_size[1]) % length(grid[1])
        end

        if x == 0
            x += 1
        end

        y += step_size[2]
    end

    counter
end

parse_grid = filename -> [collect(line) for line in split(read(open(filename), String), "\n")]

function main()
    filename = "problem.txt"
    #filename = "test.txt"
    grid = parse_grid(filename)

    # First star
    println(tree_count(grid, (3, 1)))

    # Second star
    slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ]

    println(prod([tree_count(grid, slope) for slope in slopes]))
end

main()
