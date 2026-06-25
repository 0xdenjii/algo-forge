/**
 * Title: State - Vending Machine
 * Problem: A vending machine accepts coins, lets the user select a product,
 *   dispenses it and returns change. Behaviour depends on whether it is
 *   idle, has money, or is sold out; invalid actions must be rejected.
 * Approach: Each state is a class implementing the same interface; the
 *   machine delegates to its current state, which performs transitions.
 * Complexity: O(1) per action; O(products) space.
 */
interface VMState {
  insert(m: VendingMachine, cents: number): string;
  select(m: VendingMachine, code: string): string;
  refund(m: VendingMachine): string;
}

class IdleState implements VMState {
  insert(m: VendingMachine, cents: number) { m.balance += cents; m.state = new HasMoneyState(); return `balance ${m.balance}`; }
  select() { return "insert money first"; }
  refund() { return "nothing to refund"; }
}

class HasMoneyState implements VMState {
  insert(m: VendingMachine, cents: number) { m.balance += cents; return `balance ${m.balance}`; }
  select(m: VendingMachine, code: string) {
    const item = m.stock.get(code);
    if (!item || item.qty === 0) return `${code} unavailable`;
    if (m.balance < item.price) return `need ${item.price - m.balance} more`;
    item.qty--;
    const change = m.balance - item.price;
    m.balance = 0;
    m.state = m.totalStock() === 0 ? new SoldOutState() : new IdleState();
    return `dispensed ${item.name}, change ${change}`;
  }
  refund(m: VendingMachine) { const r = m.balance; m.balance = 0; m.state = new IdleState(); return `refunded ${r}`; }
}

class SoldOutState implements VMState {
  insert(_m: VendingMachine, cents: number) { return `sold out, returning ${cents}`; }
  select() { return "sold out"; }
  refund() { return "nothing to refund"; }
}

class VendingMachine {
  state: VMState = new IdleState();
  balance = 0;
  constructor(readonly stock: Map<string, { name: string; price: number; qty: number }>) {
    if (this.totalStock() === 0) this.state = new SoldOutState();
  }
  totalStock(): number { let t = 0; for (const s of this.stock.values()) t += s.qty; return t; }
  insert(c: number) { return this.state.insert(this, c); }
  select(code: string) { return this.state.select(this, code); }
  refund() { return this.state.refund(this); }
}

const vm = new VendingMachine(new Map([
  ["A1", { name: "Water", price: 100, qty: 1 }],
  ["A2", { name: "Chips", price: 150, qty: 0 }],
]));
console.log(vm.select("A1"));   // insert money first
console.log(vm.insert(50));     // balance 50
console.log(vm.select("A1"));   // need 50 more
console.log(vm.insert(100));    // balance 150
console.log(vm.select("A2"));   // A2 unavailable
console.log(vm.select("A1"));   // dispensed Water, change 50
console.log(vm.insert(25));     // sold out, returning 25
