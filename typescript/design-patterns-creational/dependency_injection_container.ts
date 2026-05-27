/**
 * Title: Minimal Dependency Injection Container
 * Problem: Register services by typed token with singleton/transient lifetimes,
 *   resolve dependency graphs automatically, and detect circular dependencies.
 * Approach: Map token -> {factory, lifetime}; resolution stack for cycle detection.
 * Complexity: O(V + E) to resolve a graph first time; O(1) for cached singletons.
 */
export class Token<T> { declare readonly _t: T; constructor(readonly name: string) {} }
type Lifetime = "singleton" | "transient";
export class Container {
  private regs = new Map<Token<unknown>, { f: (c: Container) => unknown; life: Lifetime }>();
  private cache = new Map<Token<unknown>, unknown>();
  private resolving: Token<unknown>[] = [];
  register<T>(t: Token<T>, f: (c: Container) => T, life: Lifetime = "singleton"): this {
    this.regs.set(t, { f, life }); return this;
  }
  resolve<T>(t: Token<T>): T {
    if (this.cache.has(t)) return this.cache.get(t) as T;
    const r = this.regs.get(t);
    if (!r) throw new Error(`No provider for ${t.name}`);
    if (this.resolving.includes(t)) throw new Error(`Cycle: ${[...this.resolving, t].map((x) => x.name).join(" -> ")}`);
    this.resolving.push(t);
    try {
      const v = r.f(this) as T;
      if (r.life === "singleton") this.cache.set(t, v);
      return v;
    } finally { this.resolving.pop(); }
  }
}

class Repo { find(id: number) { return { id, name: "Ada" }; } }
class UserService { constructor(private repo: Repo) {} get(id: number) { return this.repo.find(id); } }
const REPO = new Token<Repo>("Repo"), USERS = new Token<UserService>("UserService");
const c = new Container()
  .register(REPO, () => new Repo())
  .register(USERS, (c) => new UserService(c.resolve(REPO)), "transient");
console.log(c.resolve(USERS).get(1), c.resolve(USERS) !== c.resolve(USERS), c.resolve(REPO) === c.resolve(REPO));
const A = new Token<unknown>("A"), B = new Token<unknown>("B");
c.register(A, (c) => c.resolve(B)).register(B, (c) => c.resolve(A));
try { c.resolve(A); } catch (e) { console.log((e as Error).message); } // Cycle: A -> B -> A
