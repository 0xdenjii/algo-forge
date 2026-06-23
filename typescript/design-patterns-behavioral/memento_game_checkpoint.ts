/**
 * Title: Memento - Game Checkpoints
 * Problem: A game character has position, health and inventory. Players can
 *   save named checkpoints and restore them later. Snapshots must be
 *   immutable and opaque to the caretaker (it cannot modify state).
 * Approach: Originator creates frozen deep copies wrapped in a class with a
 *   private (#) field; only the originator can read them back.
 * Complexity: save/restore O(s) for state size s; O(k * s) for k checkpoints.
 */
interface HeroState { x: number; y: number; hp: number; inventory: string[] }

class Memento {
  #state: Readonly<HeroState>;
  readonly createdAt: number;
  constructor(state: HeroState, at: number) {
    this.#state = Object.freeze({ ...state, inventory: Object.freeze([...state.inventory]) as string[] });
    this.createdAt = at;
  }
  /** Only Hero should call this; kept module-internal by convention. */
  static read(m: Memento): Readonly<HeroState> { return m.#state; }
}

class Hero {
  private s: HeroState = { x: 0, y: 0, hp: 100, inventory: [] };
  private clock = 0;
  move(dx: number, dy: number) { this.s.x += dx; this.s.y += dy; this.clock++; }
  hit(dmg: number) { this.s.hp = Math.max(0, this.s.hp - dmg); this.clock++; }
  pick(item: string) { this.s.inventory.push(item); this.clock++; }
  save(): Memento { return new Memento(this.s, this.clock); }
  restore(m: Memento) { const st = Memento.read(m); this.s = { ...st, inventory: [...st.inventory] }; }
  toString() { return `(${this.s.x},${this.s.y}) hp=${this.s.hp} inv=[${this.s.inventory}]`; }
}

class SaveSlots {
  private slots = new Map<string, Memento>();
  put(name: string, m: Memento) { this.slots.set(name, m); }
  get(name: string): Memento {
    const m = this.slots.get(name);
    if (!m) throw new Error(`no save named ${name}`);
    return m;
  }
}

const hero = new Hero(), saves = new SaveSlots();
hero.move(3, 4); hero.pick("sword");
saves.put("before-boss", hero.save());
hero.hit(80); hero.pick("key");
console.log("now:", String(hero));
hero.restore(saves.get("before-boss"));
console.log("restored:", String(hero));
hero.pick("shield"); // must not leak into the saved snapshot
hero.restore(saves.get("before-boss"));
console.log("restored again:", String(hero));
