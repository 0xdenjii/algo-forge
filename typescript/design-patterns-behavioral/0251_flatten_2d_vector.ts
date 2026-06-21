/**
 * LeetCode 251: Flatten 2D Vector
 * Problem: Design an iterator over a 2D array (rows may be empty) with
 *   next() and hasNext(), without copying the data.
 * Approach: Two indices (row, col); hasNext skips exhausted/empty rows.
 * Complexity: amortized O(1) per call (O(rows) total skipping), O(1) space.
 */
class Vector2D implements IterableIterator<number> {
  private row = 0;
  private col = 0;
  constructor(private readonly vec: readonly (readonly number[])[]) {}

  private advance(): void {
    while (this.row < this.vec.length && this.col >= this.vec[this.row].length) {
      this.row++;
      this.col = 0;
    }
  }
  hasNext(): boolean { this.advance(); return this.row < this.vec.length; }

  next(): IteratorResult<number, undefined> {
    if (!this.hasNext()) return { done: true, value: undefined };
    return { done: false, value: this.vec[this.row][this.col++] };
  }
  [Symbol.iterator](): IterableIterator<number> { return this; }
}

const v = new Vector2D([[1, 2], [], [3], [], [], [4, 5]]);
console.log(v.next().value, v.next().value, v.hasNext()); // 1 2 true
console.log([...v]); // [3, 4, 5]
console.log(v.hasNext()); // false
