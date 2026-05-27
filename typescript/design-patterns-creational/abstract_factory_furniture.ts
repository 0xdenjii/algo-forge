/**
 * Title: Abstract Factory (Furniture Shop)
 * Problem: Produce matching Chair, Sofa, CoffeeTable in Victorian or Modern style
 *   and compute total price of a matching set.
 * Approach: abstract FurnitureFactory class; concrete style factories.
 * Complexity: O(1) per item.
 */
interface Furniture { style: string; kind: string; price: number }
abstract class FurnitureFactory {
  abstract readonly style: string;
  protected abstract prices: { chair: number; sofa: number; table: number };
  private make(kind: "chair" | "sofa" | "table"): Furniture {
    return { style: this.style, kind, price: this.prices[kind] };
  }
  createChair() { return this.make("chair"); }
  createSofa() { return this.make("sofa"); }
  createCoffeeTable() { return this.make("table"); }
}
class VictorianFactory extends FurnitureFactory {
  readonly style = "Victorian"; protected prices = { chair: 300, sofa: 1500, table: 600 };
}
class ModernFactory extends FurnitureFactory {
  readonly style = "Modern"; protected prices = { chair: 120, sofa: 900, table: 250 };
}

function livingRoom(f: FurnitureFactory) {
  const set = [f.createChair(), f.createSofa(), f.createCoffeeTable()];
  return { style: f.style, total: set.reduce((s, x) => s + x.price, 0) };
}
console.log(livingRoom(new VictorianFactory()), livingRoom(new ModernFactory()));
