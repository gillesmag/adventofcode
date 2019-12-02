import strutils

proc parseProgram(program: string): seq[string] =
  return strutils.split(program, ",")

proc execute(programData: seq[string], resultAt: int = 0): int =
  var programData = programData
  var programCounter = 0
  while programData[programCounter] != "99":
    var opcode = programData[programCounter]
    var leftOperandLocation = parseInt(programData[programCounter+1])
    var leftOperand = parseInt(programData[leftOperandLocation])
    var rightOperandLocation = parseInt(programData[programCounter+2])
    var rightOperand = parseInt(programData[rightOperandLocation])
    var resultLocation = parseInt(programData[programCounter+3])
    var result: int
    
    if opcode == "1":
      result = leftOperand + rightOperand
    elif opcode == "2":
      result = leftOperand * rightOperand
    programData[resultLocation] = intToStr(result)
    programCounter += 4
  
  return parseInt(programData[resultAt])

proc partA(programData: seq[string]): int =
  assert(execute(parseProgram("1,9,10,3,2,3,11,0,99,30,40,50")) == 3500)
  assert(execute(parseProgram("1,0,0,0,99")) == 2)
  assert(execute(parseProgram("2,3,0,3,99"), 3) == 6)
  assert(execute(parseProgram("2,4,4,5,99,0"), 5) == 9801)
  assert(execute(parseProgram("1,1,1,4,99,5,6,0,99")) == 30)

  var programData = programData
  programData[1] = "12"
  programData[2] = "2"
  return execute(programData)

proc partB(programData: seq[string], targetValue = 19690720): int =
  var programData = programData
  for noun in 0..99:
    for verb in 0..99:
      programData[1] = intToStr(noun)
      programData[2] = intToStr(verb)
      if execute(programData) == targetValue:
        return 100*noun + verb


let program = readLines("input.txt")[0]
let programData = parseProgram(program)
let partAResult = partA(programData)
let partBResult = partB(programData)
assert(partAResult == 3101878)
assert(partBResult == 8444)
echo "Result for part A: ", partAResult
echo "Result for part B: ", partBResult
