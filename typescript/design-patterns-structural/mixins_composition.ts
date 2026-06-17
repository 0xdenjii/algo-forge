/**
 * Title: Mixins - Composing Behaviour Without Deep Inheritance
 * Problem: Several unrelated entities need combinations of capabilities (timestamps,
 *   soft-delete, event emission). Single inheritance cannot express arbitrary mixes.
 * Approach: TypeScript mixin functions `(Base) => class extends Base`, typed through a
 *   generic constructor constraint, composed per entity.
 * Complexity: O(1) per method call; O(listeners) per emitted event.
 */
type Ctor<T = object> = new (...args: any[]) => T;

function Timestamped<B extends Ctor>(Base: B) {
  return class extends Base {
    createdAt = new Date(0);
    updatedAt = new Date(0);
    touch(now = new Date()): void { this.updatedAt = now; }
  };
}

function SoftDeletable<B extends Ctor>(Base: B) {
  return class extends Base {
    deletedAt: Date | null = null;
    softDelete(now = new Date()): void { this.deletedAt = now; }
    restore(): void { this.deletedAt = null; }
    get isDeleted(): boolean { return this.deletedAt !== null; }
  };
}

function Emitter<B extends Ctor>(Base: B) {
  return class extends Base {
    private listeners = new Map<string, ((payload: unknown) => void)[]>();
    on(evt: string, fn: (payload: unknown) => void): void {
      this.listeners.set(evt, [...(this.listeners.get(evt) ?? []), fn]);
    }
    emit(evt: string, payload?: unknown): void { for (const fn of this.listeners.get(evt) ?? []) fn(payload); }
  };
}

class Entity { constructor(public id: string) {} }

class User extends Emitter(SoftDeletable(Timestamped(Entity))) {
  constructor(id: string, public email: string) { super(id); }
  changeEmail(email: string, now: Date): void {
    const old = this.email;
    this.email = email;
    this.touch(now);
    this.emit("emailChanged", { old, email });
  }
}
class Tag extends Timestamped(Entity) {}

const u = new User("u1", "a@example.com");
u.on("emailChanged", (p) => console.log("event:", p));
u.changeEmail("b@example.com", new Date("2026-10-16T00:00:00Z"));
u.softDelete();
console.log(u.isDeleted, u.updatedAt.toISOString());
u.restore();
const t = new Tag("t1");
t.touch(new Date("2026-01-01T00:00:00Z"));
console.log(u.isDeleted, t.updatedAt.toISOString(), "softDelete" in t); // false ... false
