/**
 * LLD - Typed Event Bus (Observer / Pub-Sub)
 * Components subscribe to named events with typed payloads; publishers emit without knowing
 * subscribers. Support unsubscribe, once-handlers and isolation of handler errors.
 * Approach: Map<event, Set<handler>> with a generic event map for compile-time payload typing.
 * Time: emit O(subscribers). Space: O(subscriptions).
 */
type Handler<T> = (payload: T) => void;

export class EventBus<Events extends Record<string, unknown>> {
  private handlers = new Map<keyof Events, Set<Handler<any>>>();

  on<K extends keyof Events>(event: K, h: Handler<Events[K]>): () => void {
    if (!this.handlers.has(event)) this.handlers.set(event, new Set());
    this.handlers.get(event)!.add(h);
    return () => this.handlers.get(event)?.delete(h);
  }

  once<K extends keyof Events>(event: K, h: Handler<Events[K]>): () => void {
    const off = this.on(event, p => {
      off();
      h(p);
    });
    return off;
  }

  emit<K extends keyof Events>(event: K, payload: Events[K]): Error[] {
    const errors: Error[] = [];
    for (const h of [...(this.handlers.get(event) ?? [])]) {
      try {
        h(payload);
      } catch (e) {
        errors.push(e as Error);
      }
    }
    return errors;
  }
}

type ShopEvents = { orderPlaced: { id: number; total: number }; userLeft: string };
const bus = new EventBus<ShopEvents>();
const off = bus.on("orderPlaced", o => console.log("email for order", o.id));
bus.once("orderPlaced", o => console.log("first order total", o.total));
bus.on("orderPlaced", () => {
  throw new Error("analytics down");
});
console.log(bus.emit("orderPlaced", { id: 1, total: 42 }).length); // 1 error isolated
off();
bus.emit("orderPlaced", { id: 2, total: 7 }); // only the failing handler remains
