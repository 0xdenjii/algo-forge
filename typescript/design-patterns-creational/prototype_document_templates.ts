/**
 * Title: Prototype (Invoice Templates with Dates/Maps)
 * Problem: Clone invoice templates containing Dates, Maps and nested arrays; the
 *   clone must be fully independent and keep its class methods.
 * Approach: hand-written clone() preserves the prototype chain (structuredClone drops classes).
 * Complexity: O(s).
 */
class Invoice {
  constructor(
    public number: string,
    public issued: Date,
    public items: { desc: string; cents: number }[],
    public meta: Map<string, string>,
  ) {}
  total(): number { return this.items.reduce((s, i) => s + i.cents, 0); }
  clone(): Invoice {
    return new Invoice(this.number, new Date(this.issued), this.items.map((i) => ({ ...i })), new Map(this.meta));
  }
}

const template = new Invoice("TEMPLATE", new Date("2026-01-01"), [{ desc: "Consulting", cents: 50000 }], new Map([["currency", "EUR"]]));
const inv = template.clone();
inv.number = "INV-001"; inv.issued.setFullYear(2027); inv.items.push({ desc: "Travel", cents: 12000 }); inv.meta.set("po", "42");
console.log(template.total(), template.issued.getFullYear(), template.meta.size); // 50000 2026 1
console.log(inv.total(), inv instanceof Invoice); // 62000 true
