/**
 * Title: Interpreter - Boolean Rule Language
 * Problem: Evaluate feature-flag rules such as
 *   `country == "FR" and (age >= 18 or not beta)` against a context object.
 *   Support and/or/not, parentheses, comparisons, strings, numbers, booleans.
 * Approach: Tokenizer + recursive-descent parser building an AST of
 *   interpretable nodes (Interpreter pattern); precedence not > and > or.
 * Complexity: O(n) tokenize/parse for n characters; O(nodes) per evaluation.
 */
type Value = string | number | boolean;
type Ctx = Record<string, Value>;
interface Node { interpret(ctx: Ctx): Value }

const lit = (v: Value): Node => ({ interpret: () => v });
const ident = (name: string): Node => ({
  interpret: (c) => { if (!(name in c)) throw new Error(`unknown variable ${name}`); return c[name]; },
});
const not = (e: Node): Node => ({ interpret: (c) => !e.interpret(c) });
const and = (l: Node, r: Node): Node => ({ interpret: (c) => Boolean(l.interpret(c)) && Boolean(r.interpret(c)) });
const or = (l: Node, r: Node): Node => ({ interpret: (c) => Boolean(l.interpret(c)) || Boolean(r.interpret(c)) });
const ops: Record<string, (a: Value, b: Value) => boolean> = {
  "==": (a, b) => a === b, "!=": (a, b) => a !== b,
  "<": (a, b) => a < b, "<=": (a, b) => a <= b, ">": (a, b) => a > b, ">=": (a, b) => a >= b,
};
const cmp = (op: string, l: Node, r: Node): Node => ({ interpret: (c) => ops[op](l.interpret(c), r.interpret(c)) });

function tokenize(src: string): string[] {
  const re = /\s*(>=|<=|==|!=|[()<>]|"[^"]*"|[A-Za-z_]\w*|\d+(?:\.\d+)?)/y;
  const out: string[] = [];
  re.lastIndex = 0;
  while (re.lastIndex < src.length) {
    if (/^\s*$/.test(src.slice(re.lastIndex))) break;
    const m = re.exec(src);
    if (!m) throw new SyntaxError(`bad input at ${re.lastIndex}`);
    out.push(m[1]);
  }
  return out;
}

function parse(src: string): Node {
  const t = tokenize(src);
  let i = 0;
  const peek = () => t[i];
  const eat = (x?: string) => {
    const tok = t[i++];
    if (tok === undefined || (x && tok !== x)) throw new SyntaxError(`expected ${x ?? "token"}, got ${tok}`);
    return tok;
  };
  const orExpr = (): Node => { let n = andExpr(); while (peek() === "or") { eat(); n = or(n, andExpr()); } return n; };
  const andExpr = (): Node => { let n = unary(); while (peek() === "and") { eat(); n = and(n, unary()); } return n; };
  const unary = (): Node => (peek() === "not" ? (eat(), not(unary())) : comparison());
  const comparison = (): Node => {
    const l = primary();
    return peek() in ops ? cmp(eat(), l, primary()) : l;
  };
  const primary = (): Node => {
    const tok = eat();
    if (tok === "(") { const n = orExpr(); eat(")"); return n; }
    if (tok.startsWith('"')) return lit(tok.slice(1, -1));
    if (/^\d/.test(tok)) return lit(Number(tok));
    if (tok === "true" || tok === "false") return lit(tok === "true");
    return ident(tok);
  };
  const root = orExpr();
  if (i !== t.length) throw new SyntaxError(`unexpected ${t[i]}`);
  return root;
}

const rule = parse('country == "FR" and (age >= 18 or not beta)');
console.log(rule.interpret({ country: "FR", age: 20, beta: true }));  // true
console.log(rule.interpret({ country: "FR", age: 16, beta: true }));  // false
console.log(rule.interpret({ country: "FR", age: 16, beta: false })); // true
console.log(rule.interpret({ country: "DE", age: 30, beta: false })); // false
