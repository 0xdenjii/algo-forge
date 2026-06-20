/**
 * LeetCode 150: Evaluate Reverse Polish Notation
 * Problem: Evaluate an arithmetic expression in RPN with + - * /. Division
 *   truncates toward zero. The input is guaranteed valid.
 * Approach: Interpreter/Command flavour: each operator token maps to a
 *   binary operation object in a registry; operands go on a stack.
 * Complexity: O(n) time, O(n) space.
 */
type BinOp = (a: number, b: number) => number;

const OPS: Readonly<Record<string, BinOp>> = {
  "+": (a, b) => a + b,
  "-": (a, b) => a - b,
  "*": (a, b) => a * b,
  "/": (a, b) => Math.trunc(a / b),
};

function evalRPN(tokens: string[]): number {
  const st: number[] = [];
  for (const tok of tokens) {
    const op = OPS[tok];
    if (op) {
      const b = st.pop(), a = st.pop();
      if (a === undefined || b === undefined) throw new Error("malformed expression");
      st.push(op(a, b));
    } else {
      st.push(Number(tok));
    }
  }
  if (st.length !== 1) throw new Error("malformed expression");
  return st[0];
}

console.log(evalRPN(["2", "1", "+", "3", "*"]));  // 9
console.log(evalRPN(["4", "13", "5", "/", "+"])); // 6
console.log(evalRPN(["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"])); // 22
