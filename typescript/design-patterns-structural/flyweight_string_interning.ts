/**
 * Title: Flyweight - Value Interning (canonical immutable objects)
 * Problem: Many equal immutable values (e.g. Money(currency, amount) or Point(x, y)) are
 *   created repeatedly. Intern them so equal values share one instance, enabling `===`
 *   equality and saving memory; allow unused instances to be garbage-collected.
 * Approach: Intern pool keyed by a canonical string, holding WeakRefs; a
 *   FinalizationRegistry cleans dead keys.
 * Complexity: O(1) average intern; memory O(live distinct values).
 */
class Interner<T extends object> {
  private pool = new Map<string, WeakRef<T>>();
  private registry = new FinalizationRegistry<string>((key) => {
    const ref = this.pool.get(key);
    if (ref && ref.deref() === undefined) this.pool.delete(key);
  });
  intern(key: string, create: () => T): T {
    const existing = this.pool.get(key)?.deref();
    if (existing) return existing;
    const obj = Object.freeze(create());
    this.pool.set(key, new WeakRef(obj));
    this.registry.register(obj, key);
    return obj;
  }
  get size(): number { return this.pool.size; }
}

class Money {
  private static interner = new Interner<Money>();
  private constructor(readonly currency: string, readonly cents: number) {}
  static of(currency: string, cents: number): Money {
    if (!Number.isInteger(cents)) throw new RangeError("cents must be an integer");
    const cur = currency.toUpperCase();
    return Money.interner.intern(`${cur}:${cents}`, () => new Money(cur, cents));
  }
  add(o: Money): Money {
    if (o.currency !== this.currency) throw new Error("currency mismatch");
    return Money.of(this.currency, this.cents + o.cents);
  }
  toString(): string { return `${(this.cents / 100).toFixed(2)} ${this.currency}`; }
  static get poolSize(): number { return Money.interner.size; }
}

const a = Money.of("eur", 500);
const b = Money.of("EUR", 250).add(Money.of("EUR", 250));
console.log(a === b, String(b)); // true "5.00 EUR"
const prices = Array.from({ length: 10_000 }, (_, i) => Money.of("USD", (i % 10) * 100));
console.log(`created ${prices.length} references, pool size ${Money.poolSize}`);
