/**
 * Title: Async Singleton (Database Connection)
 * Problem: A connection requires async setup. Concurrent callers of getInstance()
 *   must share one initialisation, never creating two connections.
 * Approach: cache the Promise (not the instance) so concurrent calls await the same work;
 *   on failure the cached promise is cleared so a later call can retry.
 * Complexity: O(1) per call after init, O(1) space.
 */
export class Db {
  private static pending: Promise<Db> | undefined;
  static created = 0;
  private constructor(readonly url: string) { Db.created++; }
  static getInstance(url = "postgres://localhost/app"): Promise<Db> {
    return (Db.pending ??= (async () => {
      await new Promise((r) => setTimeout(r, 10)); // simulate handshake
      return new Db(url);
    })().catch((e) => { Db.pending = undefined; throw e; }));
  }
}

(async () => {
  const [x, y, z] = await Promise.all([Db.getInstance(), Db.getInstance(), Db.getInstance()]);
  console.log(x === y && y === z, "created:", Db.created); // true created: 1
})();
