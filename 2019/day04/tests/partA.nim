import unittest
from ../solution04 import strToDigits, areDigitsWeaklyIncreasing, partA, partB

suite "Helper functions":
  let puzzleInput = "273025-767253"

  test "areDigitsWeaklyIncreasing":
    check areDigitsWeaklyIncreasing(strToDigits("111111")) == true
    check areDigitsWeaklyIncreasing(strToDigits("223450")) == false
    check areDigitsWeaklyIncreasing(strToDigits("123789")) == true
    
  test "partA":
    check partA(puzzleInput) == 910

  test "partB":
    check partB(puzzleInput) == 598
