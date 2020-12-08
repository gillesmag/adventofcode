struct Instruction
    opcode::String
    operand::Int
end

function run_program(instructions)
    accumulator = 0
    program_counter = 1

    executed_instructions = Set()

    while true
        if program_counter > length(instructions)
            return accumulator, program_counter
        end
        instr = instructions[program_counter]
        if program_counter ∈ executed_instructions
            break
        end
        push!(executed_instructions, program_counter)
        if instr.opcode == "acc"
            accumulator += instr.operand
        elseif instr.opcode == "jmp"
            program_counter += instr.operand
            continue
        end

        program_counter += 1
    end

    return accumulator, program_counter
end

function mutate_instructions(position, instructions)
    instr = instructions[position]
    if instr.opcode == "jmp"
        instructions[position] = Instruction("nop", instr.operand)
    elseif instr.opcode == "nop"
        instructions[position] = Instruction("jmp", instr.operand)
    end
end

function main()
    #filename = "test.txt"
    filename = "input.txt"

    raw_instructions = split(strip(read(open(filename), String)), "\n")

    instructions = []
    for instr in raw_instructions
        opcode = instr[1:3]
        operand = replace(instr[5:length(instr)], "+" => "")

        append!(instructions, [Instruction(opcode, parse(Int, operand))])
    end

    acc, _ = run_program(instructions)
    println(acc)

    for (index, instr) in enumerate(instructions)
        if instr.opcode == "acc"
            continue
        end

        new_instructions = copy(instructions)
        mutate_instructions(index, new_instructions)

        acc, pc = run_program(new_instructions)
        if pc-1 == length(instructions)
            println(acc)
        end
    end

    # println(instructions)
    # println(mutate_instructions(1, instructions))
    # println(instructions)

    #println(length(instructions))
end

main()
