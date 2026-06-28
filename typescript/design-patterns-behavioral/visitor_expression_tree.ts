/**
 * Title: Visitor - Arithmetic Expression Tree
 * Problem: An AST of Num, Var, Add, Mul and Neg nodes needs several
 *   independent operations: evaluate, pretty-print and symbolic derivative.
 *   Adding an operation should not require editing node classes.
 * Approach: Classic double dispatch: nodes accept(visitor); each operation is
 *   a Visitor<R> implementation.
 * Complexity: O(n) per visitor pass for n nodes; O(depth) recursion.
 */
interface Visitor<R> {
  num(n: Num): R; variable(v: Var): R; add(a: Add): R; mul(m: Mul): R; neg(n: Neg): R;
}
interface Expr { accept<R>(v: Visitor<R>): R }

class Num implements Expr { constructor(readonly value: number) {} accept<R>(v: Visitor<R>) { return v.num(this); } }
class Var implements Expr { constructor(readonly name: string) {} accept<R>(v: Visitor<R>) { return v.variable(this); } }
class Add implements Expr { constructor(readonly l: Expr, readonly r: Expr) {} accept<R>(v: Visitor<R>) { return v.add(this); } }
class Mul implements Expr { constructor(readonly l: Expr, readonly r: Expr) {} accept<R>(v: Visitor<R>) { return v.mul(this); } }
class Neg implements Expr { constructor(readonly e: Expr) {} accept<R>(v: Visitor<R>) { return v.neg(this); } }

class Evaluator implements Visitor<number> {
  constructor(private env: Record<string, number>) {}
  num(n: Num) { return n.value; }
  variable(v: Var) {
    if (!(v.name in this.env)) throw new Error(`unbound ${v.name}`);
    return this.env[v.name];
  }
  add(a: Add) { return a.l.accept(this) + a.r.accept(this); }
  mul(m: Mul) { return m.l.accept(this) * m.r.accept(this); }
  neg(n: Neg) { return -n.e.accept(this); }
}

class Printer implements Visitor<string> {
  num(n: Num) { return String(n.value); }
  variable(v: Var) { return v.name; }
  add(a: Add) { return `(${a.l.accept(this)} + ${a.r.accept(this)})`; }
  mul(m: Mul) { return `${m.l.accept(this)} * ${m.r.accept(this)}`; }
  neg(n: Neg) { return `-${n.e.accept(this)}`; }
}

/** d/dx with light simplification of 0 and 1 terms. */
class Derivative implements Visitor<Expr> {
  constructor(private x: string) {}
  private static add(a: Expr, b: Expr): Expr {
    if (a instanceof Num && a.value === 0) return b;
    if (b instanceof Num && b.value === 0) return a;
    return new Add(a, b);
  }
  private static mul(a: Expr, b: Expr): Expr {
    if ((a instanceof Num && a.value === 0) || (b instanceof Num && b.value === 0)) return new Num(0);
    if (a instanceof Num && a.value === 1) return b;
    if (b instanceof Num && b.value === 1) return a;
    return new Mul(a, b);
  }
  num() { return new Num(0); }
  variable(v: Var) { return new Num(v.name === this.x ? 1 : 0); }
  add(a: Add) { return Derivative.add(a.l.accept(this), a.r.accept(this)); }
  mul(m: Mul) { return Derivative.add(Derivative.mul(m.l.accept(this), m.r), Derivative.mul(m.l, m.r.accept(this))); }
  neg(n: Neg) { const d = n.e.accept(this); return d instanceof Num ? new Num(-d.value) : new Neg(d); }
}

// f(x) = x*x + 3*x + -y
const x = new Var("x");
const f = new Add(new Add(new Mul(x, x), new Mul(new Num(3), x)), new Neg(new Var("y")));
const p = new Printer();
console.log(f.accept(p));                                  // ((x * x + 3 * x) + -y)
console.log(f.accept(new Evaluator({ x: 2, y: 1 })));      // 9
const df = f.accept(new Derivative("x"));
console.log(df.accept(p), "=", df.accept(new Evaluator({ x: 2 }))); // 7
