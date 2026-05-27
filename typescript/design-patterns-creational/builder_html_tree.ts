/**
 * Title: Composite Builder (HTML Tree)
 * Problem: Build nested HTML elements fluently (open child, add text, go up)
 *   and render them with proper escaping.
 * Approach: builder keeps a stack of open nodes; child() pushes, up() pops.
 * Complexity: O(n) render for n nodes.
 */
interface HNode { tag: string; attrs: Record<string, string>; children: (HNode | string)[] }
const ESC: Record<string, string> = { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" };
const esc = (s: string) => s.replace(/[&<>"]/g, (c) => ESC[c]);

export class HtmlBuilder {
  private readonly root: HNode;
  private readonly stack: HNode[];
  constructor(tag: string) { this.root = { tag, attrs: {}, children: [] }; this.stack = [this.root]; }
  private get cur(): HNode { return this.stack[this.stack.length - 1]; }
  child(tag: string, attrs: Record<string, string> = {}) {
    const n: HNode = { tag, attrs, children: [] };
    this.cur.children.push(n); this.stack.push(n); return this;
  }
  text(t: string) { this.cur.children.push(t); return this; }
  up() { if (this.stack.length === 1) throw new Error("already at root"); this.stack.pop(); return this; }
  build(): string {
    const r = (n: HNode | string): string => typeof n === "string" ? esc(n)
      : `<${n.tag}${Object.entries(n.attrs).map(([k, v]) => ` ${k}="${esc(v)}"`).join("")}>${n.children.map(r).join("")}</${n.tag}>`;
    return r(this.root);
  }
}

console.log(new HtmlBuilder("ul").child("li", { class: "a" }).text("one").up().child("li").text("<two>").up().build());
