/**
 * LeetCode 535 - Encode and Decode TinyURL
 * Design a URL shortener: encode(longUrl) -> short URL and decode(shortUrl) -> original URL.
 * Approach: auto-increment id rendered in base62; bidirectional maps make encode idempotent.
 * Time: encode O(log id), decode O(1). Space: O(n).
 */
const ALPHA = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

export class Codec {
  private id = 1;
  private byCode = new Map<string, string>();
  private byUrl = new Map<string, string>();

  private toBase62(n: number): string {
    let s = "";
    while (n > 0) {
      s = ALPHA[n % 62] + s;
      n = Math.floor(n / 62);
    }
    return s;
  }

  encode(url: string): string {
    let code = this.byUrl.get(url);
    if (!code) {
      code = this.toBase62(this.id++);
      this.byUrl.set(url, code);
      this.byCode.set(code, url);
    }
    return "http://tiny.url/" + code;
  }

  decode(short: string): string {
    return this.byCode.get(short.substring(short.lastIndexOf("/") + 1)) ?? "";
  }
}

const cd = new Codec();
const short = cd.encode("https://leetcode.com/problems/design-tinyurl");
console.log(short, cd.decode(short));
