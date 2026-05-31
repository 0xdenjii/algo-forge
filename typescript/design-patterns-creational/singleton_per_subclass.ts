/**
 * Title: Singleton per Subclass (Inherited instance())
 * Problem: A base Service class should guarantee one instance per subclass
 *   (CacheService, MetricsService) via a single inherited static instance().
 * Approach: static generic method typed on `this` (the constructor); WeakMap keyed by class.
 * Complexity: O(1) per lookup.
 */
const instances = new WeakMap<object, unknown>();
abstract class Service {
  static instance<T extends Service>(this: new () => T): T {
    let inst = instances.get(this) as T | undefined;
    if (!inst) { inst = new this(); instances.set(this, inst); }
    return inst;
  }
  abstract name(): string;
}
class CacheService extends Service { readonly store = new Map<string, string>(); name() { return "cache"; } }
class MetricsService extends Service { hits = 0; name() { return "metrics"; } }

CacheService.instance().store.set("k", "v");
MetricsService.instance().hits++;
console.log(CacheService.instance().store.get("k"), MetricsService.instance().hits,
  (CacheService.instance() as Service) !== (MetricsService.instance() as Service)); // v 1 true
