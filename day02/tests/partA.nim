import unittest
from ../day02 import execute, parseProgram

suite "Program execution":
  test "Program '1,9,10,3,2,3,11,0,99,30,40,50'":
    check execute(parseProgram("1,9,10,3,2,3,11,0,99,30,40,50")) == 3500

  test "Program '1,0,0,0,99'":
    check execute(parseProgram("1,0,0,0,99")) == 2

  test "Program '2,3,0,3,99' with output at position 3":
    check execute(parseProgram("2,3,0,3,99"), 3) == 6

  test "Program '2,4,4,5,99,0' with output at position 5":
    check execute(parseProgram("2,4,4,5,99,0"), 5) == 9801

  test "Program '1,1,1,4,99,5,6,0,99'":
    check execute(parseProgram("1,1,1,4,99,5,6,0,99")) == 30
