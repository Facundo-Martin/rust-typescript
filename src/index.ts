import fs from "fs";

function printIterableList(): void {
  const list: number[] = [1, 2, 3];
  const iterated = list.map((x) => x + 1);
  console.log("iterated list:", iterated);
}

function printLines() {
  return fs
    .readFileSync("lines")
    .toString()
    .split("\n")
    .filter((_, i) => i % 2 === 0)
    .filter((_, i) => i > 1 && i < 4)
    .forEach((line) => console.log(line));
}

printIterableList();
printLines();
