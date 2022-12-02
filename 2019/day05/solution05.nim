import strutils
import sequtils, sugar

type
  Opcode* = enum
    addOp = 1, multiplyOp = 2, readOp = 3, writeOp = 4, haltOp = 99
  ParameterMode* = enum
    position = 0, immediate = 1
  Instruction = tuple
    opcode: Opcode
    parameterModes: seq[ParameterMode]

proc parseProgram*(program: string): seq[int] =
  return strutils.split(program, ",").map(x => parseInt(x))

proc parseInstruction*(instr: int): Instruction =
  var instr = instr
  result.opcode = Opcode(instr mod 100)
  instr = instr div 100

  case result.opcode:
  of addOp, multiplyOp:
    for _ in 1..2:
      let parameterMode = instr mod 10
      instr = instr div 10

      case parameterMode:
      of 0: result.parameterModes.add(position)
      of 1: result.parameterModes.add(immediate)
      else: discard
    result.parameterModes.add(immediate)
  of readOp, writeOp: result.parameterModes.add(immediate)#position)
  of haltOp: discard

proc execute*(programData: seq[int], resultAt: int = 0): int =
  var programData = programData
  var programCounter = 0
  var i = 0
  while programData[programCounter] != 99 and programCounter < programData.len:
    let instruction = parseInstruction(programData[programCounter])
    var parameters = newSeq[int]()

    for paramOffset, paramMode in instruction.parameterModes.pairs:
      let parameterValue = programData[programCounter+paramOffset+1]
      # Last parameter (write) is never in position (indirect access) mode
      # echo instruction.parameterModes
      # if instruction.opcode == readOp or instruction.opcode == writeOp:
      #   parameters.add(parameterValue)
      #   continue
      # if paramOffset == 2:
      #   parameters.add(parameterValue)
      case instruction.parameterModes[paramOffset]:
      of position: parameters.add(programData[parameterValue])
      of immediate: parameters.add(parameterValue)

    # echo programData[programCounter], " ", instruction, " ", parameters
    case instruction.opcode:
    of addOp: programData[parameters[2]] = parameters[0] + parameters[1]
    of multiplyOp: programData[parameters[2]] = parameters[0] * parameters[1]
    of readOp: programData[parameters[0]] = parseInt(readLine(stdin))
    of writeOp: echo programData[parameters[0]]
    of haltOp: return programData[resultAt]

    programCounter += 1+instruction.parameterModes.len
  
  return programData[resultAt]

proc partA() =
  let program = readLines("input.txt")[0]
  let programData = parseProgram(program)
  discard execute(programData)
  
proc main() =
  # let program = readLines("../day02/input.txt")[0]
  # var programData = parseProgram(program)
  # programData[1] = 12
  # programData[2] = 2
  # echo execute(programData) == 3101878

  partA()
  # echo parseInstruction(1002)
  # echo execute(parseProgram("1002,4,3,4,33"), 4) == 99
  # echo execute(parseProgram("3,1,99"), resultAt = 1)
  # echo execute(parseProgram("4,3,99,123"))
  # let partAResult = partA(programData)
  # let partBResult = partB(programData)
  # assert(partAResult == 3101878)
  # assert(partBResult == 8444)
  # echo "Result for part A: ", partAResult
  # echo "Result for part B: ", partBResult

  # echo execute(parseProgram("1002,4,3,4,33"), 4)
  # assert(execute(parseProgram("1101,100,-1,4,0"), 4) == 99)
  
when isMainModule:
  main()
