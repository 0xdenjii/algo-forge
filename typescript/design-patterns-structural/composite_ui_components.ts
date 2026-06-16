/**
 * Title: Composite - UI Component Tree with Layout
 * Problem: A UI has leaf widgets (Label, Button) and containers (VBox, HBox). Every
 *   component reports its size; containers lay children out vertically/horizontally and
 *   render to text.
 * Approach: Component interface with measure()/render(); containers aggregate children.
 * Complexity: O(total cells) to render, O(n) to measure.
 */
interface Size { w: number; h: number }
interface Component { measure(): Size; render(): string[] }

class Label implements Component {
  constructor(private text: string) {}
  measure(): Size { return { w: this.text.length, h: 1 }; }
  render(): string[] { return [this.text]; }
}

class Button implements Component {
  constructor(private text: string) {}
  measure(): Size { return { w: this.text.length + 4, h: 3 }; }
  render(): string[] {
    const bar = "+" + "-".repeat(this.text.length + 2) + "+";
    return [bar, `| ${this.text} |`, bar];
  }
}

abstract class Container implements Component {
  protected children: Component[] = [];
  add(...c: Component[]): this { this.children.push(...c); return this; }
  abstract measure(): Size;
  abstract render(): string[];
}

class VBox extends Container {
  measure(): Size {
    return this.children.reduce<Size>((s, c) => {
      const m = c.measure();
      return { w: Math.max(s.w, m.w), h: s.h + m.h };
    }, { w: 0, h: 0 });
  }
  render(): string[] {
    const w = this.measure().w;
    return this.children.flatMap((c) => c.render().map((l) => l.padEnd(w)));
  }
}

class HBox extends Container {
  constructor(private gap = 1) { super(); }
  measure(): Size {
    const ms = this.children.map((c) => c.measure());
    return {
      w: ms.reduce((s, m) => s + m.w, 0) + this.gap * Math.max(0, ms.length - 1),
      h: Math.max(0, ...ms.map((m) => m.h)),
    };
  }
  render(): string[] {
    const h = this.measure().h;
    const cols = this.children.map((c) => {
      const w = c.measure().w;
      const lines = c.render();
      return Array.from({ length: h }, (_, i) => (lines[i] ?? "").padEnd(w));
    });
    return Array.from({ length: h }, (_, i) => cols.map((col) => col[i]).join(" ".repeat(this.gap)));
  }
}

const ui = new VBox().add(new Label("Confirm deletion?"), new HBox(2).add(new Button("Yes"), new Button("Cancel")));
console.log(ui.measure()); // { w: 21, h: 4 }
console.log(ui.render().join("\n"));
