/**
 * Problem 339 / 364: Nested List Weight Sum I and II
 * Problem: Given a nested list (a Composite of ints and lists), return the sum of each
 *   integer times its depth (I), or times (maxDepth - depth + 1) (II).
 * Approach: Recursive visit for I. For II, BFS level by level keeping a running sum of all
 *   integers seen so far and adding it once per level, so shallower values count more often.
 * Complexity: O(n) time; O(depth) stack for I, O(width) queue for II.
 */
type Nested = number | Nested[];

function depthSum(list: Nested[], depth = 1): number {
  return list.reduce<number>((s, x) => s + (typeof x === "number" ? x * depth : depthSum(x, depth + 1)), 0);
}

function depthSumInverse(list: Nested[]): number {
  let level: Nested[] = list;
  let running = 0;
  let total = 0;
  while (level.length) {
    const next: Nested[] = [];
    for (const x of level) {
      if (typeof x === "number") running += x;
      else next.push(...x);
    }
    total += running;
    level = next;
  }
  return total;
}

console.log(depthSum([[1, 1], 2, [1, 1]])); // 10
console.log(depthSum([1, [4, [6]]])); // 27
console.log(depthSumInverse([[1, 1], 2, [1, 1]])); // 8
console.log(depthSumInverse([1, [4, [6]]])); // 17
