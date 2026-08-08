/**
 * LLD - Splitwise (expense sharing)
 * Users add expenses split EQUAL, EXACT or PERCENT. Show per-user balances and a minimal-ish
 * list of settlement transfers.
 * Approach: Strategy per split type producing shares in integer cents; net balances map;
 * greedy settlement matching largest creditor with largest debtor (at most n-1 transfers).
 * Time: addExpense O(participants), settle O(n log n) per transfer. Space: O(users).
 */
type Split =
  | { kind: "EQUAL"; users: string[] }
  | { kind: "EXACT"; shares: Record<string, number> }
  | { kind: "PERCENT"; shares: Record<string, number> };

function computeShares(total: number, s: Split): Map<string, number> {
  const out = new Map<string, number>();
  if (s.kind === "EQUAL") {
    const base = Math.floor(total / s.users.length);
    let rem = total - base * s.users.length;
    s.users.forEach(u => out.set(u, base + (rem-- > 0 ? 1 : 0)));
  } else if (s.kind === "EXACT") {
    const sum = Object.values(s.shares).reduce((a, b) => a + b, 0);
    if (sum !== total) throw new Error("exact shares must sum to total");
    Object.entries(s.shares).forEach(([u, v]) => out.set(u, v));
  } else {
    const pct = Object.values(s.shares).reduce((a, b) => a + b, 0);
    if (pct !== 100) throw new Error("percentages must sum to 100");
    let assigned = 0;
    const entries = Object.entries(s.shares);
    entries.forEach(([u, p], i) => {
      const v = i === entries.length - 1 ? total - assigned : Math.round((total * p) / 100);
      assigned += v;
      out.set(u, v);
    });
  }
  return out;
}

export class Splitwise {
  private balance = new Map<string, number>(); // positive = is owed

  addExpense(paidBy: string, totalCents: number, split: Split): void {
    this.balance.set(paidBy, (this.balance.get(paidBy) ?? 0) + totalCents);
    for (const [u, share] of computeShares(totalCents, split)) this.balance.set(u, (this.balance.get(u) ?? 0) - share);
  }

  balances(): Record<string, number> {
    return Object.fromEntries(this.balance);
  }

  settle(): string[] {
    const cred = [...this.balance].filter(([, v]) => v > 0).map(([u, v]) => ({ u, v }));
    const debt = [...this.balance].filter(([, v]) => v < 0).map(([u, v]) => ({ u, v: -v }));
    const out: string[] = [];
    while (cred.length && debt.length) {
      cred.sort((a, b) => b.v - a.v);
      debt.sort((a, b) => b.v - a.v);
      const c = cred[0];
      const d = debt[0];
      const amt = Math.min(c.v, d.v);
      out.push(`${d.u} pays ${c.u} ${(amt / 100).toFixed(2)}`);
      c.v -= amt;
      d.v -= amt;
      if (!c.v) cred.shift();
      if (!d.v) debt.shift();
    }
    return out;
  }
}

const sw = new Splitwise();
sw.addExpense("alice", 3000, { kind: "EQUAL", users: ["alice", "bob", "carol"] });
sw.addExpense("bob", 1000, { kind: "PERCENT", shares: { alice: 50, carol: 50 } });
console.log(sw.balances());
console.log(sw.settle());
