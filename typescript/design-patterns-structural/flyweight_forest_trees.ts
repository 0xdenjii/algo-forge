/**
 * Title: Flyweight - Forest Rendering
 * Problem: A game renders a forest of 100k trees. Each tree has a species with heavy shared
 *   data (texture, mesh) plus per-tree position/scale. Minimize memory.
 * Approach: TreeType flyweights (shared) from a factory; Forest stores per-tree extrinsic
 *   state in parallel typed arrays (struct-of-arrays) for compactness.
 * Complexity: plant O(1) amortized; memory O(types + n * 13 bytes) instead of O(n * texture).
 */
class TreeType {
  readonly texture: Uint8Array;
  constructor(readonly name: string, readonly color: string, textureBytes: number) {
    this.texture = new Uint8Array(textureBytes); // stands in for a heavy shared asset
  }
  describe(x: number, y: number, scale: number): string { return `${this.name}(${this.color}) at ${x},${y} x${scale.toFixed(2)}`; }
}

class TreeFactory {
  private static types = new Map<string, TreeType>();
  static get(name: string, color: string): TreeType {
    const key = `${name}:${color}`;
    let t = this.types.get(key);
    if (!t) { t = new TreeType(name, color, 64 * 1024); this.types.set(key, t); }
    return t;
  }
  static get size(): number { return this.types.size; }
}

class Forest {
  private xs: Float32Array;
  private ys: Float32Array;
  private scales: Float32Array;
  private typeIdx: Uint8Array;
  private types: TreeType[] = [];
  private n = 0;
  constructor(capacity: number) {
    this.xs = new Float32Array(capacity);
    this.ys = new Float32Array(capacity);
    this.scales = new Float32Array(capacity);
    this.typeIdx = new Uint8Array(capacity);
  }
  plant(x: number, y: number, scale: number, name: string, color: string): void {
    if (this.n >= this.xs.length) throw new Error("forest full");
    const t = TreeFactory.get(name, color);
    let idx = this.types.indexOf(t);
    if (idx < 0) idx = this.types.push(t) - 1;
    this.xs[this.n] = x; this.ys[this.n] = y; this.scales[this.n] = scale; this.typeIdx[this.n] = idx;
    this.n++;
  }
  describe(i: number): string { return this.types[this.typeIdx[i]].describe(this.xs[i], this.ys[i], this.scales[i]); }
  approxBytes(): number {
    return this.n * 13 + this.types.reduce((s, t) => s + t.texture.byteLength, 0);
  }
}

const species: [string, string][] = [["oak", "green"], ["pine", "dark-green"], ["birch", "white"]];
const N = 100_000;
const forest = new Forest(N);
let seed = 7;
const rnd = () => ((seed = (seed * 1103515245 + 12345) % 2 ** 31) / 2 ** 31);
for (let i = 0; i < N; i++) {
  const [name, color] = species[i % species.length];
  forest.plant(Math.floor(rnd() * 1000), Math.floor(rnd() * 1000), 0.5 + rnd(), name, color);
}
console.log(forest.describe(0));
console.log(`types=${TreeFactory.size}, ~${(forest.approxBytes() / 1024).toFixed(0)} KiB vs naive ~${((N * 64 * 1024) / 2 ** 30).toFixed(1)} GiB`);
