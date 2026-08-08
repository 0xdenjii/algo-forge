/**
 * LLD - Vending Machine (State pattern)
 * Machine accepts coins, lets the user select a product, dispenses it and returns change.
 * Invalid actions in a given state are rejected rather than corrupting state.
 * Approach: State interface implemented by Idle / HasMoney / Dispensing; context delegates.
 * Time: O(1) per action (change making O(coin types)). Space: O(products).
 */
interface State {
  insert(m: VendingMachine, cents: number): void;
  select(m: VendingMachine, code: string): void;
  cancel(m: VendingMachine): number;
}

const COINS = [5, 10, 25, 100];

class Idle implements State {
  insert(m: VendingMachine, cents: number): void {
    m.credit += cents;
    m.setState(new HasMoney());
  }
  select(): void {
    throw new Error("insert money first");
  }
  cancel(): number {
    return 0;
  }
}

class HasMoney implements State {
  insert(m: VendingMachine, cents: number): void {
    m.credit += cents;
  }
  select(m: VendingMachine, code: string): void {
    const p = m.inventory.get(code);
    if (!p || p.qty === 0) throw new Error(`${code} unavailable`);
    if (m.credit < p.price) throw new Error(`need ${p.price - m.credit} more cents`);
    m.setState(new Dispensing());
    m.dispense(code);
  }
  cancel(m: VendingMachine): number {
    const c = m.credit;
    m.credit = 0;
    m.setState(new Idle());
    return c;
  }
}

class Dispensing implements State {
  insert(): void {
    throw new Error("busy");
  }
  select(): void {
    throw new Error("busy");
  }
  cancel(): number {
    throw new Error("busy");
  }
}

export class VendingMachine {
  credit = 0;
  private state: State = new Idle();
  readonly inventory = new Map<string, { name: string; price: number; qty: number }>();
  lastChange = 0;

  setState(s: State): void {
    this.state = s;
  }
  stock(code: string, name: string, price: number, qty: number): void {
    this.inventory.set(code, { name, price, qty });
  }
  insert(cents: number): void {
    if (!COINS.includes(cents)) throw new Error(`rejected coin ${cents}`);
    this.state.insert(this, cents);
  }
  select(code: string): void {
    this.state.select(this, code);
  }
  cancel(): number {
    return this.state.cancel(this);
  }
  dispense(code: string): void {
    const p = this.inventory.get(code)!;
    p.qty--;
    this.lastChange = this.credit - p.price;
    this.credit = 0;
    console.log(`dispensed ${p.name}, change ${this.lastChange}`);
    this.setState(new Idle());
  }
}

const vm = new VendingMachine();
vm.stock("A1", "Water", 65, 2);
vm.insert(25);
vm.insert(25);
try {
  vm.select("A1");
} catch (e) {
  console.log((e as Error).message); // need 15 more cents
}
vm.insert(25);
vm.select("A1"); // dispensed Water, change 10
console.log(vm.cancel()); // 0
