/**
 * Problem 251: Flatten 2D Vector
 * Problem: Design an iterator over a 2D array supporting next() and hasNext(), skipping
 *   empty inner arrays. Structurally this is an Adapter: it exposes a 2D structure through
 *   a flat iterator interface (and also implements the JS Iterable protocol).
 * Approach: Two indices (row, col); advance past exhausted rows lazily in hasNext().
 * Complexity: amortized O(1) per operation, O(1) extra space.
 */
class Vector2D implements Iterable<number> {
  private r = 0;
  private c = 0;
  constructor(private readonly vec: number[][]) {}

  hasNext(): boolean {
    while (this.r < this.vec.length && this.c >= this.vec[this.r].length) {
      this.r++;
      this.c = 0;
    }
    return this.r < this.vec.length;
  }

  next(): number {
    if (!this.hasNext()) throw new Error("No more elements");
    return this.vec[this.r][this.c++];
  }

  *[Symbol.iterator](): Iterator<number> {
    while (this.hasNext()) yield this.next();
  }
}

const it = new Vector2D([[1, 2], [], [3], [], [4, 5, 6]]);
console.log(it.next(), it.next(), it.hasNext()); // 1 2 true
console.log([...it]); // [3, 4, 5, 6]
