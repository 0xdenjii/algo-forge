/**
 * LeetCode 1188 - Design Bounded Blocking Queue (async TypeScript adaptation)
 * enqueue waits while the queue is full; dequeue waits while it is empty; size() reports count.
 * Approach: buffer plus FIFO lists of pending resolvers for producers and consumers;
 * handing items/slots directly to waiters preserves fairness.
 * Time: O(1) amortized per op. Space: O(capacity + waiters).
 */
export class BoundedBlockingQueue<T> {
  private items: T[] = [];
  private takers: ((v: T) => void)[] = [];
  private putters: { item: T; resolve: () => void }[] = [];
  constructor(private capacity: number) {}

  enqueue(item: T): Promise<void> {
    const taker = this.takers.shift();
    if (taker) {
      taker(item);
      return Promise.resolve();
    }
    if (this.items.length < this.capacity) {
      this.items.push(item);
      return Promise.resolve();
    }
    return new Promise(resolve => this.putters.push({ item, resolve }));
  }

  dequeue(): Promise<T> {
    if (this.items.length) {
      const v = this.items.shift()!;
      const p = this.putters.shift();
      if (p) {
        this.items.push(p.item);
        p.resolve();
      }
      return Promise.resolve(v);
    }
    return new Promise(resolve => this.takers.push(resolve));
  }

  size(): number {
    return this.items.length;
  }
}

(async () => {
  const q = new BoundedBlockingQueue<number>(2);
  const producer = (async () => {
    for (let i = 1; i <= 5; i++) {
      await q.enqueue(i);
      console.log("produced", i);
    }
  })();
  const got: number[] = [];
  for (let i = 0; i < 5; i++) got.push(await q.dequeue());
  await producer;
  console.log(got, q.size()); // [1,2,3,4,5] 0
})();
