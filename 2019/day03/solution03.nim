import strutils
import sequtils
import sugar

type
  Position = tuple
    x: int
    y: int
  Direction = enum
    up,down,left,right
  LinePart = tuple
    direction: Direction
    amount: int
  Line = tuple
    startPos: Position
    endPos: Position

proc isHorizontal(line: Line): bool =
  if line.startPos.y == line.endPos.y:
    return true
  else:
    return false

proc manhattanDistance(pos: Position): int =
  return abs(pos.x) + abs(pos.y)
    
proc parseWireSpecification(line: string): seq[LinePart] =
  for part in strutils.split(line, ","):
    var direction: Direction
    case part[0]:
    of 'U': direction = up
    of 'D': direction = down
    of 'L': direction = left
    of 'R': direction = right
    else:
        direction = down
    result.add((direction: direction, amount: parseInt(part[1..<part.len])))

proc computeLineSegments(spec: seq[LinePart]): seq[Line] =
  var currentPos = (x: 0, y: 0)
  for part in spec:
    var newX = currentPos.x
    var newY = currentPos.y
    case part.direction:
    of up: newY = currentPos.y + part.amount
    of down: newY = currentPos.y - part.amount
    of left: newX = currentPos.x - part.amount
    of right: newX = currentPos.x + part.amount
    var newPos = (x: newX, y: newY)
    result.add((startPos: currentPos, endPos: newPos))
    currentPos = newPos

proc computeIntersectionPoint(hline, vline: Line): Position =
  # Take maximum and minimum of horizontal line, since horizontal line
  # has same y values for start and end positions, we can use it to
  # compare to vline.
  var minX = min(hline.startPos.x, hline.endPos.x)
  var maxX = max(hline.startPos.x, hline.endPos.x)

  # Again, take minimum and maximum of vertical line, since vertical
  # line has the same x values for start and end positions, we can use
  # it to compare to hline.
  var minY = min(vline.startPos.y, vline.endPos.y)
  var maxY = max(vline.startPos.y, vline.endPos.y)

  if minX < vline.startPos.x and vline.startPos.x < maxX:
    if minY < hline.startPos.y and hline.startPos.y < maxY:
      return (x: vline.startPos.x, y: hline.startPos.y)
  return (x: 0, y: 0)

proc computeIntersections(hlines, vlines: seq[Line]): seq[Position] =
  for hline in hlines:
    for vline in vlines:
      let p = computeIntersectionPoint(hline, vline)
      if p != (x: 0, y: 0):
        result.add(p)
    
proc computeLineIntersections(wireOne, wireTwo: seq[Line]): Position =
  var wireOneHorizontalLines = wireOne.filter(x => isHorizontal(x))
  var wireOneVerticalLines = wireOne.filter(x => not isHorizontal(x))
  var selfIntersections = computeIntersections(wireOneHorizontalLines, wireOneVerticalLines)
  
  var wireTwoHorizontalLines = wireTwo.filter(x => isHorizontal(x))
  var wireTwoVerticalLines = wireTwo.filter(x => not isHorizontal(x))
  selfIntersections = selfIntersections & computeIntersections(wireTwoHorizontalLines, wireTwoVerticalLines)
  
  var horizontalLines = wireOneHorizontalLines & wireTwoHorizontalLines
  var verticalLines = wireOneVerticalLines & wireTwoVerticalLines
  var intersectionPoints = computeIntersections(horizontalLines, verticalLines)
  
  for interPoint in selfIntersections:
    let index = find(intersectionPoints, interPoint)
    if index != -1:
      del(intersectionPoints, index)

  var smallestPos = intersectionPoints[0]
  for pos in intersectionPoints[1..<intersectionPoints.len]:
    if manhattanDistance(pos) < manhattanDistance(smallestPos):
      smallestPos = pos
  return smallestPos

proc computeClosestIntersection(lineSpec1, lineSpec2: string): int=
  var wireOne = parseWireSpecification(lineSpec1)
  var wireTwo = parseWireSpecification(lineSpec2)

  var segmentsOne = computeLineSegments(wireOne)
  var segmentsTwo = computeLineSegments(wireTwo)

  return manhattanDistance(computeLineIntersections(segmentsOne, segmentsTwo))

proc partA() =
  assert(
    computeClosestIntersection(
      "R8,U5,L5,D3",
      "U7,R6,D4,L4"
    ) == 6
  )

  assert(
    computeClosestIntersection(
      "R75,D30,R83,U83,L12,D49,R71,U7,L72",
      "U62,R66,U55,R34,D71,R55,D58,R83"
    ) == 159
  )

  assert(
    computeClosestIntersection(
      "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
      "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
    ) == 135
  )

  var lines = newSeq[string]()
  for line in "input.txt".lines:
    lines.add(line)
  var firstLine = lines[0]
  var secondLine = lines[1]
  let answer = computeClosestIntersection(firstLine, secondLine)
  assert(answer == 209)  
  
proc main() =
  partA()


when isMainModule:
  main()

assert(isHorizontal((startPos: (x:0, y:0), endPos: (x: 10, y: 0))) == true)
assert(isHorizontal((startPos: (x:10, y:0), endPos: (x: 10, y: 5))) == false)
