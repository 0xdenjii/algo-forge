/**
 * Title: Builder with Director (Pizza)
 * Problem: Construct pizzas step by step; a Director knows standard recipes
 *   (Margherita, Pepperoni) and drives the builder through the steps.
 * Approach: concrete builder that resets after build() + Director encapsulating recipes.
 * Complexity: O(t) for t toppings.
 */
interface Pizza { size: "S" | "M" | "L"; crust: string; sauce: string; toppings: string[] }
const fresh = (): Pizza => ({ size: "M", crust: "regular", sauce: "tomato", toppings: [] });
class PizzaBuilder {
  private p = fresh();
  size(s: Pizza["size"]) { this.p.size = s; return this; }
  crust(c: string) { this.p.crust = c; return this; }
  sauce(s: string) { this.p.sauce = s; return this; }
  topping(t: string) { this.p.toppings.push(t); return this; }
  build(): Pizza { const out = this.p; this.p = fresh(); return out; }
}
class PizzaDirector {
  constructor(private readonly b: PizzaBuilder) {}
  margherita(size: Pizza["size"]) { return this.b.size(size).crust("thin").topping("mozzarella").topping("basil").build(); }
  pepperoni(size: Pizza["size"]) { return this.b.size(size).topping("mozzarella").topping("pepperoni").build(); }
  whiteVeggie(size: Pizza["size"]) { return this.b.size(size).sauce("cream").topping("spinach").topping("mushroom").build(); }
}

const d = new PizzaDirector(new PizzaBuilder());
console.log(d.margherita("L"), d.pepperoni("S"), d.whiteVeggie("M"));
