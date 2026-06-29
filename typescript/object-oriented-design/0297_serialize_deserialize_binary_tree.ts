/**
 * LeetCode 297 - Serialize and Deserialize Binary Tree
 * Design a Codec that turns a binary tree into a string and reconstructs the identical tree.
 * Approach: preorder traversal with "#" for null children, comma-separated; rebuild with a cursor.
 * Time: O(n) both ways. Space: O(n).
 */
export class TreeNode {
  constructor(
    public val: number,
    public left: TreeNode | null = null,
    public right: TreeNode | null = null,
  ) {}
}

export class Codec {
  serialize(root: TreeNode | null): string {
    const out: string[] = [];
    const stack: Array<TreeNode | null> = [root];
    while (stack.length) {
      const n = stack.pop()!;
      if (!n) {
        out.push("#");
        continue;
      }
      out.push(String(n.val));
      stack.push(n.right, n.left);
    }
    return out.join(",");
  }

  deserialize(data: string): TreeNode | null {
    const tokens = data.split(",");
    let i = 0;
    const build = (): TreeNode | null => {
      const tok = tokens[i++];
      if (tok === "#" || tok === undefined) return null;
      const node = new TreeNode(Number(tok));
      node.left = build();
      node.right = build();
      return node;
    };
    return build();
  }
}

const codec = new Codec();
const tree = new TreeNode(1, new TreeNode(2), new TreeNode(3, new TreeNode(4), new TreeNode(5)));
const s = codec.serialize(tree);
console.log(s); // 1,2,#,#,3,4,#,#,5,#,#
console.log(codec.serialize(codec.deserialize(s)) === s); // true
console.log(codec.deserialize(codec.serialize(null))); // null
