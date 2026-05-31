/**
 * Title: Static Factory Methods (Color value object)
 * Problem: Provide named constructors (Color.fromHex, Color.fromRgb) that validate
 *   input and return cached (interned) instances so equal colors are identical.
 * Approach: private constructor + descriptive static creators (Effective Java item 1).
 * Complexity: O(1).
 */
export class Color {
  private static readonly cache = new Map<string, Color>();
  private constructor(readonly r: number, readonly g: number, readonly b: number) { Object.freeze(this); }
  static fromRgb(r: number, g: number, b: number): Color {
    for (const v of [r, g, b]) if (!Number.isInteger(v) || v < 0 || v > 255) throw new RangeError(`bad channel ${v}`);
    const key = `${r},${g},${b}`;
    let c = Color.cache.get(key);
    if (!c) { c = new Color(r, g, b); Color.cache.set(key, c); }
    return c;
  }
  static fromHex(hex: string): Color {
    const m = /^#?([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})$/i.exec(hex);
    if (!m) throw new Error(`bad hex ${hex}`);
    return Color.fromRgb(parseInt(m[1], 16), parseInt(m[2], 16), parseInt(m[3], 16));
  }
  static readonly BLACK = Color.fromRgb(0, 0, 0);
  toHex(): string { return "#" + [this.r, this.g, this.b].map((v) => v.toString(16).padStart(2, "0")).join(""); }
}

console.log(Color.fromHex("#ff8000").toHex(), Color.fromHex("000000") === Color.BLACK); // #ff8000 true
try { Color.fromRgb(300, 0, 0); } catch (e) { console.log((e as Error).message); }
