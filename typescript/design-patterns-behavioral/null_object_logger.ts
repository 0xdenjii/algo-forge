/**
 * Title: Null Object - Optional Logger and Discount Policy
 * Problem: Services accept an optional logger and an optional discount.
 *   Avoid scattering `if (logger)` / `if (discount)` checks by supplying
 *   do-nothing implementations that honour the same interface.
 * Approach: NullLogger and NoDiscount singletons as defaults.
 * Complexity: O(1) overhead per call; O(1) space.
 */
interface Logger { info(msg: string): void; error(msg: string): void }
interface Discount { apply(total: number): number }

const NullLogger: Logger = Object.freeze({ info() {}, error() {} });
const NoDiscount: Discount = Object.freeze({ apply: (t: number) => t });

class MemoryLogger implements Logger {
  readonly lines: string[] = [];
  info(m: string) { this.lines.push(`INFO ${m}`); }
  error(m: string) { this.lines.push(`ERROR ${m}`); }
}
const percentOff = (p: number): Discount => ({ apply: (t) => Math.round(t * (100 - p)) / 100 });

class OrderService {
  constructor(private log: Logger = NullLogger, private discount: Discount = NoDiscount) {}
  total(items: { price: number; qty: number }[]): number {
    if (items.some((i) => i.qty <= 0)) { this.log.error("non-positive quantity"); throw new RangeError("bad qty"); }
    const raw = items.reduce((s, i) => s + i.price * i.qty, 0);
    const final = this.discount.apply(raw);
    this.log.info(`raw=${raw} final=${final}`);
    return final;
  }
}

const items = [{ price: 20, qty: 2 }, { price: 10, qty: 1 }];
console.log(new OrderService().total(items)); // 50, no logging, no discount
const mem = new MemoryLogger();
console.log(new OrderService(mem, percentOff(10)).total(items)); // 45
console.log(mem.lines);
