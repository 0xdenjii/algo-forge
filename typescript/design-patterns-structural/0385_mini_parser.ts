/**
 * Problem 385: Mini Parser
 * Problem: Deserialize a string such as "[123,[456,[789]]]" into a nested-integer Composite.
 *   Each element is an integer (possibly negative) or a nested list.
 * Approach: Single left-to-right pass with an explicit stack of currently open lists.
 * Complexity: O(n) time, O(depth) space.
 */
type NI = number | NI[];

function deserialize(s: string): NI {
  if (s[0] !== "[") return Number(s);
  const stack: NI[][] = [];
  let result: NI[] = [];
  let num = "";
  const flush = (): void => {
    if (num !== "") {
      stack[stack.length - 1].push(Number(num));
      num = "";
    }
  };
  for (const ch of s) {
    if (ch === "[") stack.push([]);
    else if (ch === "]") {
      flush();
      const done = stack.pop() as NI[];
      if (stack.length) stack[stack.length - 1].push(done);
      else result = done;
    } else if (ch === ",") flush();
    else num += ch;
  }
  return result;
}

console.log(JSON.stringify(deserialize("324"))); // 324
console.log(JSON.stringify(deserialize("[123,[456,[789]]]"))); // [123,[456,[789]]]
console.log(JSON.stringify(deserialize("[-1,[],[2,[-30]]]"))); // [-1,[],[2,[-30]]]
