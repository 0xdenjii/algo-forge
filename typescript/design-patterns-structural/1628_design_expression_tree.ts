/**
 * Problem 1628: Design an Expression Tree With Evaluate Function
 * Problem: Build an expression tree from postfix tokens, e.g. ["3","4","+","2","*"], and
 *   evaluate it. Numbers are leaves; operators are composite nodes with two children.
 * Approach: Stack-based construction; polymorphic evaluate()/toString() on nodes (Composite).
 * Complexity: O(n) build and evaluate, O(n) space.
 */
interface Expr { evaluate(): number; toString(): string }

class Num implements Expr {
  constructor(private v: number) {}
  evaluate(): number { return this.v; }
  toString(): string { return String(this.v); }
}

type Op = "+" | "-" | "*" | "/";
const OPS: Record<Op, (a: number, b: number) => number> = {
  "+": (a, b) => a + b,
  "-": (a, b) => a - b,
  "*": (a, b) => a * b,
  "/": (a, b) => Math.trunc(a / b),
};
const isOp = (t: string): t is Op => Object.prototype.hasOwnProperty.call(OPS, t);

class BinOp implements Expr {
  constructor(private op: Op, private l: Expr, private r: Expr) {}
  evaluate(): number { return OPS[this.op](this.l.evaluate(), this.r.evaluate()); }
  toString(): string { return `(${this.l} ${this.op} ${this.r})`; }
}

function buildTree(postfix: string[]): Expr {
  const st: Expr[] = [];
  for (const t of postfix) {
    if (isOp(t)) {
      const r = st.pop();
      const l = st.pop();
      if (!l || !r) throw new Error("malformed expression");
      st.push(new BinOp(t, l, r));
    } else {
      st.push(new Num(Number(t)));
    }
  }
  if (st.length !== 1) throw new Error("malformed expression");
  return st[0];
}

const e = buildTree(["3", "4", "+", "2", "*", "7", "/"]);
console.log(e.toString(), "=", e.evaluate()); // (((3 + 4) * 2) / 7) = 2
console.log(buildTree(["4", "5", "2", "7", "+", "-", "*"]).evaluate()); // -16
