struct MaskInstruction
    value::String
end

struct StoreInstruction
    address::Int
    value::Int
end

function to_bit_array(num::Int64)
    bits = zeros(Int, 36)
    bit_idx = 36
    while num != 0
        bits[bit_idx] = num % 2
        num >>= 1
        bit_idx -= 1
        if bit_idx < 0
            break
        end
    end
    return bits
end

function to_int(num::Array{Int})
    size = length(num)
    reconstructed_number = 0
    for position ∈ 1:size
        reconstructed_number += num[size+1-position] * 2^(position-1)
    end
    return reconstructed_number
end

function apply_mask(mask::MaskInstruction, value::Int)
    bit_value = to_bit_array(value)
    for (idx, mask_bit) ∈ enumerate(mask.value)
        if mask_bit ∈ ['0', '1']
            bit_value[idx] = parse(Int, mask_bit)
        end
    end

    return to_int(bit_value)
end

function parse_program(instructions::Array{String})
    parsed_instructions = []
    for instruction ∈ instructions
        if occursin("mask", instruction)
            push!(
                parsed_instructions,
                MaskInstruction(split(instruction, " = ")[2])
            )
        else
            caps = match(r"mem\[(\d+)\] = (\d+)", instruction).captures
            address, value = parse.(Int, caps)
            push!(
                parsed_instructions,
                StoreInstruction(
                    address, value
                )
            )
        end
    end
    return parsed_instructions
end

function binary_strings(length::Int)
    numbers = []
    for i ∈ 1:(1 << length)
        bits = []
        for j ∈ (length-1):-1:0
            push!(bits, ((1 << j)&i) > 0 ? 1 : 0)
        end
        push!(numbers, bits)
    end
    return numbers
end

function floating_positions(mask::MaskInstruction)
    floating_positions = findall(x -> x == 'X', mask.value)



end

function compute_addresses(mask::MaskInstruction, address::Int)
    mask = mask.value
    address_bits = to_bit_array(address)
    decoded_addresses = []

    for bit_pos ∈ findall(x -> x == '1', mask)
        address_bits[bit_pos] = 1
    end

    floating_positions = findall(x -> x == 'X', mask)
    bin_strings = binary_strings(length(floating_positions))

    for bin_string ∈ bin_strings
        floating_address = copy(address_bits)
        bin_idx = 1
        for pos ∈ floating_positions
            floating_address[pos] = bin_string[bin_idx]
            bin_idx += 1
        end

        push!(
            decoded_addresses,
            to_int(floating_address)
        )
    end

    return decoded_addresses
end

function run_program(instructions, decoder_version = 1)
    current_mask = instructions[1]
    memory = Dict()

    for instruction ∈ instructions[2:length(instructions)]
        if instruction isa MaskInstruction
            current_mask = instruction
        else
            if decoder_version == 1
                result = apply_mask(current_mask, instruction.value)
                memory[instruction.address] = result
            elseif decoder_version == 2
                decoded_addresses = compute_addresses(current_mask, instruction.address)
                for address ∈ decoded_addresses
                    memory[address] = instruction.value
                end
            else
                error("Memory decoder version unknown")
            end
        end
    end

    return sum(values(memory))
end


function main()
    #filename = "test.txt"
    #filename = "test2.txt"
    filename = "input.txt"
    instructions = parse_program(readlines(filename))
    run = v -> println(run_program(instructions, v))

    #run(1)

    run(2)
end

main()
