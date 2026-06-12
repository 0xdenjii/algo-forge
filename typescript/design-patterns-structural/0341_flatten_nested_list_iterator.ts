/**
 * Problem 341: Flatten Nested List Iterator
 * Problem: Given a nested list of integers (each element is an integer or another list, i.e.
 *   a Composite), implement an iterator that yields all integers in order.
 * Approach: Stack of (list, index) frames; hasNext() descends until an integer is on top.
 * Complexity: amortized O(1) per next/hasNext; O(depth) space.
 */
class NestedInteger {
  private constructor(private value: number | null, private list: NestedInteger[]) {}
  static int(v: number): NestedInteger { return new NestedInteger(v, []); }
  static list(items: NestedInteger[]): NestedInteger { return new NestedInteger(null, items); }
  isInteger(): boolean { return this.value !== null; }
  getInteger(): number | null { return this.value; }
  getList(): NestedInteger[] { return this.list; }
}

class NestedIterator {
  private stack: { list: NestedInteger[]; i: number }[];
  constructor(nested: NestedInteger[]) { this.stack = [{ list: nested, i: 0 }]; }

  hasNext(): boolean {
    while (this.stack.length) {
      const top = this.stack[this.stack.length - 1];
      if (top.i >= top.list.length) { this.stack.pop(); continue; }
      const cur = top.list[top.i];
      if (cur.isInteger()) return true;
      top.i++;
      this.stack.push({ list: cur.getList(), i: 0 });
    }
    return false;
  }

  next(): number {
    if (!this.hasNext()) throw new Error("exhausted");
    const top = this.stack[this.stack.length - 1];
    return top.list[top.i++].getInteger() as number;
  }
}

const I = NestedInteger.int;
const L = NestedInteger.list;
const it = new NestedIterator([L([I(1), I(1)]), I(2), L([I(1), L([]), L([I(4), L([I(6)])])])]);
const out: number[] = [];
while (it.hasNext()) out.push(it.next());
console.log(out); // [1, 1, 2, 1, 4, 6]
