/**
 * Title: Node Factory with Interning (Expression Evaluator)
 * Problem: Parse arithmetic like "2*(3+4)-5" into an AST whose nodes are created
 *   via a NodeFactory (which interns identical number literals), then evaluate.
 * Approach: recursive descent parser delegating all construction to the factory.
 * Complexity: O(n) parse and evaluate.
 */
type BinOp = "+" | "-" | "*" | "/";
type Expr = { k: "num"; v: number } | { k: "bin"; op: BinOp; l: Expr; r: Expr };
class NodeFactory {
  private nums = new Map<number, Expr>();
  num(v: number): Expr { let n = this.nums.get(v); if (!n) { n = { k: "num", v }; this.nums.set(v, n); } return n; }
  bin(op: BinOp, l: Expr, r: Expr): Expr { return { k: "bin", op, l, r }; }
}
function parse(src: string, f = new NodeFactory()): Expr {
  const s = src.replace(/\s+/g, "");
  let i = 0;
  const expr = (): Expr => {
    let l = term();
    while (s[i] === "+" || s[i] === "-") { const op = s[i++] as BinOp; l = f.bin(op, l, term()); }
    return l;
  };
  const term = (): Expr => {
    let l = atom();
    while (s[i] === "*" || s[i] === "/") { const op = s[i++] as BinOp; l = f.bin(op, l, atom()); }
    return l;
  };
  const atom = (): Expr => {
    if (s[i] === "(") { i++; const e = expr(); if (s[i++] !== ")") throw new Error("expected )"); return e; }
    const m = /^\d+(\.\d+)?/.exec(s.slice(i));
    if (!m) throw new Error(`unexpected input at ${i}`);
    i += m[0].length;
    return f.num(Number(m[0]));
  };
  const e = expr();
  if (i !== s.length) throw new Error(`trailing input at ${i}`);
  return e;
}
const OPS: Record<BinOp, (a: number, b: number) => number> = {
  "+": (a, b) => a + b, "-": (a, b) => a - b, "*": (a, b) => a * b, "/": (a, b) => a / b,
};
const evaluate = (e: Expr): number => e.k === "num" ? e.v : OPS[e.op](evaluate(e.l), evaluate(e.r));

console.log(evaluate(parse("2*(3+4)-5")), evaluate(parse("10/4 + 2*2"))); // 9 6.5
const ast = parse("3+3") as Extract<Expr, { k: "bin" }>;
console.log("interned:", ast.l === ast.r); // true
