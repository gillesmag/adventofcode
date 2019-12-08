import unittest
from ../intcode import parseInstruction, Opcode, ParameterMode

suite "Intcode interpreter":
  let opCodeMapping = [
    (num: 1, opcode: addOp),
    (num: 2, opcode: multiplyOp),
    (num: 3, opcode: readOp),
    (num: 4, opcode: writeOp),
    (num: 99, opcode: haltOp)
  ]
  test "Parse instruction":
    for mapping in opCodeMapping:
      check parseInstruction(mapping.num).opcode == mapping.opcode
    check parseInstruction(01101).parameterModes == @[immediate, immediate, immediate]
    check parseInstruction(1002).parameterModes == @[position, immediate, immediate]
    check parseInstruction(3).parameterModes == @[immediate]
