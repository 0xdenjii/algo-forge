/**
 * Title: URL Builder
 * Problem: Construct URLs from scheme, host, port, path segments, query params and
 *   fragment, encoding each component correctly and omitting default ports.
 * Approach: fluent builder; encodeURIComponent per segment; URLSearchParams for query.
 * Complexity: O(len) of output.
 */
export class UrlBuilder {
  private scheme: "http" | "https" = "https";
  private host = "";
  private port?: number;
  private segments: string[] = [];
  private params = new URLSearchParams();
  private frag?: string;
  https(on = true) { this.scheme = on ? "https" : "http"; return this; }
  hostname(h: string) { this.host = h; return this; }
  withPort(p: number) { this.port = p; return this; }
  path(...segs: string[]) { this.segments.push(...segs); return this; }
  query(k: string, v: string | number | boolean) { this.params.append(k, String(v)); return this; }
  fragment(f: string) { this.frag = f; return this; }
  build(): string {
    if (!this.host) throw new Error("host required");
    const def = this.scheme === "https" ? 443 : 80;
    const port = this.port && this.port !== def ? `:${this.port}` : "";
    const path = this.segments.length ? "/" + this.segments.map(encodeURIComponent).join("/") : "";
    const q = this.params.toString();
    return `${this.scheme}://${this.host}${port}${path}${q ? "?" + q : ""}${this.frag ? "#" + encodeURIComponent(this.frag) : ""}`;
  }
}

console.log(new UrlBuilder().hostname("shop.io").withPort(443).path("search", "a b/c")
  .query("q", "tent & stove").query("page", 2).fragment("top").build());
console.log(new UrlBuilder().https(false).hostname("localhost").withPort(3000).build());
