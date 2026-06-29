/**
 * LeetCode 208 - Implement Trie (Prefix Tree)
 * Support insert(word), search(word) for exact match, and startsWith(prefix).
 * Approach: tree of nodes with child Map<char, node> and an end-of-word flag.
 * Time: O(L) per op. Space: O(total characters inserted).
 */
class TrieNode {
  readonly children = new Map<string, TrieNode>();
  isWord = false;
}

export class Trie {
  private readonly root = new TrieNode();

  insert(word: string): void {
    let node = this.root;
    for (const ch of word) {
      let next = node.children.get(ch);
      if (!next) {
        next = new TrieNode();
        node.children.set(ch, next);
      }
      node = next;
    }
    node.isWord = true;
  }
  private walk(s: string): TrieNode | undefined {
    let node: TrieNode | undefined = this.root;
    for (const ch of s) {
      node = node.children.get(ch);
      if (!node) return undefined;
    }
    return node;
  }
  search(word: string): boolean {
    return this.walk(word)?.isWord ?? false;
  }
  startsWith(prefix: string): boolean {
    return this.walk(prefix) !== undefined;
  }
}

const t = new Trie();
t.insert("apple");
console.log(t.search("apple"), t.search("app"), t.startsWith("app")); // true false true
t.insert("app");
console.log(t.search("app")); // true
