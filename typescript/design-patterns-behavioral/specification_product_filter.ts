/**
 * Title: Specification - Composable Product Filter
 * Problem: A catalogue must be filtered by many business rules (category,
 *   price range, in stock, rating) combined with and/or/not, without writing
 *   a new filter method for every combination (open/closed principle).
 * Approach: Spec<T> wraps a predicate with fluent and/or/not combinators.
 * Complexity: O(n * k) for n products and k leaf rules; O(k) space.
 */
class Spec<T> {
  constructor(readonly test: (x: T) => boolean, readonly label = "spec") {}
  and(o: Spec<T>) { return new Spec<T>((x) => this.test(x) && o.test(x), `(${this.label} AND ${o.label})`); }
  or(o: Spec<T>) { return new Spec<T>((x) => this.test(x) || o.test(x), `(${this.label} OR ${o.label})`); }
  not() { return new Spec<T>((x) => !this.test(x), `NOT ${this.label}`); }
  static all<T>(...s: Spec<T>[]) { return s.reduce((a, b) => a.and(b)); }
}

interface Product { name: string; category: string; price: number; stock: number; rating: number }

const category = (c: string) => new Spec<Product>((p) => p.category === c, `category=${c}`);
const priceBetween = (lo: number, hi: number) => new Spec<Product>((p) => p.price >= lo && p.price <= hi, `price ${lo}..${hi}`);
const inStock = new Spec<Product>((p) => p.stock > 0, "inStock");
const ratingAtLeast = (r: number) => new Spec<Product>((p) => p.rating >= r, `rating>=${r}`);

const catalogue: Product[] = [
  { name: "Tent 2P", category: "camping", price: 89, stock: 4, rating: 4.5 },
  { name: "Tent 4P", category: "camping", price: 249, stock: 0, rating: 4.8 },
  { name: "Road bike", category: "cycling", price: 499, stock: 2, rating: 4.2 },
  { name: "Helmet", category: "cycling", price: 39, stock: 10, rating: 3.9 },
];

const spec = Spec.all(inStock, ratingAtLeast(4)).and(category("camping").or(priceBetween(0, 50).not()));
console.log(spec.label);
console.log(catalogue.filter(spec.test).map((p) => p.name)); // Tent 2P, Road bike
