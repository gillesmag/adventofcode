import strutils
import sequtils
import sugar
import math

type
  Image = seq[seq[seq[int]]]
  Layer = seq[seq[int]]

proc readImage(filename: string, size: tuple[width: int, height: int]): Image =
  let pixelValues = readLines(filename)[0]
  var currentLayer = newSeq[seq[int]]()
  var currentRow = newSeq[int]()
  for i in 0..<pixelValues.len:
    currentRow.add(parseInt($pixelValues[i]))
    if currentRow.len == size.width:
      currentLayer.add(currentRow)
      currentRow = newSeq[int]()
    if currentLayer.len == size.height:
      result.add(currentLayer)
      currentLayer = newSeq[seq[int]]()

proc fewestZeroDigits(image: Image): Layer =
  var counts = newSeq[int]()
  for layer in image:
    let rowCounts = layer.map(x => count(x, 0))
    counts.add(sum(rowCounts))
  let layer = counts.find(min(counts))
  result = image[layer]

proc checksum(image: Image): int =
  let layer = fewestZeroDigits(image)
  let ones = sum(layer.map(x => count(x, 1)))
  let twos = sum(layer.map(x => count(x, 2)))
  return ones * twos

proc decode(image: Image): seq[seq[int]] =
  let layerCount = image.len
  let height = image[0].len
  let width = image[0][0].len

  let layerIndices = toSeq(0..<image.len)
  var currentRow = newSeq[int]()
  for h in 0..<height:
    for w in 0..<width:
       let colorDepth = layerIndices.map(x => image[x][h][w])
       let firstBlack = if colorDepth.find(0) >= 0: colorDepth.find(0) else: colorDepth.len
       let firstWhite = if colorDepth.find(1) >= 0: colorDepth.find(1) else: colorDepth.len
       currentRow.add(colorDepth[min(firstBlack, firstWhite)])
    result.add(currentRow)
    currentRow = newSeq[int]()

proc print(image: seq[seq[int]]) =
  for row in image:
    for pixel in row:
      case pixel:
      of 0: write(stdout, " ")
      of 1: write(stdout, "█")
      else: discard
      # write(stdout, pixel)
    write(stdout, '\n')
  flushFile(stdout)
    
proc partA(image: Image) =
  echo checksum(image)

proc partB(image: Image) =
  let decodedImage = decode(image)
  print(decodedImage)
  
proc main() =
  let image = readImage("input.txt", (width: 25, height: 6))
  partA(image)
  partB(image)

when isMainModule:
  main()
