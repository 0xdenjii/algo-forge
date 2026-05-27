/**
 * Title: SQL Query Builder
 * Problem: Build parameterised SELECT statements with where/orderBy/limit,
 *   returning { text, values } safe from injection.
 * Approach: accumulate clauses; identifiers validated and quoted; values go to a params array.
 * Complexity: O(c) for c clauses.
 */
type Op = "=" | "<" | ">" | "<=" | ">=" | "<>";
export class SelectBuilder {
  private cols: string[] = ["*"];
  private wheres: string[] = [];
  private values: unknown[] = [];
  private order: string[] = [];
  private lim?: number;
  private static ident(s: string): string {
    if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(s)) throw new Error(`bad identifier ${s}`);
    return `"${s}"`;
  }
  constructor(private readonly table: string) {}
  select(...c: string[]) { this.cols = c.map(SelectBuilder.ident); return this; }
  where(col: string, op: Op, v: unknown) {
    this.values.push(v);
    this.wheres.push(`${SelectBuilder.ident(col)} ${op} $${this.values.length}`);
    return this;
  }
  orderBy(col: string, dir: "ASC" | "DESC" = "ASC") { this.order.push(`${SelectBuilder.ident(col)} ${dir}`); return this; }
  limit(n: number) { if (!Number.isInteger(n) || n < 0) throw new RangeError("limit"); this.lim = n; return this; }
  build(): { text: string; values: unknown[] } {
    let text = `SELECT ${this.cols.join(", ")} FROM ${SelectBuilder.ident(this.table)}`;
    if (this.wheres.length) text += ` WHERE ${this.wheres.join(" AND ")}`;
    if (this.order.length) text += ` ORDER BY ${this.order.join(", ")}`;
    if (this.lim !== undefined) text += ` LIMIT ${this.lim}`;
    return { text, values: [...this.values] };
  }
}

console.log(new SelectBuilder("users").select("id", "name").where("age", ">=", 18)
  .where("country", "=", "FR").orderBy("name").limit(10).build());
try { new SelectBuilder("users; DROP TABLE x").build(); } catch (e) { console.log((e as Error).message); }
