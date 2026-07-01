/**
 * LeetCode 341 - Flatten Nested List Iterator
 * Given a nested list of integers (each element is an integer or a list), implement an
 * iterator returning the integers in order via next()/hasNext().
 * Approach: lazy stack of array iterators; hasNext() descends into lists until an integer is on top.
 * Time: amortized O(1) per element. Space: O(depth).
 */
export type NestedInteger = number | NestedInteger[];

export class NestedIterator {
  private readonly stack: Array<Iterator<NestedInteger>> = [];
  private nextVal: number | undefined;

  constructor(list: NestedInteger[]) {
    this.stack.push(list[Symbol.iterator]());
  }

  hasNext(): boolean {
    while (this.nextVal === undefined && this.stack.length) {
      const r = this.stack[this.stack.length - 1].next();
      if (r.done) this.stack.pop();
      else if (typeof r.value === "number") this.nextVal = r.value;
      else this.stack.push(r.value[Symbol.iterator]());
    }
    return this.nextVal !== undefined;
  }

  next(): number {
    if (!this.hasNext()) throw new Error("no more elements");
    const v = this.nextVal!;
    this.nextVal = undefined;
    return v;
  }
}

const ni = new NestedIterator([[1, 1], 2, [1, 1], [[], [[3]]]]);
const flat: number[] = [];
while (ni.hasNext()) flat.push(ni.next());
console.log(flat); // [1, 1, 2, 1, 1, 3]
