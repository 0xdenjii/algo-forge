/**
 * LLD - Vending Machine (State pattern)
 * Machine accepts coins, lets the user select a product, dispenses it and returns change;
 * invalid actions in a given state are rejected. Sold-out products and refunds must be handled.
 * Approach: each state (Idle, HasMoney) is an object implementing the same interface and
 * transitions the machine; inventory and balance live in the context object.
 * Time: O(1) per action. Space: O(products).
 */
interface State {
  insert(m: VendingMachine, cents: number): void;
  select(m: VendingMachine, code: string): string;
  refund(m: VendingMachine): number;
}

const Idle: State = {
  insert(m, cents) {
    m.balance += cents;
    m.state = HasMoney;
  },
  select() {
    return "insert money first";
  },
  refund() {
    return 0;
  },
};

const HasMoney: State = {
  insert(m, cents) {
    m.balance += cents;
  },
  select(m, code) {
    const item = m.inventory.get(code);
    if (!item) return `unknown product ${code}`;
    if (item.qty === 0) return `${item.name} sold out`;
    if (m.balance < item.price) return `need ${item.price - m.balance} more`;
    item.qty--;
    const change = m.balance - item.price;
    m.balance = 0;
    m.state = Idle;
    return `dispensed ${item.name}, change ${change}`;
  },
  refund(m) {
    const r = m.balance;
    m.balance = 0;
    m.state = Idle;
    return r;
  },
};

export class VendingMachine {
  state: State = Idle;
  balance = 0;
  readonly inventory = new Map<string, { name: string; price: number; qty: number }>();

  stock(code: string, name: string, price: number, qty: number): void {
    this.inventory.set(code, { name, price, qty });
  }
  insert(cents: number): void {
    if (![5, 10, 25, 100].includes(cents)) throw new Error(`rejected coin ${cents}`);
    this.state.insert(this, cents);
  }
  select(code: string): string {
    return this.state.select(this, code);
  }
  refund(): number {
    return this.state.refund(this);
  }
}

const vm = new VendingMachine();
vm.stock("A1", "Water", 75, 1);
console.log(vm.select("A1")); // insert money first
vm.insert(25);
console.log(vm.select("A1")); // need 50 more
vm.insert(100);
console.log(vm.select("A1")); // dispensed Water, change 50
vm.insert(100);
console.log(vm.select("A1"), vm.refund()); // Water sold out 100
