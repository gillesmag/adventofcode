from strutils import parseFloat
import math

proc computeFuelMass(mass: float): float =
  return math.floor(mass / 3) - 2

proc partA(filename: string): float =
  for line in filename.lines:
    result += computeFuelMass(parseFloat(line))

proc partB(filename: string): float =
  for line in filename.lines:
    var fuelMass = computeFuelMass(parseFloat(line))
    while fuelMass > 0:
      result += fuelMass
      fuelMass = computeFuelMass(fuelMass)

var result = partA("input-a.txt")
assert(result == 3324332.0, "Wrong solution for the first star")

result = partB("input-b.txt")
assert(result == 4983626.0, "Wrong solution for the second star")
