/**
 * Title: Test Data Builder
 * Problem: Tests need valid Order objects with sensible defaults while overriding
 *   only the fields relevant to each test; derived totals stay consistent.
 * Approach: builder with defaults, withX overrides, and computed fields in build().
 * Complexity: O(i) for i line items.
 */
interface Line { sku: string; qty: number; unitCents: number }
interface Order { id: string; customer: string; lines: Line[]; totalCents: number; status: "new" | "paid" | "shipped" }
let seq = 0;
export class OrderBuilder {
  private id = `ord-${++seq}`;
  private customer = "test-customer";
  private lines: Line[] = [{ sku: "SKU-1", qty: 1, unitCents: 1000 }];
  private status: Order["status"] = "new";
  static anOrder(): OrderBuilder { return new OrderBuilder(); }
  withCustomer(c: string) { this.customer = c; return this; }
  withLines(...l: Line[]) { this.lines = l; return this; }
  paid() { this.status = "paid"; return this; }
  shipped() { this.status = "shipped"; return this; }
  build(): Order {
    const lines = this.lines.map((l) => ({ ...l }));
    return { id: this.id, customer: this.customer, lines, status: this.status,
      totalCents: lines.reduce((s, l) => s + l.qty * l.unitCents, 0) };
  }
}

const o = OrderBuilder.anOrder().withLines({ sku: "A", qty: 2, unitCents: 250 }, { sku: "B", qty: 1, unitCents: 999 }).paid().build();
console.assert(o.totalCents === 1499, "total should be 1499");
console.log(o, OrderBuilder.anOrder().build().totalCents); // ... 1000
