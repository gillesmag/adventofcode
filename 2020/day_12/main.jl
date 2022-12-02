using Test

function rotate(current_direction, instruction)
    all_directions = ['N', 'E', 'S', 'W']
    direction, amount = instruction
    rotation_orientation = direction == 'L' ? -1 : 1
    current_direction_idx = findnext(x -> x == current_direction, all_directions, 1)
    scaled_amount = amount ÷ 90  # angles are multiples of 90 (360 / 4 directions = 90)
    next_dir_idx = (current_direction_idx + rotation_orientation * scaled_amount + 4) % 4
    next_dir_idx = next_dir_idx == 0 ? 4 : next_dir_idx
    return all_directions[next_dir_idx]
end

function tests()
    @test rotate('N', ('L',  90)) == 'W'
    @test rotate('N', ('R',  90)) == 'E'
    @test rotate('N', ('L', 180)) == 'S'
    @test rotate('N', ('R', 180)) == 'S'
    @test rotate('E', ('L',  90)) == 'N'
    @test rotate('E', ('R',  90)) == 'S'
    @test rotate('E', ('L', 180)) == 'W'
    @test rotate('E', ('R', 180)) == 'W'
    @test rotate('S', ('L',  90)) == 'E'
    @test rotate('S', ('R',  90)) == 'W'
    @test rotate('S', ('L', 180)) == 'N'
    @test rotate('S', ('R', 180)) == 'N'
    @test rotate('W', ('L',  90)) == 'S'
    @test rotate('W', ('R',  90)) == 'N'
    @test rotate('W', ('L', 180)) == 'E'
    @test rotate('W', ('R', 180)) == 'E'
end

function part1(instructions)
    current_direction = 'E'
    h_dir, v_dir = 0, 0

    for instruction in instructions
        direction, amount = instruction[1], parse(Int, instruction[2:length(instruction)])

        if direction ∈ ['N', 'S']
            v_dir += amount * (direction == 'N' ? 1 : -1)
        elseif direction ∈ ['E', 'W']
            h_dir += amount * (direction == 'E' ? 1 : -1)
        elseif direction ∈ ['L', 'R']
            current_direction = rotate(current_direction, (direction, amount))
        elseif direction == 'F'
            if current_direction ∈ ['N', 'S']
                v_dir += amount * (current_direction == 'N' ? 1 : -1)
            elseif current_direction ∈ ['E', 'W']
                h_dir += amount * (current_direction == 'E' ? 1 : -1)
            end
        end
    end

    return abs(h_dir) + abs(v_dir)
end

function part2(instructions)
    ship_x, ship_y = 0, 0
    waypoint_x, waypoint_y = 10, 1

    for instruction in instructions
        direction, amount = instruction[1], parse(Int, instruction[2:length(instruction)])

        if direction ∈ ['N', 'S']
            waypoint_y += amount * (direction == 'N' ? 1 : -1)
        elseif direction ∈ ['E', 'W']
            waypoint_x += amount * (direction == 'E' ? 1 : -1)
        elseif direction ∈ ['L', 'R']

            orientation = direction == 'L' ? 1 : -1
            s = Int(round(sin(deg2rad(orientation * amount))))
            c = Int(round(cos(deg2rad(orientation * amount))))

            coeff_matrix = [c -s; s c]
            waypoint_x, waypoint_y = coeff_matrix * [waypoint_x, waypoint_y]

        elseif direction == 'F'
            dx, dy = (waypoint_x, waypoint_y) .* amount
            ship_x, ship_y = (ship_x, ship_y) .+ (dx, dy)
        end
    end

    return abs(ship_x) + abs(ship_y)
end

function main()
    #tests()
    #filename = "test.txt"
    #filename = "test2.txt"
    filename = "input.txt"
    instructions = readlines(filename)

    println(part1(instructions))
    println(part2(instructions))
end

main()
