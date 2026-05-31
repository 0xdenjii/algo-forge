/**
 * Title: Eager Singleton (Logger)
 * Problem: Expose a single Logger created at module load time; every import
 *   receives the same object with a shared log buffer.
 * Approach: static readonly field initialised eagerly; ES modules are evaluated once.
 * Complexity: O(1) access, O(n) space for n log lines.
 */
export class Logger {
  static readonly INSTANCE = new Logger();
  private readonly lines: string[] = [];
  private constructor() {}
  log(level: "info" | "warn" | "error", msg: string): void {
    this.lines.push(`[${level.toUpperCase()}] ${msg}`);
  }
  dump(): readonly string[] { return this.lines; }
}

Logger.INSTANCE.log("info", "started");
Logger.INSTANCE.log("warn", "low disk");
console.log(Logger.INSTANCE.dump());
