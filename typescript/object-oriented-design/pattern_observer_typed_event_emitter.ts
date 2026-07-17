/**
 * Design Pattern - Observer (type-safe event emitter)
 * Subjects publish named events; observers subscribe and receive strongly typed payloads.
 * Must support unsubscribe, one-shot listeners, and listeners removing themselves during emit.
 * Approach: generic over an event map so on/emit are checked by the compiler; emit iterates a snapshot.
 * Time: on/off O(1) (Set), emit O(listeners). Space: O(listeners).
 */
type Listener<P> = (payload: P) => void;

export class TypedEmitter<Events extends Record<string, unknown>> {
  private readonly listeners: { [K in keyof Events]?: Set<Listener<Events[K]>> } = {};

  on<K extends keyof Events>(event: K, fn: Listener<Events[K]>): () => void {
    (this.listeners[event] ??= new Set()).add(fn);
    return () => this.off(event, fn);
  }

  once<K extends keyof Events>(event: K, fn: Listener<Events[K]>): () => void {
    const off = this.on(event, (p) => {
      off();
      fn(p);
    });
    return off;
  }

  off<K extends keyof Events>(event: K, fn: Listener<Events[K]>): void {
    this.listeners[event]?.delete(fn);
  }

  emit<K extends keyof Events>(event: K, payload: Events[K]): void {
    for (const fn of [...(this.listeners[event] ?? [])]) fn(payload);
  }
}

// Example domain: a stock ticker subject
type TickerEvents = { price: { symbol: string; value: number }; halted: { symbol: string } };

class StockTicker extends TypedEmitter<TickerEvents> {
  update(symbol: string, value: number): void {
    this.emit("price", { symbol, value });
  }
}

const ticker = new StockTicker();
const seen: string[] = [];
const unsub = ticker.on("price", ({ symbol, value }) => seen.push(`${symbol}=${value}`));
ticker.once("price", () => seen.push("first-only"));
ticker.on("halted", ({ symbol }) => seen.push(`halt ${symbol}`));
ticker.update("ACME", 10);
ticker.update("ACME", 11);
unsub();
ticker.update("ACME", 12);
ticker.emit("halted", { symbol: "ACME" });
console.log(seen); // ['ACME=10', 'first-only', 'ACME=11', 'halt ACME']
