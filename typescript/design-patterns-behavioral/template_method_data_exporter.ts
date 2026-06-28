/**
 * Title: Template Method - Report Exporter (CSV / JSON / Markdown)
 * Problem: Exporting a report always follows: validate rows, write header,
 *   write each row, write footer. Formats differ only in those steps.
 * Approach: Abstract class fixes the algorithm in a final-by-convention
 *   export() method; subclasses implement the primitive steps and may
 *   override an optional hook (footer).
 * Complexity: O(r * c) for r rows and c columns; O(output) space.
 */
type Row = Record<string, string | number>;

abstract class Exporter {
  export(columns: string[], rows: Row[]): string {
    for (const r of rows) for (const c of columns) {
      if (!(c in r)) throw new Error(`row missing column ${c}`);
    }
    const out = [this.header(columns), ...rows.map((r, i) => this.row(columns, r, i))];
    const f = this.footer(rows.length);
    if (f !== null) out.push(f);
    return out.filter((s) => s !== "").join("\n");
  }
  protected abstract header(cols: string[]): string;
  protected abstract row(cols: string[], r: Row, index: number): string;
  protected footer(_count: number): string | null { return null; } // hook
}

class CsvExporter extends Exporter {
  private esc = (v: unknown) => { const s = String(v); return /[",\n]/.test(s) ? `"${s.replace(/"/g, '""')}"` : s; };
  protected header(cols: string[]) { return cols.map(this.esc).join(","); }
  protected row(cols: string[], r: Row) { return cols.map((c) => this.esc(r[c])).join(","); }
}

class JsonLinesExporter extends Exporter {
  protected header() { return ""; }
  protected row(cols: string[], r: Row) { return JSON.stringify(Object.fromEntries(cols.map((c) => [c, r[c]]))); }
}

class MarkdownExporter extends Exporter {
  protected header(cols: string[]) { return `| ${cols.join(" | ")} |\n|${cols.map(() => "---").join("|")}|`; }
  protected row(cols: string[], r: Row) { return `| ${cols.map((c) => r[c]).join(" | ")} |`; }
  protected footer(n: number) { return `\n_${n} rows_`; }
}

const cols = ["name", "qty"];
const rows: Row[] = [{ name: "tent, 2p", qty: 3 }, { name: 'bike "pro"', qty: 1 }];
for (const e of [new CsvExporter(), new JsonLinesExporter(), new MarkdownExporter()]) {
  console.log(e.export(cols, rows), "\n");
}
