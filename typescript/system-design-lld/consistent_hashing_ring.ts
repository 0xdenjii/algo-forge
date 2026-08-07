/**
 * LLD - Consistent Hashing Ring with virtual nodes
 * Map keys to servers so that adding/removing a server only remaps ~1/N of the keys.
 * Approach: each server gets V virtual points (FNV-1a hash) on a sorted ring;
 * a key goes to the first point clockwise, found with binary search.
 * Time: lookup O(log(N*V)), add/remove O(N*V log(N*V)). Space: O(N*V).
 */
function fnv1a(s: string): number {
  let h = 0x811c9dc5;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 0x01000193);
  }
  return h >>> 0;
}

export class HashRing {
  private ring: { h: number; node: string }[] = [];
  constructor(private vnodes = 100) {}

  add(node: string): void {
    for (let i = 0; i < this.vnodes; i++) this.ring.push({ h: fnv1a(`${node}#${i}`), node });
    this.ring.sort((a, b) => a.h - b.h);
  }

  remove(node: string): void {
    this.ring = this.ring.filter(p => p.node !== node);
  }

  get(key: string): string {
    if (!this.ring.length) throw new Error("empty ring");
    const h = fnv1a(key);
    let lo = 0;
    let hi = this.ring.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (this.ring[mid].h < h) lo = mid + 1;
      else hi = mid;
    }
    return this.ring[lo % this.ring.length].node;
  }
}

const ring = new HashRing();
["s1", "s2", "s3"].forEach(s => ring.add(s));
const keys = Array.from({ length: 1000 }, (_, i) => `user:${i}`);
const before = keys.map(k => ring.get(k));
ring.add("s4");
const moved = keys.filter((k, i) => ring.get(k) !== before[i]).length;
console.log(`moved ${moved}/1000 keys (~250 expected)`);
