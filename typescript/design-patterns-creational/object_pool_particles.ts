/**
 * Title: Object Pool (Game Particles, GC-friendly)
 * Problem: A particle system spawns thousands of short-lived particles per second;
 *   avoid allocation churn by recycling dead particles.
 * Approach: preallocate a fixed array; free-list of indices; reset on reuse.
 * Complexity: O(1) spawn/kill, O(n) update, O(capacity) space.
 */
class Particle { x = 0; y = 0; vx = 0; vy = 0; life = 0; alive = false; }
export class ParticlePool {
  private readonly items: Particle[];
  private readonly free: number[];
  constructor(capacity: number) {
    this.items = Array.from({ length: capacity }, () => new Particle());
    this.free = Array.from({ length: capacity }, (_, i) => capacity - 1 - i);
  }
  spawn(x: number, y: number, vx: number, vy: number, life: number): boolean {
    const i = this.free.pop();
    if (i === undefined) return false;
    Object.assign(this.items[i], { x, y, vx, vy, life, alive: true });
    return true;
  }
  update(dt: number): void {
    this.items.forEach((p, i) => {
      if (!p.alive) return;
      p.x += p.vx * dt; p.y += p.vy * dt; p.life -= dt;
      if (p.life <= 0) { p.alive = false; this.free.push(i); }
    });
  }
  get active(): number { return this.items.length - this.free.length; }
}

const pool = new ParticlePool(3);
console.log([1, 2, 3, 4].map((i) => pool.spawn(0, 0, 1, 1, i * 0.5))); // [true,true,true,false]
pool.update(0.6); console.log("active", pool.active); // 2
console.log(pool.spawn(0, 0, 0, 0, 1)); // true (recycled slot)
