/**
 * Title: Decorator - Coffee Shop (Starbuzz)
 * Problem: Beverages have a base price; condiments (milk, mocha, whip) add cost and change
 *   the description. Any combination, including doubles, must be supported without a
 *   subclass per combination.
 * Approach: Condiment decorators wrap a Beverage and implement the same interface.
 *   Prices are in integer cents to avoid floating-point drift.
 * Complexity: O(k) per cost()/description() for k stacked decorators.
 */
interface Beverage { description(): string; cost(): number }

class Espresso implements Beverage {
  description(): string { return "Espresso"; }
  cost(): number { return 199; }
}
class HouseBlend implements Beverage {
  description(): string { return "House Blend"; }
  cost(): number { return 89; }
}

abstract class CondimentDecorator implements Beverage {
  constructor(protected inner: Beverage, private label: string, private price: number) {}
  description(): string { return `${this.inner.description()}, ${this.label}`; }
  cost(): number { return this.inner.cost() + this.price; }
}
class Milk extends CondimentDecorator { constructor(b: Beverage) { super(b, "Milk", 10); } }
class Mocha extends CondimentDecorator { constructor(b: Beverage) { super(b, "Mocha", 20); } }
class Whip extends CondimentDecorator { constructor(b: Beverage) { super(b, "Whip", 10); } }

const fmt = (b: Beverage): string => `${b.description()} $${(b.cost() / 100).toFixed(2)}`;

console.log(fmt(new Espresso())); // Espresso $1.99
console.log(fmt(new Whip(new Mocha(new Mocha(new HouseBlend()))))); // $1.39
console.log(fmt(new Milk(new Espresso()))); // $2.09
