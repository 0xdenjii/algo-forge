/**
 * LeetCode 211 - Design Add and Search Words Data Structure
 * addWord(word) stores a word; search(word) returns true if any stored word matches,
 * where "." in the query matches any single letter.
 * Approach: trie + DFS that branches over all children on ".".
 * Time: add O(L); search O(26^dots * L) worst case. Space: O(total chars).
 */
type TNode = { kids: Map<string, TNode>; end: boolean };
const mk = (): TNode => ({ kids: new Map(), end: false });

export class WordDictionary {
  private readonly root = mk();

  addWord(word: string): void {
    let n = this.root;
    for (const c of word) {
      if (!n.kids.has(c)) n.kids.set(c, mk());
      n = n.kids.get(c)!;
    }
    n.end = true;
  }

  search(word: string): boolean {
    const dfs = (n: TNode, i: number): boolean => {
      if (i === word.length) return n.end;
      const c = word[i];
      if (c === ".") {
        for (const k of n.kids.values()) if (dfs(k, i + 1)) return true;
        return false;
      }
      const k = n.kids.get(c);
      return k !== undefined && dfs(k, i + 1);
    };
    return dfs(this.root, 0);
  }
}

const wd = new WordDictionary();
["bad", "dad", "mad"].forEach((w) => wd.addWord(w));
console.log(wd.search("pad"), wd.search("bad"), wd.search(".ad"), wd.search("b..")); // false true true true
