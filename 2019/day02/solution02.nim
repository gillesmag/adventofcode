import strutils
import sequtils, sugar

proc parseProgram*(program: string): seq[int] =
  return strutils.split(program, ",").map(x => parseInt(x))

proc execute*(programData: seq[int], resultAt: int = 0): int =
  var programData = programData
  var programCounter = 0
  while programData[programCounter] != 99:
    var opcode = programData[programCounter]
    var leftOperandPointer = programData[programCounter+1]
    var rightOperandPointer = programData[programCounter+2]
    var leftOperand = programData[leftOperandPointer]
    var rightOperand = programData[rightOperandPointer]
    var resultPointer = programData[programCounter+3]
    
    if opcode == 1:
      programData[resultPointer] = leftOperand + rightOperand
    elif opcode == 2:
      programData[resultPointer] = leftOperand * rightOperand

    programCounter += 4
  
  return programData[resultAt]

proc partA(programData: seq[int]): int =
  var programData = programData
  programData[1] = 12
  programData[2] = 2
  return execute(programData)

proc partB(programData: seq[int], targetValue = 19690720): int =
  var programData = programData
  for noun in 0..99:
    for verb in 0..99:
      programData[1] = noun
      programData[2] = verb
      if execute(programData) == targetValue:
        return 100*noun + verb


proc main() =
  let program = readLines("input.txt")[0]
  let programData = parseProgram(program)
  let partAResult = partA(programData)
  let partBResult = partB(programData)
  assert(partAResult == 3101878)
  assert(partBResult == 8444)
  echo "Result for part A: ", partAResult
  echo "Result for part B: ", partBResult

when isMainModule:
  main()
