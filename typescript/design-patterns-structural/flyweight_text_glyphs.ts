/**
 * Title: Flyweight - Text Editor Glyphs
 * Problem: A document with millions of characters cannot afford one heavy object (font,
 *   size, style, glyph metrics) per character. Share intrinsic state, keep only extrinsic
 *   state (position) per character.
 * Approach: GlyphFactory caches immutable Glyph flyweights keyed by (char, font, size, bold);
 *   the document stores compact {glyph, x} records.
 * Complexity: O(1) average factory lookup; memory O(distinct glyphs + n small refs).
 */
class Glyph {
  readonly width: number;
  constructor(readonly char: string, readonly font: string, readonly size: number, readonly bold: boolean) {
    // Pretend this is an expensive metrics computation.
    this.width = Math.ceil(size * (char === " " ? 0.3 : 0.6) * (bold ? 1.1 : 1));
    Object.freeze(this);
  }
  draw(x: number, y: number): string { return `${this.char}@${x},${y}`; }
}

class GlyphFactory {
  private pool = new Map<string, Glyph>();
  get(char: string, font: string, size: number, bold = false): Glyph {
    const key = `${char}|${font}|${size}|${bold ? 1 : 0}`;
    let g = this.pool.get(key);
    if (!g) { g = new Glyph(char, font, size, bold); this.pool.set(key, g); }
    return g;
  }
  get count(): number { return this.pool.size; }
}

class Document {
  private chars: { glyph: Glyph; x: number; y: number }[] = [];
  private cursorX = 0;
  constructor(private factory: GlyphFactory) {}
  type(text: string, font = "Inter", size = 12, bold = false): void {
    for (const ch of text) {
      const glyph = this.factory.get(ch, font, size, bold);
      this.chars.push({ glyph, x: this.cursorX, y: 0 });
      this.cursorX += glyph.width;
    }
  }
  get length(): number { return this.chars.length; }
  render(n: number): string { return this.chars.slice(0, n).map((c) => c.glyph.draw(c.x, c.y)).join(" "); }
}

const factory = new GlyphFactory();
const doc = new Document(factory);
for (let i = 0; i < 1000; i++) doc.type("hello world ");
doc.type("BOLD", "Inter", 12, true);
console.log(`chars=${doc.length}, flyweights=${factory.count}`); // chars=12004, flyweights=12
console.log(doc.render(5));
