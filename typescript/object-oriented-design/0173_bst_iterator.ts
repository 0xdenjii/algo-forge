/**
 * LeetCode 173 - Binary Search Tree Iterator
 * Iterate a BST in ascending order with next() and hasNext().
 * next/hasNext must run in average O(1) using O(h) memory.
 * Approach: explicit stack holding the left spine; after popping a node, push the left spine of its right child.
 * Time: amortized O(1) per call. Space: O(h).
 */
export class TreeNode {
  constructor(
    public val: number,
    public left: TreeNode | null = null,
    public right: TreeNode | null = null,
  ) {}
}

export class BSTIterator {
  private readonly stack: TreeNode[] = [];

  constructor(root: TreeNode | null) {
    this.pushLeft(root);
  }
  private pushLeft(node: TreeNode | null): void {
    for (; node; node = node.left) this.stack.push(node);
  }
  next(): number {
    const node = this.stack.pop()!;
    this.pushLeft(node.right);
    return node.val;
  }
  hasNext(): boolean {
    return this.stack.length > 0;
  }
}

//        7
//      /   \
//     3     15
//          /  \
//         9    20
const root = new TreeNode(7, new TreeNode(3), new TreeNode(15, new TreeNode(9), new TreeNode(20)));
const bi = new BSTIterator(root);
const out: number[] = [];
while (bi.hasNext()) out.push(bi.next());
console.log(out); // [3, 7, 9, 15, 20]
