/**
 * Title: Bridge - Shapes x Renderers
 * Problem: Shapes (Circle, Rectangle) must be drawable by several renderers (SVG, ASCII)
 *   without a class explosion (CircleSvg, CircleAscii, RectSvg, ...).
 * Approach: The Shape abstraction holds a Renderer implementor; both hierarchies vary
 *   independently (M + N classes instead of M * N).
 * Complexity: O(1) per SVG draw; ASCII is O(w * h) for its grid.
 */
interface Renderer {
  circle(cx: number, cy: number, r: number): string;
  rect(x: number, y: number, w: number, h: number): string;
}

class SvgRenderer implements Renderer {
  circle(cx: number, cy: number, r: number): string { return `<circle cx="${cx}" cy="${cy}" r="${r}"/>`; }
  rect(x: number, y: number, w: number, h: number): string { return `<rect x="${x}" y="${y}" width="${w}" height="${h}"/>`; }
}

class AsciiRenderer implements Renderer {
  circle(_cx: number, _cy: number, r: number): string {
    const rows: string[] = [];
    for (let y = -r; y <= r; y++) {
      let row = "";
      for (let x = -r; x <= r; x++) row += Math.abs(Math.hypot(x, y) - r) < 0.5 ? "*" : " ";
      rows.push(row);
    }
    return rows.join("\n");
  }
  rect(_x: number, _y: number, w: number, h: number): string {
    return Array.from({ length: h }, (_, i) =>
      i === 0 || i === h - 1 ? "#".repeat(w) : "#" + " ".repeat(w - 2) + "#",
    ).join("\n");
  }
}

abstract class Shape {
  constructor(protected renderer: Renderer) {}
  abstract draw(): string;
}
class Circle extends Shape {
  constructor(r: Renderer, private cx: number, private cy: number, private radius: number) { super(r); }
  draw(): string { return this.renderer.circle(this.cx, this.cy, this.radius); }
}
class Rectangle extends Shape {
  constructor(r: Renderer, private x: number, private y: number, private w: number, private h: number) { super(r); }
  draw(): string { return this.renderer.rect(this.x, this.y, this.w, this.h); }
}

for (const r of [new SvgRenderer(), new AsciiRenderer()]) {
  console.log(new Circle(r, 5, 5, 3).draw());
  console.log(new Rectangle(r, 0, 0, 6, 3).draw());
}
