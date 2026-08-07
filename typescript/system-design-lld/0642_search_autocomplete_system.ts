/**
 * LeetCode 642 - Design Search Autocomplete System
 * Given historical sentences with counts, return the top 3 hottest sentences for the typed
 * prefix after each character (ties broken lexicographically). '#' ends the input and records it.
 * Approach: trie whose nodes keep counts of every sentence passing through; sort matches on query.
 * Time: input O(m log m) for m matches. Space: O(total characters * depth).
 */
class AcNode {
  kids = new Map<string, AcNode>();
  counts = new Map<string, number>();
}

export class AutocompleteSystem {
  private root = new AcNode();
  private cur: AcNode | null;
  private typed = "";

  constructor(sentences: string[], times: number[]) {
    sentences.forEach((s, i) => this.add(s, times[i]));
    this.cur = this.root;
  }

  private add(s: string, c: number): void {
    let n = this.root;
    for (const ch of s) {
      if (!n.kids.has(ch)) n.kids.set(ch, new AcNode());
      n = n.kids.get(ch)!;
      n.counts.set(s, (n.counts.get(s) ?? 0) + c);
    }
  }

  input(c: string): string[] {
    if (c === "#") {
      this.add(this.typed, 1);
      this.typed = "";
      this.cur = this.root;
      return [];
    }
    this.typed += c;
    this.cur = this.cur?.kids.get(c) ?? null;
    if (!this.cur) return [];
    return [...this.cur.counts]
      .sort((a, b) => b[1] - a[1] || (a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0))
      .slice(0, 3)
      .map(e => e[0]);
  }
}

const ac = new AutocompleteSystem(["i love you", "island", "iroman", "i love leetcode"], [5, 3, 2, 2]);
console.log(ac.input("i")); // ["i love you", "island", "i love leetcode"]
console.log(ac.input(" ")); // ["i love you", "i love leetcode"]
console.log(ac.input("a")); // []
console.log(ac.input("#")); // []
