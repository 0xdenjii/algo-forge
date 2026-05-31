/**
 * Title: Lazy Singleton (Configuration Manager)
 * Problem: Provide exactly one shared ConfigManager instance, created on first use.
 *   Callers must not be able to construct it with `new`.
 * Approach: private constructor + static getInstance() that creates on demand.
 * Complexity: O(1) time per access, O(1) space.
 */
export class ConfigManager {
  private static instance: ConfigManager | undefined;
  private readonly values = new Map<string, string>();
  private constructor() {}
  static getInstance(): ConfigManager {
    return (ConfigManager.instance ??= new ConfigManager());
  }
  set(key: string, value: string): void { this.values.set(key, value); }
  get(key: string): string | undefined { return this.values.get(key); }
}

const a = ConfigManager.getInstance();
a.set("env", "prod");
const b = ConfigManager.getInstance();
console.log(a === b, b.get("env")); // true prod
