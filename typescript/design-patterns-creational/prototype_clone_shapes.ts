/**
 * Title: Prototype (Cloneable Shapes)
 * Problem: Copy shapes polymorphically without knowing their concrete class;
 *   clones must be independent (deep copy of nested state).
 * Approach: abstract clone() implemented in each subclass via a shared copy helper.
 * Complexity: O(s) per clone where s is object size.
 */
abstract class Shape {
  x = 0; y = 0; color = "black"; tags: string[] = [];
  protected copyInto<T extends Shape>(t: T): T {
    t.x = this.x; t.y = this.y; t.color = this.color; t.tags = [...this.tags];
    return t;
  }
  abstract clone(): Shape;
}
class Circle extends Shape {
  radius = 1;
  clone(): Circle { const c = this.copyInto(new Circle()); c.radius = this.radius; return c; }
}
class Rect extends Shape {
  w = 1; h = 1;
  clone(): Rect { const r = this.copyInto(new Rect()); r.w = this.w; r.h = this.h; return r; }
}

const c = new Circle(); c.radius = 5; c.tags.push("big");
const shapes: Shape[] = [c, new Rect()];
const copies = shapes.map((s) => s.clone());
copies[0].tags.push("copy");
console.log(c.tags, copies[0].tags, copies[0] instanceof Circle, (copies[0] as Circle).radius); // ['big'] ['big','copy'] true 5
