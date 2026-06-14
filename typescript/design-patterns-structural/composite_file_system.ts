/**
 * Title: Composite - File System Tree
 * Problem: Model files and directories uniformly: both have a name and size; directories
 *   contain children. Support total size, find-by-predicate, and a `tree`-style printout.
 * Approach: Abstract FsNode with FileNode (leaf) and Directory (composite) recursing.
 * Complexity: size/find/print are O(n) over all nodes; O(depth) recursion stack.
 */
abstract class FsNode {
  constructor(readonly name: string) {}
  abstract size(): number;
  abstract find(pred: (f: FileNode) => boolean): FileNode[];
  abstract print(prefix?: string): string[];
}

class FileNode extends FsNode {
  constructor(name: string, private bytes: number) { super(name); }
  size(): number { return this.bytes; }
  find(pred: (f: FileNode) => boolean): FileNode[] { return pred(this) ? [this] : []; }
  print(prefix = ""): string[] { return [`${prefix}${this.name} (${this.bytes})`]; }
}

class Directory extends FsNode {
  private children: FsNode[] = [];
  add(...nodes: FsNode[]): this { this.children.push(...nodes); return this; }
  size(): number { return this.children.reduce((s, c) => s + c.size(), 0); }
  find(pred: (f: FileNode) => boolean): FileNode[] { return this.children.flatMap((c) => c.find(pred)); }
  print(prefix = ""): string[] {
    return [`${prefix}${this.name}/ [${this.size()}]`, ...this.children.flatMap((c) => c.print(prefix + "  "))];
  }
}

const root = new Directory("root").add(
  new FileNode("readme.md", 120),
  new Directory("src").add(
    new FileNode("main.ts", 900),
    new FileNode("util.ts", 300),
    new Directory("assets").add(new FileNode("logo.png", 4000)),
  ),
  new Directory("empty"),
);
console.log(root.print().join("\n"));
console.log("ts files:", root.find((f) => f.name.endsWith(".ts")).map((f) => f.name));
