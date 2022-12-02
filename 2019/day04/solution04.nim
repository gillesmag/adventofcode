import strutils
import sequtils
import sugar

proc strToDigits*(number: string): seq[int] =
  for n in number:
    result.add(parseInt($n))

proc areDigitsWeaklyIncreasing*(digits: seq[int]): bool =
  for i in 1..<digits.len:
    if digits[i] - digits[i-1] < 0:
      return false
  return true

proc hasTwoAdjacentDigits(digits: seq[int]): bool =
  for i in 1..<digits.len:
    if digits[i-1] == digits[i]:
      return true

proc twoAdjacentNotMore(digits: seq[int]): bool =
  var adjDigitCount = newSeq[tuple[num: int, amount: int]]()
  var currentDigit = digits[0]
  var amount = 1
  for i in 1..<digits.len:
    if digits[i-1] == digits[i]:
      amount += 1
      if i == digits.len-1:
        adjDigitCount.add((num: currentDigit, amount: amount))
    else:
      adjDigitCount.add((num: currentDigit, amount: amount))
      currentDigit = digits[i]
      amount = 1

  if adjDigitCount.filter(x => x.amount == 2).len >= 1:
    return true
  return false

proc partA*(rangeStr: string): int =
  let numberRange = rangeStr.split("-").map(x => parseInt(x))
  let startRange = numberRange[0]
  let endRange = numberRange[1]
  for n in startRange..endRange:
    let digits = strToDigits(intToStr(n))
    if not hasTwoAdjacentDigits(digits):
      continue
    if not areDigitsWeaklyIncreasing(digits):
      continue
    result += 1

proc partB*(rangeStr: string): int =
  let numberRange = rangeStr.split("-").map(x => parseInt(x))
  let startRange = numberRange[0]
  let endRange = numberRange[1]
  for n in startRange..endRange:
    let digits = strToDigits(intToStr(n))
    if not twoAdjacentNotMore(digits):
      continue
    if not areDigitsWeaklyIncreasing(digits):
      continue
    result += 1
    
proc main() =
  let input = "273025-767253"
  echo partA(input)
  echo partB(input)

when isMainModule:
  main()
