/**
 * Title: Chain of Responsibility - ATM Cash Dispenser
 * Problem: An ATM holds limited counts of 100/50/20/10 notes. Dispense an
 *   amount using the largest notes first; if the exact amount cannot be
 *   made from inventory, dispense nothing.
 * Approach: Each note handler takes what it can and passes the remainder to
 *   the next handler. Plan first, commit only if the remainder reaches 0.
 *   (Greedy is exact for this canonical denomination set with unlimited
 *   notes; with limited counts we fall back to reporting failure.)
 * Complexity: O(d) per request for d denominations; O(d) space.
 */
type Plan = Map<number, number>;

abstract class NoteHandler {
  private next: NoteHandler | null = null;
  setNext(h: NoteHandler): NoteHandler { this.next = h; return h; }
  plan(amount: number, acc: Plan): number {
    const rest = this.take(amount, acc);
    return this.next && rest > 0 ? this.next.plan(rest, acc) : rest;
  }
  protected abstract take(amount: number, acc: Plan): number;
  abstract commit(acc: Plan): void;
}

class Denomination extends NoteHandler {
  constructor(readonly value: number, private count: number) { super(); }
  protected take(amount: number, acc: Plan): number {
    const n = Math.min(Math.floor(amount / this.value), this.count);
    if (n > 0) acc.set(this.value, n);
    return amount - n * this.value;
  }
  commit(acc: Plan): void { this.count -= acc.get(this.value) ?? 0; }
  get remaining() { return this.count; }
}

class ATM {
  private handlers: Denomination[];
  constructor(inventory: [number, number][]) {
    this.handlers = inventory.sort((a, b) => b[0] - a[0]).map(([v, c]) => new Denomination(v, c));
    this.handlers.reduce<NoteHandler>((a, b) => a.setNext(b));
  }
  withdraw(amount: number): Plan | null {
    const plan: Plan = new Map();
    if (this.handlers[0].plan(amount, plan) !== 0) return null;
    this.handlers.forEach((h) => h.commit(plan));
    return plan;
  }
}

const atm = new ATM([[100, 2], [50, 1], [20, 5], [10, 1]]);
console.log(atm.withdraw(280)); // 100x2, 50x1, 20x1, 10x1
console.log(atm.withdraw(15));  // null
console.log(atm.withdraw(80));  // 20x4
console.log(atm.withdraw(10));  // null (no tens left)
