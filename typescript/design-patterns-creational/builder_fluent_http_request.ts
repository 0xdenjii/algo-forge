/**
 * Title: Fluent Builder (HTTP Request)
 * Problem: Build an immutable HttpRequest with optional headers, query params,
 *   body and timeout; validate (e.g. GET cannot have a body) at build().
 * Approach: mutable builder with chainable setters; build() validates and freezes.
 * Complexity: O(h + q) for h headers, q params.
 */
type Method = "GET" | "POST" | "PUT" | "DELETE";
export interface HttpRequest {
  readonly method: Method; readonly url: string;
  readonly headers: Readonly<Record<string, string>>; readonly body?: string; readonly timeoutMs: number;
}
export class RequestBuilder {
  private method: Method = "GET";
  private headers: Record<string, string> = {};
  private query = new URLSearchParams();
  private body?: string;
  private timeoutMs = 30_000;
  constructor(private readonly baseUrl: string) {}
  withMethod(m: Method) { this.method = m; return this; }
  header(k: string, v: string) { this.headers[k.toLowerCase()] = v; return this; }
  param(k: string, v: string | number) { this.query.append(k, String(v)); return this; }
  json(data: unknown) { this.body = JSON.stringify(data); return this.header("content-type", "application/json"); }
  timeout(ms: number) { if (ms <= 0) throw new RangeError("timeout must be > 0"); this.timeoutMs = ms; return this; }
  build(): HttpRequest {
    if (this.body !== undefined && this.method === "GET") throw new Error("GET cannot have a body");
    const qs = this.query.toString();
    return Object.freeze({
      method: this.method, url: qs ? `${this.baseUrl}?${qs}` : this.baseUrl,
      headers: Object.freeze({ ...this.headers }), body: this.body, timeoutMs: this.timeoutMs,
    });
  }
}

console.log(new RequestBuilder("https://api.x.com/items").withMethod("POST").param("v", 2).json({ a: 1 }).timeout(5000).build());
try { new RequestBuilder("https://x").json({}).build(); } catch (e) { console.log((e as Error).message); }
