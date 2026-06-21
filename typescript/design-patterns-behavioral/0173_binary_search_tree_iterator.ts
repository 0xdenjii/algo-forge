/**
 * LeetCode 173: Binary Search Tree Iterator
 * Problem: Implement an iterator over a BST that returns values in ascending
 *   order. next() returns the next smallest number, hasNext() whether one
 *   exists. Memory must be O(h).
 * Approach: Iterator pattern with an explicit stack of the left spine; also
 *   implements Symbol.iterator so it works with for...of and spread.
 * Complexity: amortized O(1) per next, O(h) space.
 */
class TreeNode {
  constructor(public val: number, public left: TreeNode | null = null, public right: TreeNode | null = null) {}
}

class BSTIterator implements Iterable<number> {
  private stack: TreeNode[] = [];
  constructor(root: TreeNode | null) { this.pushLeft(root); }
  private pushLeft(n: TreeNode | null): void { while (n) { this.stack.push(n); n = n.left; } }
  hasNext(): boolean { return this.stack.length > 0; }
  next(): number {
    const n = this.stack.pop();
    if (!n) throw new Error("iterator exhausted");
    this.pushLeft(n.right);
    return n.val;
  }
  *[Symbol.iterator](): Iterator<number> { while (this.hasNext()) yield this.next(); }
}

const root = new TreeNode(7, new TreeNode(3), new TreeNode(15, new TreeNode(9), new TreeNode(20)));
console.log([...new BSTIterator(root)]); // [3, 7, 9, 15, 20]
