/**
 * Title: Prototype Registry (Game Enemy Spawner)
 * Problem: Spawn many enemies from preconfigured templates ("goblin", "orc");
 *   modifying a spawned enemy must not affect the template.
 * Approach: registry of plain-data prototypes; spawn() returns structuredClone of the template.
 * Complexity: O(s) per spawn.
 */
interface Enemy { name: string; hp: number; stats: { atk: number; def: number }; loot: string[] }
export class EnemyRegistry {
  private readonly protos = new Map<string, Enemy>();
  add(key: string, e: Enemy): this { this.protos.set(key, structuredClone(e)); return this; }
  spawn(key: string, overrides: Partial<Enemy> = {}): Enemy {
    const p = this.protos.get(key);
    if (!p) throw new Error(`unknown prototype ${key}`);
    return { ...structuredClone(p), ...structuredClone(overrides) };
  }
}

const reg = new EnemyRegistry()
  .add("goblin", { name: "Goblin", hp: 30, stats: { atk: 5, def: 2 }, loot: ["coin"] })
  .add("orc", { name: "Orc", hp: 80, stats: { atk: 12, def: 6 }, loot: ["axe"] });
const g1 = reg.spawn("goblin"); g1.stats.atk = 99;
console.log(g1.stats.atk, reg.spawn("goblin").stats.atk, reg.spawn("orc", { name: "Orc Chief" }).name); // 99 5 Orc Chief
