/**
 * Title: Object Pool (Async Connection Pool)
 * Problem: Reuse a bounded number of expensive connections; acquire() waits when
 *   all are busy, release() hands the connection to the next waiter (FIFO).
 * Approach: idle stack + waiter queue of resolvers; lazily create up to max.
 * Complexity: O(1) acquire/release amortised; O(max + waiters) space.
 */
interface Conn { id: number }
export class Pool {
  private idle: Conn[] = [];
  private waiters: ((c: Conn) => void)[] = [];
  private created = 0;
  constructor(private readonly max: number) {}
  acquire(): Promise<Conn> {
    const c = this.idle.pop();
    if (c) return Promise.resolve(c);
    if (this.created < this.max) return Promise.resolve({ id: ++this.created });
    return new Promise((res) => this.waiters.push(res));
  }
  release(c: Conn): void {
    const w = this.waiters.shift();
    if (w) w(c); else this.idle.push(c);
  }
  async use<T>(fn: (c: Conn) => Promise<T>): Promise<T> {
    const c = await this.acquire();
    try { return await fn(c); } finally { this.release(c); }
  }
}

(async () => {
  const pool = new Pool(2);
  const used = await Promise.all([1, 2, 3, 4, 5].map((i) =>
    pool.use(async (c) => { await new Promise((r) => setTimeout(r, 5)); return `job${i}@conn${c.id}`; })));
  console.log(used); // only conn1 / conn2 ever appear
})();
