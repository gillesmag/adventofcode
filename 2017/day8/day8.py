test_input = """b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"""

MEMORY = {}


def parse_subinstruction(s):
    operands = ["", ""]
    (operands[0], operation, operands[1]) = s.split(" ")
    try:
        operands[1] = int(operands[1])
    except ValueError:
        pass
    return (tuple(operands), operation)


def parse_instructions(s):
    instructions = []
    for i in s:
        instr, cond = i.split(" if ")
        instr = parse_subinstruction(instr)
        cond = parse_subinstruction(cond)
        instructions.append((instr, cond))
    return instructions


def var_check(variable):
    """ Checks if a variable is in MEMORY and returns it's value,
        if not, creates it and sets its default value to 0. """
    if MEMORY.get(variable) == None:
        MEMORY[variable] = 0
        return 0
    else:
        return MEMORY[variable]

def eval_condition(cond):
    var = cond[0][0]
    val = var_check(var)
    operation = cond[1]
    return eval("{} {} {}".format(val, operation, cond[0][1]))


def execute(instructions):
    highest_value = 0
    for pair in instructions:
        instr, cond = pair
        if eval_condition(cond):
            var = instr[0][0]
            var_check(var)
            operation = instr[1]
            if operation == "inc":
                MEMORY[var] += instr[0][1]
            elif operation == "dec":
                MEMORY[var] -= instr[0][1]
        new_value = max(MEMORY.values())
        if new_value > highest_value:
            highest_value = new_value
    return highest_value



def main():
    code = [line.strip() for line in open("input.txt")]
    instructions = parse_instructions(code)
    highest_value = execute(instructions)
    print("Lowest value in memory:", max(MEMORY.values()))
    print("Highest value held in memory:", highest_value)

if __name__ == '__main__':
    main()
