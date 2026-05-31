/**
 * Title: Registry-based Factory (Plugin Parsers)
 * Problem: Parsers for file extensions are registered at runtime; create(ext)
 *   returns a fresh parser, or throws for unknown extensions. Open/closed: no switch.
 * Approach: Map<string, () => Parser> of constructors.
 * Complexity: O(1) registration and creation.
 */
interface Parser { parse(input: string): unknown }
export class ParserFactory {
  private readonly ctors = new Map<string, () => Parser>();
  register(ext: string, ctor: () => Parser): this { this.ctors.set(ext.toLowerCase(), ctor); return this; }
  create(ext: string): Parser {
    const ctor = this.ctors.get(ext.toLowerCase());
    if (!ctor) throw new Error(`No parser for .${ext}`);
    return ctor();
  }
}

const f = new ParserFactory()
  .register("json", () => ({ parse: (s) => JSON.parse(s) }))
  .register("csv", () => ({ parse: (s) => s.trim().split("\n").map((l) => l.split(",")) }));
console.log(f.create("JSON").parse('{"a":1}'), f.create("csv").parse("a,b\n1,2"));
try { f.create("xml"); } catch (e) { console.log((e as Error).message); }
