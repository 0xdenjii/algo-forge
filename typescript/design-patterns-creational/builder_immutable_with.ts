/**
 * Title: Immutable "with" Builder (Value Object)
 * Problem: Create variants of an immutable ServerConfig without mutating the
 *   original (copy-on-write, like Java record withers / Lombok @With).
 * Approach: each withX() returns a new frozen instance; defaults supplied once.
 * Complexity: O(f) per copy for f fields.
 */
type Fields = { host: string; port: number; tls: boolean; workers: number };
export class ServerConfig implements Readonly<Fields> {
  readonly host!: string; readonly port!: number; readonly tls!: boolean; readonly workers!: number;
  private constructor(f: Fields) { Object.assign(this, f); Object.freeze(this); }
  static defaults(): ServerConfig { return new ServerConfig({ host: "0.0.0.0", port: 8080, tls: false, workers: 4 }); }
  private copy(p: Partial<Fields>): ServerConfig {
    return new ServerConfig({ host: this.host, port: this.port, tls: this.tls, workers: this.workers, ...p });
  }
  withHost(host: string) { return this.copy({ host }); }
  withPort(port: number) { if (port < 1 || port > 65535) throw new RangeError("port"); return this.copy({ port }); }
  withTls(tls = true) { return this.copy({ tls }); }
  withWorkers(workers: number) { if (workers < 1) throw new RangeError("workers"); return this.copy({ workers }); }
}

const base = ServerConfig.defaults();
const prod = base.withHost("api.example.com").withPort(443).withTls().withWorkers(16);
console.log(base, prod, Object.isFrozen(prod));
