/**
 * LeetCode 208 - Implement Trie (Prefix Tree)
 * Support insert(word), search(word) and startsWith(prefix).
 * Approach: nodes with a child map and an end-of-word flag.
 * Time: O(L) per op. Space: O(total characters).
 */
class TrieNode {
  children = new Map<string, TrieNode>();
  end = false;
}

export class Trie {
  private root = new TrieNode();

  insert(w: string): void {
    let n = this.root;
    for (const ch of w) {
      if (!n.children.has(ch)) n.children.set(ch, new TrieNode());
      n = n.children.get(ch)!;
    }
    n.end = true;
  }

  private walk(s: string): TrieNode | null {
    let n: TrieNode | undefined = this.root;
    for (const ch of s) {
      n = n.children.get(ch);
      if (!n) return null;
    }
    return n;
  }

  search(w: string): boolean {
    return this.walk(w)?.end ?? false;
  }
  startsWith(p: string): boolean {
    return this.walk(p) !== null;
  }
}

const t = new Trie();
t.insert("apple");
console.log(t.search("apple"), t.search("app"), t.startsWith("app")); // true false true
