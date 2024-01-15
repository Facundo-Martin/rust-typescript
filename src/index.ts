function printIterableList(): void {
  const list: number[] = [1, 2, 3];
  const iterated = list.map((x) => x + 1);
  console.log("iterated list:", iterated);
}

printIterableList();
