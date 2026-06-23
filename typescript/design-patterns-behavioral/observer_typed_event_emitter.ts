/**
 * Title: Observer - Type-safe Event Emitter
 * Problem: Build an EventEmitter whose event names and payload types are
 *   checked at compile time. Support on, once, off and emit.
 * Approach: Generic over an event map; listeners kept per key in a Map of Sets.
 * Complexity: emit O(listeners of that event); on/off O(1).
 */
type Listener<T> = (payload: T) => void;

class TypedEmitter<Events extends Record<string, unknown>> {
  private listeners = new Map<keyof Events, Set<Listener<never>>>();

  on<K extends keyof Events>(event: K, fn: Listener<Events[K]>): () => void {
    let set = this.listeners.get(event);
    if (!set) this.listeners.set(event, (set = new Set()));
    set.add(fn as Listener<never>);
    return () => this.off(event, fn);
  }

  once<K extends keyof Events>(event: K, fn: Listener<Events[K]>): void {
    const wrapper: Listener<Events[K]> = (p) => { this.off(event, wrapper); fn(p); };
    this.on(event, wrapper);
  }

  off<K extends keyof Events>(event: K, fn: Listener<Events[K]>): void {
    this.listeners.get(event)?.delete(fn as Listener<never>);
  }

  emit<K extends keyof Events>(event: K, payload: Events[K]): number {
    const set = this.listeners.get(event);
    if (!set) return 0;
    const snapshot = [...set] as Listener<Events[K]>[];
    for (const fn of snapshot) fn(payload);
    return snapshot.length;
  }
}

type AppEvents = { login: { user: string }; logout: { user: string; reason: string }; tick: number };
const bus = new TypedEmitter<AppEvents>();
bus.on("login", ({ user }) => console.log(`welcome ${user}`));
bus.once("tick", (n) => console.log(`first tick ${n}`));
bus.emit("login", { user: "ana" });
bus.emit("tick", 1);
console.log("listeners on 2nd tick:", bus.emit("tick", 2)); // 0
// bus.emit("tick", "x"); // compile error
