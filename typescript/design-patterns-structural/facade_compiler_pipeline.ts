/**
 * Title: Facade - Compiler Pipeline
 * Problem: Compiling an arithmetic expression involves a Lexer, Parser, and CodeGenerator
 *   (stack-machine bytecode) plus a VM. Expose `Compiler.run(source)` hiding all stages.
 * Approach: Facade wires the subsystems; each subsystem is a small, real implementation
 *   (recursive-descent parser for + - * / and parentheses).
 * Complexity: O(n) for lexing, parsing, codegen and execution.
 */
type Token = { kind: "num"; value: number } | { kind: "op"; value: string };
type Ast = { type: "num"; value: number } | { type: "bin"; op: string; l: Ast; r: Ast };
type Instr = ["push", number] | ["op", string];

class Lexer {
  tokenize(src: string): Token[] {
    const out: Token[] = [];
    const re = /\s*(\d+(?:\.\d+)?|[-+*/()])/y;
    let m: RegExpExecArray | null;
    while (re.lastIndex < src.length && (m = re.exec(src))) {
      out.push(/\d/.test(m[1]) ? { kind: "num", value: Number(m[1]) } : { kind: "op", value: m[1] });
    }
    if (src.slice(re.lastIndex).trim()) throw new SyntaxError(`unexpected input at ${re.lastIndex}`);
    return out;
  }
}

class Parser {
  private pos = 0;
  private toks: Token[] = [];
  parse(tokens: Token[]): Ast {
    this.toks = tokens;
    this.pos = 0;
    const ast = this.expr();
    if (this.pos !== tokens.length) throw new SyntaxError("trailing tokens");
    return ast;
  }
  private peekOp(...ops: string[]): string | null {
    const t = this.toks[this.pos];
    return t && t.kind === "op" && ops.includes(t.value) ? t.value : null;
  }
  private expr(): Ast {
    let l = this.term();
    for (let op; (op = this.peekOp("+", "-")); ) { this.pos++; l = { type: "bin", op, l, r: this.term() }; }
    return l;
  }
  private term(): Ast {
    let l = this.factor();
    for (let op; (op = this.peekOp("*", "/")); ) { this.pos++; l = { type: "bin", op, l, r: this.factor() }; }
    return l;
  }
  private factor(): Ast {
    const t = this.toks[this.pos++];
    if (!t) throw new SyntaxError("unexpected end");
    if (t.kind === "num") return { type: "num", value: t.value };
    if (t.value === "-") return { type: "bin", op: "-", l: { type: "num", value: 0 }, r: this.factor() };
    if (t.value === "(") {
      const e = this.expr();
      if (!this.peekOp(")")) throw new SyntaxError("expected )");
      this.pos++;
      return e;
    }
    throw new SyntaxError(`unexpected ${t.value}`);
  }
}

class CodeGenerator {
  emit(ast: Ast, out: Instr[] = []): Instr[] {
    if (ast.type === "num") out.push(["push", ast.value]);
    else { this.emit(ast.l, out); this.emit(ast.r, out); out.push(["op", ast.op]); }
    return out;
  }
}

class StackVM {
  execute(code: Instr[]): number {
    const st: number[] = [];
    for (const [kind, arg] of code) {
      if (kind === "push") { st.push(arg as number); continue; }
      const b = st.pop()!, a = st.pop()!;
      st.push(arg === "+" ? a + b : arg === "-" ? a - b : arg === "*" ? a * b : a / b);
    }
    return st[0];
  }
}

class Compiler {
  private lexer = new Lexer();
  private parser = new Parser();
  private gen = new CodeGenerator();
  private vm = new StackVM();
  compile(src: string): Instr[] { return this.gen.emit(this.parser.parse(this.lexer.tokenize(src))); }
  run(src: string): number { return this.vm.execute(this.compile(src)); }
}

const c = new Compiler();
console.log(c.compile("1 + 2 * 3"));
console.log(c.run("1 + 2 * 3"), c.run("(1 + 2) * 3"), c.run("-4 + 10 / (2 + 3)")); // 7 9 -2
