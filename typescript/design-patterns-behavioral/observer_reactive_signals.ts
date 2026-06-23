/**
 * Title: Observer - Reactive Signals with Computed Values and Effects
 * Problem: Implement minimal signals (as in SolidJS/Preact): signal(v) holds
 *   a value, computed(fn) derives from signals with automatic dependency
 *   tracking and caching, effect(fn) re-runs when dependencies change.
 * Approach: Global "current observer" stack records reads; writes mark
 *   computeds dirty and schedule effects (deduplicated per write).
 * Complexity: write O(transitive dependents); read O(1) when clean.
 */
interface Observer { notify(): void; deps: Set<Set<Observer>> }
const stack: Observer[] = [];

function track(subs: Set<Observer>) {
  const cur = stack[stack.length - 1];
  if (cur) { subs.add(cur); cur.deps.add(subs); }
}
function cleanup(o: Observer) { o.deps.forEach((s) => s.delete(o)); o.deps.clear(); }

function signal<T>(initial: T) {
  let value = initial;
  const subs = new Set<Observer>();
  return {
    get(): T { track(subs); return value; },
    set(v: T) {
      if (Object.is(v, value)) return;
      value = v;
      [...subs].forEach((o) => o.notify());
    },
  };
}

function computed<T>(fn: () => T) {
  let cached: T, dirty = true;
  const subs = new Set<Observer>();
  const self: Observer = {
    deps: new Set(),
    notify() { if (!dirty) { dirty = true; [...subs].forEach((o) => o.notify()); } },
  };
  return {
    get(): T {
      track(subs);
      if (dirty) {
        cleanup(self); stack.push(self);
        try { cached = fn(); } finally { stack.pop(); }
        dirty = false;
      }
      return cached;
    },
  };
}

function effect(fn: () => void): () => void {
  const self: Observer = { deps: new Set(), notify: () => run() };
  const run = () => { cleanup(self); stack.push(self); try { fn(); } finally { stack.pop(); } };
  run();
  return () => cleanup(self);
}

const first = signal("Ada"), last = signal("Lovelace"), showFull = signal(true);
let computeCount = 0;
const full = computed(() => { computeCount++; return `${first.get()} ${last.get()}`; });
const out: string[] = [];
const stop = effect(() => out.push(showFull.get() ? full.get() : first.get()));
first.set("Augusta");
showFull.set(false);        // effect now depends only on first/showFull
last.set("King");           // does not trigger the effect
first.set("Ada");
stop();
first.set("ignored");
console.log(out, "computeCount:", computeCount);
